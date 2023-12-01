use std::{fs::File, io::Read, time::SystemTime};

use burn::{
    module::Module,
    nn::{self, BatchNorm, PaddingConfig2d},
    record::{BinBytesRecorder, FullPrecisionSettings, Recorder},
    tensor::{backend::Backend, Tensor},
};
use image::ImageBuffer;

const LABELS: usize = 10;

type NdArrayBackend = burn::backend::ndarray::NdArray;

#[derive(Module, Debug)]
pub struct ConvBlock<B: Backend>
where
    B: Backend,
{
    conv: nn::conv::Conv2d<B>,
    norm: BatchNorm<B, 2>,
    activation: nn::GELU,
}

impl<B: Backend> ConvBlock<B> {
    fn new(channels: [usize; 2], kernel_size: [usize; 2]) -> Self {
        let conv = nn::conv::Conv2dConfig::new(channels, kernel_size)
            .with_padding(PaddingConfig2d::Valid)
            .init();
        let norm = nn::BatchNormConfig::new(channels[1]).init();

        Self {
            conv,
            norm,
            activation: nn::GELU::new(),
        }
    }

    fn forward(&self, input: Tensor<B, 4>) -> Tensor<B, 4> {
        let x = self.conv.forward(input);
        let x = self.norm.forward(x);

        self.activation.forward(x)
    }
}

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    conv1: ConvBlock<B>,
    conv2: ConvBlock<B>,
    conv3: ConvBlock<B>,
    dropout: nn::Dropout,
    fc1: nn::Linear<B>,
    fc2: nn::Linear<B>,
    activation: nn::GELU,
}

impl<B: Backend> Model<B> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let conv1 = ConvBlock::new([1, 8], [3, 3]); // out: [Batch,8,26,26]
        let conv2 = ConvBlock::new([8, 16], [3, 3]); // out: [Batch,16,24x24]
        let conv3 = ConvBlock::new([16, 24], [3, 3]); // out: [Batch,24,22x22]
        let hidden_size = 24 * 22 * 22;
        let fc1 = nn::LinearConfig::new(hidden_size, 32)
            .with_bias(false)
            .init();
        let fc2 = nn::LinearConfig::new(32, LABELS).with_bias(false).init();

        let dropout = nn::DropoutConfig::new(0.5).init();

        Self {
            conv1,
            conv2,
            conv3,
            fc1,
            fc2,
            dropout,
            activation: nn::GELU::new(),
        }
    }

    pub fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 2> {
        let [batch_size, height, width] = input.dims();

        let x = input.reshape([batch_size, 1, height, width]).detach();
        let x = self.conv1.forward(x);
        let x = self.conv2.forward(x);
        let x = self.conv3.forward(x);

        let [batch_size, channels, height, width] = x.dims();
        let x = x.reshape([batch_size, channels * height * width]);

        let x = self.dropout.forward(x);
        let x = self.fc1.forward(x);
        let x = self.activation.forward(x);

        self.fc2.forward(x)
    }
}

static STATE_ENCODED: &[u8] = include_bytes!("../model.bin");

fn build_and_load_model() -> anyhow::Result<Model<NdArrayBackend>> {
    let mut state_encoded = Vec::new();
    let mut state = File::open("model.bin")?;
    state.read_to_end(&mut state_encoded)?;

    let model: Model<NdArrayBackend> = Model::new();
    let record = BinBytesRecorder::<FullPrecisionSettings>::default()
        .load(STATE_ENCODED.to_vec())
        .expect("Failed to decode state");

    Ok(model.load_record(record))
}

fn save_to_img(idx: usize, tensor: &Tensor<NdArrayBackend, 3>) {
    let mut img_buffer = ImageBuffer::new(28, 28);
    for (x, row) in tensor.clone().reshape([28, 28]).iter_dim(0).enumerate() {
        for (y, col) in row.reshape([28]).to_data().value.iter().enumerate() {
            img_buffer.put_pixel(
                x as u32,
                y as u32,
                image::Rgb([
                    (*col * 255.) as u8,
                    (*col * 255.) as u8,
                    (*col * 255.) as u8,
                ]),
            );
        }
    }
    img_buffer.save(format!("input{}.png", idx)).unwrap();
}

fn load_input_data_from_candle() -> anyhow::Result<(Tensor<NdArrayBackend, 3>, Vec<u32>)> {
    let input =
        candle_datasets::vision::mnist::load_dir("/home/yjn/Downloads/mnist_dataset").unwrap();
    let test_images: candle_core::Tensor =
        input.test_images.to_device(&candle_core::Device::Cpu)?;
    println!("{:?}", test_images.shape());
    let bsz = test_images.shape().dims2()?.0;
    let test_images = test_images.reshape((1, ()))?.get(0)?;
    let test_images = test_images.to_vec1::<f32>()?;
    let test_labels: Vec<u32> = input
        .test_labels
        .to_dtype(candle_core::DType::U32)?
        .to_device(&candle_core::Device::Cpu)?
        .to_vec1()?;

    let input_images: Tensor<NdArrayBackend, 1> = Tensor::from_floats(test_images.as_slice());
    let input_images = input_images.reshape([bsz, 28, 28]);
    Ok((input_images, test_labels))
}

fn inference(model: Model<NdArrayBackend>) -> anyhow::Result<()> {
    let (input_images, test_labels) = load_input_data_from_candle().unwrap();

    for (idx, image) in input_images.iter_dim(0).enumerate() {
        save_to_img(idx, &image);
        let t = SystemTime::now();
        let output = model.forward(image);
        let output = burn::tensor::activation::softmax(output, 1);
        let label = output.argmax(1).reshape([1]).into_scalar();

        println!(
            "expect label: {}, got: {}, take: {}",
            test_labels.get(idx).unwrap(),
            label,
            t.elapsed().unwrap().as_millis()
        );
        if idx == 9 {
            break;
        }
    }

    // let test_labels = input
    //     .test_labels
    //     .to_dtype(candle_core::DType::U32)?
    //     .to_device(&candle_core::Device::Cpu)?;
    // println!("test_images shape: {:?}", test_images.shape());

    // let sum_ok = test_logits
    //     .argmax(D::Minus1)?
    //     .eq(&test_labels)?
    //     .to_dtype(DType::F32)?
    //     .sum_all()?
    //     .to_scalar::<f32>()?;

    // let test_accuracy = sum_ok / test_labels.dims1()? as f32;
    // println!("{:5.2}%", 100. * test_accuracy);

    Ok(())
}

fn main() {
    let model = build_and_load_model().expect("load model failed");

    let t = SystemTime::now();
    inference(model).unwrap();
    println!("infer take time: {}ms", t.elapsed().unwrap().as_millis())
}
