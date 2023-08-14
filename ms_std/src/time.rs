use core::time::Duration;

use crate::libos::libos;

#[derive(Clone, Copy)]
pub struct SystemTime {
    nanos: u128,
}

impl SystemTime {
    pub fn now() -> Self {
        Self {
            nanos: libos!(get_time()).unwrap(),
        }
    }

    pub fn duration_since(&self, earlier: SystemTime) -> Duration {
        let sub = self.nanos - earlier.nanos;
        Duration::new((sub / 1_000_000_000) as u64, (sub % 1_000_000_000) as u32)
    }

    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
}
