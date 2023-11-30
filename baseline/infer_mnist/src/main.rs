use std::{fs::File, io::Read, time::SystemTime};

use burn::{
    module::Module,
    nn::{self, BatchNorm, PaddingConfig2d},
    record::{BinBytesRecorder, FullPrecisionSettings, Recorder},
    tensor::{backend::Backend, Distribution, Tensor},
};

const LABELS: usize = 10;

pub type NdArrayBackend = burn::backend::ndarray::NdArray;

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
    pub fn new(channels: [usize; 2], kernel_size: [usize; 2]) -> Self {
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

    pub fn forward(&self, input: Tensor<B, 4>) -> Tensor<B, 4> {
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

// static STATE_ENCODED: &[u8] = include_bytes!("../model.bin");

pub fn build_and_load_model() -> anyhow::Result<Model<NdArrayBackend>> {
    let mut state_encoded = Vec::new();
    let mut state = File::open("model.bin")?;
    state.read_to_end(&mut state_encoded);

    println!("mmap_file ok");

    let model: Model<NdArrayBackend> = Model::new();
    let record = BinBytesRecorder::<FullPrecisionSettings>::default()
        .load(state_encoded)
        .expect("Failed to decode state");

    Ok(model.load_record(record))
}

pub fn inference(model: Model<NdArrayBackend>) {
    let input: Tensor<NdArrayBackend, 3> =
        Tensor::random([1, 28, 28], Distribution::Normal(0., 100.));

    let output = model.forward(input);
    let output = burn::tensor::activation::softmax(output, 1);
    let output = output.into_data().convert::<f32>().value;

    println!("{:?}", output)
}

fn infer(
    input: candle_datasets::vision::Dataset,
    model: Model<NdArrayBackend>,
) -> anyhow::Result<()> {
    let test_images = input.test_images.to_device(&candle_core::Device::Cpu)?;
    println!("{:?}", test_images.shape());
    let bsz = test_images.shape().dims2()?.0;
    let test_images = test_images.reshape((1, ()))?.get(0)?;
    let test_images = test_images.to_vec1::<f32>()?;
    let test_labels: Vec<u32> = input
        .test_labels
        .to_dtype(candle_core::DType::U32)?
        .to_device(&candle_core::Device::Cpu)?
        .to_vec1()?;

    // println!("label[0] {:?}", test_labels.get(0)?);

    let input_images: Tensor<NdArrayBackend, 1> = Tensor::from_floats(test_images.as_slice());
    let input_images = input_images.reshape([bsz, 28, 28]);
    println!("{:?}", input_images.shape());

    for (idx, i) in input_images.iter_dim(0).enumerate() {
        let output = model.forward(i);
        let output = burn::tensor::activation::softmax(output, 1);
        let label = output.argmax(1).reshape([1]).into_scalar();

        println!(
            "expect label: {}, got: {}",
            test_labels.get(idx).unwrap(),
            label
        );
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
    let input =
        candle_datasets::vision::mnist::load_dir("/home/yjn/Downloads/mnist_dataset").unwrap();
    let model = build_and_load_model().expect("load model failed");

    let t = SystemTime::now();
    infer(input, model).unwrap();
    println!("infer take time: {}ms", t.elapsed().unwrap().as_millis())
}
