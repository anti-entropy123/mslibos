use core::{alloc::Layout, borrow::Borrow};

use alloc::rc::Rc;
use ms_hostcall::Verify;

use crate::libos::libos;

use ms_std_proc_macro::Verify as VerifyMacro;

pub type FaaSFuncResult<T> = Result<DataBuffer<T>, ()>;

#[derive(Debug)]
pub struct DataBuffer<T>
where
    T: Verify,
{
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
        let l = Layout::new::<Rc<T>>();

        let raw_ptr = {
            let p = libos!(buffer_alloc(l)).expect("alloc failed.");
            let buffer = p as *mut Rc<T>;
            unsafe { core::ptr::write(buffer, Rc::default()) }
            let rc = unsafe {
                let rc = Rc::clone(&(*buffer));
                // must guarantee strong == 1 and weak == 0.
                buffer.drop_in_place();
                rc
            };

            Rc::into_raw(rc)
        };

        Self {
            inner: unsafe { Rc::from_raw(raw_ptr) },
            // fingerprint: T::__fingerprint(),
        }
    }

    pub fn from_buffer() -> Option<Self> {
        let raw_ptr = libos!(access_buffer());

        raw_ptr.map(|raw_ptr| Self {
            inner: unsafe { Rc::clone(&*(raw_ptr as *mut Rc<T>)) },
            // fingerprint: T::__fingerprint(),
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

#[derive(VerifyMacro, Default)]
pub struct Zero {}
