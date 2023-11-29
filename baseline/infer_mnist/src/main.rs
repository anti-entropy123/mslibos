use std::time::SystemTime;

use candle_core::{DType, Device, Result, Tensor, D};
use candle_nn::{Conv2d, Linear, VarBuilder, VarMap};

const LABELS: usize = 10;

#[derive(Debug)]
struct ConvNet {
    conv1: Conv2d,
    conv2: Conv2d,
    fc1: Linear,
    fc2: Linear,
}

impl ConvNet {
    fn new(vs: VarBuilder) -> Result<Self> {
        let conv1 = candle_nn::conv2d(1, 32, 5, Default::default(), vs.pp("c1"))?;
        let conv2 = candle_nn::conv2d(32, 64, 5, Default::default(), vs.pp("c2"))?;
        let fc1 = candle_nn::linear(1024, 1024, vs.pp("fc1"))?;
        let fc2 = candle_nn::linear(1024, LABELS, vs.pp("fc2"))?;

        Ok(Self {
            conv1,
            conv2,
            fc1,
            fc2,
        })
    }

    fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        let (b_sz, _img_dim) = xs.dims2()?;
        xs.reshape((b_sz, 1, 28, 28))?
            .apply(&self.conv1)?
            .max_pool2d(2)?
            .apply(&self.conv2)?
            .max_pool2d(2)?
            .flatten_from(1)?
            .apply(&self.fc1)?
            .relu()?
            .apply(&self.fc2)
    }
}

fn infer(m: candle_datasets::vision::Dataset) -> anyhow::Result<()> {
    let dev = Device::Cpu;
    let mut varmap = VarMap::new();
    let vs = VarBuilder::from_varmap(&varmap, DType::F32, &dev);
    let model = ConvNet::new(vs)?;
    varmap.load("./mnist_model_file")?;

    let test_images = m.test_images.to_device(&dev)?;
    let test_labels = m.test_labels.to_dtype(DType::U32)?.to_device(&dev)?;
    // println!("test_images shape: {:?}", test_images.shape());
    // println!("test_labels shape: {:?}", test_labels.shape());

    let test_logits = model.forward(&test_images)?;
    // println!("test_logits shape: {:?}", test_logits.shape());

    // let t1 = test_logits.argmax(D::Minus1)?;
    // println!("t1 shape: {:?}, t1[0]={:?}", t1.shape(), t1.get(0).unwrap());

    // let t2 = t1.eq(&test_labels)?;
    // println!("t2 shape: {:?}, t2[0]={:?}", t2.shape(), t2.get(0).unwrap());

    // let t3 = t2.to_dtype(DType::F32)?;
    // println!("t3 shape: {:?}, t3[0]={:?}", t3.shape(), t3.get(0).unwrap());

    // let t4 = t3.sum_all()?;
    // println!("t4 shape: {:?}, t4[0]={:?}", t4.shape(), t4.get(0).unwrap());

    // let sum_ok = t4.to_scalar::<f32>()?;
    let sum_ok = test_logits
        .argmax(D::Minus1)?
        .eq(&test_labels)?
        .to_dtype(DType::F32)?
        .sum_all()?
        .to_scalar::<f32>()?;

    let test_accuracy = sum_ok / test_labels.dims1()? as f32;
    println!("{:5.2}%", 100. * test_accuracy);

    Ok(())
}

fn main() {
    let m = candle_datasets::vision::mnist::load_dir("/home/yjn/Downloads/mnist_dataset").unwrap();

    let t = SystemTime::now();
    infer(m).unwrap();
    println!("infer take time: {}ms", t.elapsed().unwrap().as_millis())
}
