use std::time::SystemTime;

pub mod share_mem;

pub trait TransferData {
    fn build(data_size: usize) -> (Box<dyn DataSender>, Box<dyn DataReceiver>);
}

pub trait DataSender {
    fn send(&self) -> SystemTime;
}

pub trait DataReceiver {
    fn receive(&self) -> SystemTime;
}
