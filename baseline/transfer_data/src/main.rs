use transfer_data::{socket::TcpSocket, TransferData};

fn main() {
    let (s, r) = TcpSocket::build(128 * 1024);

    let start = s.send();
    let dur = r.receive().duration_since(start).unwrap();

    println!("dur: {:?}", dur)
}
