use core::{alloc::Layout, borrow::Borrow};

use alloc::{rc::Rc, string::String};
use ms_hostcall::Verify;

use crate::{libos::libos, println};

use ms_std_proc_macro::Verify as VerifyMacro;

pub type FaaSFuncResult<T> = Result<DataBuffer<T>, ()>;

#[derive(Debug)]
pub struct DataBuffer<T> {
    inner: Rc<T>,
    // fingerprint: u64,
}

impl<T> DataBuffer<T>
where
    T: Verify,
{
    pub fn new() -> Self
    where
        T: Default,
    {
        Self::with_slot(String::new())
    }

    pub fn with_slot(slot: String) -> Self
    where
        T: Default,
    {
        let p = {
            let l: Layout = Layout::new::<Rc<T>>();
            let fingerprint = T::__fingerprint();
            // println!("T::__fingerprint: {}", fingerprint);
            libos!(buffer_alloc(slot, l, fingerprint)).expect("alloc failed.")
        };
        let raw_ptr = {
            let buffer = p as *mut Rc<T>;
            unsafe { core::ptr::write(buffer, Rc::default()) }
            let rc = unsafe { Rc::clone(&(*buffer)) };

            Rc::into_raw(rc)
        };

        let inner = unsafe {
            // must guarantee strong == 1, otherwise will unable to get mut ref.
            Rc::decrement_strong_count(raw_ptr);
            Rc::from_raw(raw_ptr)
        };
        assert_eq!(Rc::strong_count(&inner), 1);

        Self {
            inner,
            // fingerprint: T::__fingerprint(),
        }
    }

    pub fn from_buffer() -> Option<Self> {
        Self::from_buffer_slot(String::new())
    }

    pub fn from_buffer_slot(slot: String) -> Option<Self> {
        let buffer_meta: Option<(usize, u64)> = libos!(access_buffer(slot));

        buffer_meta.map(|(raw_ptr, fingerprint)| {
            if fingerprint != T::__fingerprint() {
                println!("wrong data type, {}, {}", fingerprint, T::__fingerprint());
                panic!("");
            };

            let raw_ptr = {
                let buffer = raw_ptr as *mut Rc<T>;
                let rc = unsafe { Rc::clone(&(*buffer)) };

                Rc::into_raw(rc)
            };

            let inner = unsafe {
                // must guarantee strong == 1, otherwise will unable to get mut ref.
                Rc::decrement_strong_count(raw_ptr);
                Rc::from_raw(raw_ptr)
            };
            assert_eq!(Rc::strong_count(&inner), 1);

            Self {
                inner,
                // fingerprint: T::__fingerprint(),
            }
        })
    }
}

impl<T> Default for DataBuffer<T>
where
    T: Default + Verify,
{
    fn default() -> DataBuffer<T> {
        Self::new()
    }
}

impl<T> core::ops::Deref for DataBuffer<T>
where
    T: Verify,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // assert_eq!(T::__fingerprint(), self.fingerprint);
        self.inner.borrow()
    }
}

impl<T> core::ops::DerefMut for DataBuffer<T>
where
    T: Verify,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        // assert_eq!(T::__fingerprint(), self.fingerprint);
        Rc::get_mut(&mut self.inner).unwrap()
    }
}

impl<T> From<T> for DataBuffer<T>
where
    T: Default + Verify,
{
    fn from(value: T) -> Self {
        let mut t = DataBuffer::<T>::default();
        *t = value;
        t
    }
}

impl<T> Drop for DataBuffer<T> {
    fn drop(&mut self) {
        // println!("drop DataBuffer");
        assert_eq!(Rc::strong_count(&self.inner), 1);
        let c = Rc::into_raw(Rc::clone(&self.inner));
        unsafe {
            Rc::increment_strong_count(c);
        };
        let _ = unsafe { Rc::from_raw(c) };

        assert_eq!(Rc::strong_count(&self.inner), 2);
    }
}

#[derive(VerifyMacro, Default)]
pub struct Zero {}
