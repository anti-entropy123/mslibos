#![no_std]
pub use ms_std;

#[no_mangle]
pub fn main() -> ms_std::agent::FaaSFuncResult<()> {
    panic!("should return err")
}
