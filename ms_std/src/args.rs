use core::str::FromStr;

#[derive(Debug)]
pub(crate) struct ArgsItem {
    pub(crate) key: heapless::String<32>,
    pub(crate) val: heapless::String<32>,
}

impl ArgsItem {
    pub(crate) fn from_kv(k: &str, v: &str) -> Self {
        Self {
            key: heapless::String::from_str(k).unwrap(),
            val: heapless::String::from_str(v).unwrap(),
        }
    }
}

pub(crate) static mut ARGS_LIST: heapless::Vec<ArgsItem, 16> = heapless::Vec::new();

pub fn get(name: &str) -> Option<&'static str> {
    let mut args_base_addr: usize;
    unsafe {
        core::arch::asm!(
            "mov {}, rsp", out(reg) args_base_addr
        )
    };
    let page_size = 4096;
    let args_base_addr = (args_base_addr + page_size - 1) & (!page_size + 1);
    let args_list = unsafe { &mut *(args_base_addr as *mut heapless::Vec<ArgsItem, 16>) };

    for item in args_list {
        if item.key == name {
            return Some(item.val.as_str());
        }
    }

    None
}
