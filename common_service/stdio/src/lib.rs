#[no_mangle]
pub fn host_stdout(buf: &str) -> isize {
    print!("{}", buf);
    buf.len() as isize
}
