use core::{alloc::Layout, borrow::Borrow, mem::ManuallyDrop};

use alloc::{boxed::Box, rc::Rc, string::String};
use ms_hostcall::Verify;

use crate::{libos::libos, println};

pub type FaaSFuncResult<T> = Result<DataBuffer<T>, ()>;

#[derive(Debug)]
pub struct DataBuffer<T> {
    inner: ManuallyDrop<Box<T>>,
    // fingerprint: u64,
    used: bool,
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
            let l: Layout = Layout::new::<T>();
            let fingerprint = T::__fingerprint();
            // println!("T::__fingerprint: {}", fingerprint);
            libos!(buffer_alloc(slot, l, fingerprint)).expect("alloc failed.") as *mut T
        };

        unsafe { core::ptr::write(p, T::default()) };
        let inner = unsafe { Box::from_raw(p) };
        // let inner = unsafe {
        //     // must guarantee strong == 1, otherwise will unable to get mut ref.
        //     Rc::decrement_strong_count(raw_ptr);
        //     Rc::from_raw(raw_ptr)
        // };
        // assert_eq!(Rc::strong_count(&inner), 1);

        Self {
            inner: ManuallyDrop::new(inner),
            used: false,
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

            let inner = unsafe { Box::from_raw(raw_ptr as *mut T) };

            // let inner = unsafe {
            //     // must guarantee strong == 1, otherwise will unable to get mut ref.
            //     Rc::decrement_strong_count(raw_ptr);
            //     Rc::from_raw(raw_ptr)
            // };
            // assert_eq!(Rc::strong_count(&inner), 1);

            Self {
                inner: ManuallyDrop::new(inner),
                // fingerprint: T::__fingerprint(),
                used: true,
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
        // Rc::get_mut(&mut self.inner).unwrap()
        self.inner.as_mut()
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
        // assert_eq!(Rc::strong_count(&self.inner), 1);
        // let c = Rc::into_raw(Rc::clone(&self.inner));
        // unsafe {
        //     Rc::increment_strong_count(c);
        // };
        // let _ = unsafe { Rc::from_raw(c) };

        // assert_eq!(Rc::strong_count(&self.inner), 2);
        if self.used {
            println!("drop DataBuffer val");
            unsafe { drop(ManuallyDrop::take(&mut self.inner)) };
        }
    }
}
