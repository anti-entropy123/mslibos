#![no_std]
extern crate alloc;

use alloc::{collections::BTreeMap, string::String};

use burn::{
    module::Module,
    nn::{self, BatchNorm, PaddingConfig2d},
    record::{BinBytesRecorder, FullPrecisionSettings, Recorder},
    tensor::{backend::Backend, Distribution, Tensor},
};
use ms_std::{
    agent::{FaaSFuncError, FaaSFuncResult as Result},
    fs::File,
    mm::Mmap,
    println,
};
use ms_std_proc_macro::FaasData;

const NUM_CLASSES: usize = 10;

pub type NdArrayBackend = burn::backend::ndarray::NdArray;

#[derive(FaasData)]
struct ConvFaasInput {}

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
        let fc2 = nn::LinearConfig::new(32, NUM_CLASSES)
            .with_bias(false)
            .init();

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

pub fn build_and_load_model() -> core::result::Result<Model<NdArrayBackend>, FaaSFuncError> {
    let state = File::open("model.bin")?;
    let state_encoded = Mmap::mmap_file(state)?;
    println!("mmap_file ok");

    let model: Model<NdArrayBackend> = Model::new();
    let record = BinBytesRecorder::<FullPrecisionSettings>::default()
        .load(state_encoded.as_ref().to_vec())
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

#[no_mangle]
pub fn main(_args: &BTreeMap<String, String>) -> Result<()> {
    let model = build_and_load_model()?;
    println!("load model successfully");
    inference(model);

    Ok(().into())
}
