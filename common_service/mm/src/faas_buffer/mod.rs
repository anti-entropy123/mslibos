extern crate alloc;

use alloc::string::String;
use hashbrown::HashMap;
use lazy_static::lazy_static;

use spin::Mutex;

mod apis;

lazy_static! {
    static ref BUFFER_REGISTER: Mutex<HashMap<String, (usize, u64)>> = Mutex::new(HashMap::new());
}
static DEFAULT_BUFFER_ENTRY: Mutex<Option<(usize, u64)>> = Mutex::new(None);
