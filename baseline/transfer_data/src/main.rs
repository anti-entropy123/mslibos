#[allow(unused)]
use transfer_data::{function::Function, socket::TcpSocket, TransferData};

fn main() {
    let (s, r) = Function::build(128 * 1024);

    let start = s.send();
    let dur = r.receive().duration_since(start).unwrap();

    println!("dur: {:?}", dur)
}
