use std::sync::Mutex;

use lazy_static::lazy_static;

pub const PAGE_SIZE: usize = 0x1000;

lazy_static! {
    static ref ID_KEEPER: Mutex<u64> = Mutex::new(1);
}

pub fn gen_new_id() -> u64 {
    let mut keeper = ID_KEEPER.lock().unwrap();
    let result = *keeper;
    *keeper += 1;
    result
}

#[macro_export]
macro_rules! round_up {
    ($x:expr) => {{
        use $crate::utils::PAGE_SIZE;
        ($x as usize + PAGE_SIZE - 1) & (!PAGE_SIZE + 1)
    }};
}

#[macro_export]
macro_rules! round_down {
    ($x:expr) => {{
        use $crate::utils::PAGE_SIZE;
        ($x as usize) & (!PAGE_SIZE + 1)
    }};
}

#[macro_export]
macro_rules! now_millis {
    () => {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    };
}

#[test]
fn test_now_millis() {
    let now = now_millis!();
    assert!(now > 0, "now:{} not > 0", now)
}

#[test]
fn test_round_page() {
    assert_eq!(round_down!(0x1001), 0x1000);
    assert_eq!(round_down!(0x1fff), 0x1000);
    assert_eq!(round_down!(0x2000), 0x2000);

    assert_eq!(round_up!(0x2001), 0x3000);
    assert_eq!(round_up!(0x1fff), 0x2000);
    assert_eq!(round_up!(0x2000), 0x2000);
}
