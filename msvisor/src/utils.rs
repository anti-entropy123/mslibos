use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref ID_KEEPER: Mutex<u64> = Mutex::new(1);
}

pub fn gen_new_id() -> u64 {
    let mut keeper = ID_KEEPER.lock().unwrap();
    let result = *keeper;
    *keeper += 1;
    result
}
