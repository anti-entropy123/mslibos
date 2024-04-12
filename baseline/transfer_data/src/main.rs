use transfer_data::{share_mem::ShareMem, TransferData};

fn main() {
    let (s, r) = ShareMem::build(128 * 1024);

    let start = s.send();
    let dur = r.receive().duration_since(start).unwrap();

    println!("dur: {:?}", dur)
}
