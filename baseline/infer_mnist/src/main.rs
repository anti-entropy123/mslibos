use std::{
    fs::File,
    io::{Read, Write},
    iter::zip,
    time::SystemTime,
};

use burn::{
    module::Module,
    nn::{self, BatchNorm, PaddingConfig2d},
    record::{BinBytesRecorder, FullPrecisionSettings, Recorder},
    tensor::{backend::Backend, Tensor},
};

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

#[allow(dead_code)]
fn save_to_file(image_tensor: &Vec<u8>, labels_data: &Vec<u8>) {
    // let mut img_buffer = ImageBuffer::new(28, 28);
    let mut images = File::create("input_image.bin").unwrap();
    images.write_all(image_tensor.as_slice()).unwrap();

    let mut labels = File::create("labels.bin").unwrap();
    labels.write_all(labels_data.as_slice()).unwrap()
}

fn load_input_data_from_candle() -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let data_set =
        candle_datasets::vision::mnist::load_dir("/home/yjn/Downloads/mnist_dataset").unwrap();

    let test_images: candle_core::Tensor =
        data_set.test_images.to_device(&candle_core::Device::Cpu)?;
    let test_images = test_images.reshape((1, ()))?.get(0)?;
    let test_images: Vec<u8> = test_images
        .to_vec1::<f32>()?
        .iter()
        .map(|v| (v * 255.) as u8)
        .collect();

    let test_labels: Vec<u8> = data_set
        .test_labels
        .to_dtype(candle_core::DType::U8)?
        .to_device(&candle_core::Device::Cpu)?
        .to_vec1()?;

    Ok((test_images, test_labels))
}

fn tranform_image_tensor(input: Vec<u8>) -> Tensor<NdArrayBackend, 3> {
    Tensor::from_floats(
        input
            .iter()
            .map(|v| ((*v as f32 / 255.) - 0.1307) / 0.3081)
            .collect::<Vec<f32>>()
            .as_slice(),
    )
    .reshape([-1, 28, 28])
}

fn inference(model: Model<NdArrayBackend>) -> anyhow::Result<()> {
    let (input_images, test_labels) = load_input_data_from_candle().unwrap();
    // println!("input_images: {:?}", input_images);
    // save_to_file(&input_images, &test_labels);

    let input_images = tranform_image_tensor(input_images);
    // println!("input_images: {:?}", input_images);

    let bsz = input_images.shape().dims[0];
    let images_and_labels = zip(input_images.iter_dim(0), test_labels);

    let mut correct = 0;
    for (image, expect_label) in images_and_labels {
        let t = SystemTime::now();
        let output = model.forward(image);
        let output = burn::tensor::activation::softmax(output, 1);
        let label = output.argmax(1).reshape([1]).into_scalar() as u8;
        // println!(
        //     "expect label: {}, got: {}, take: {}",
        //     .get(idx).unwrap(),
        //     label,
        //     t.elapsed().unwrap().as_millis()
        // );
        if expect_label == label {
            correct += 1;
        }
    }
    println!("test_accuracy: {}", correct as f32 / bsz as f32);
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
