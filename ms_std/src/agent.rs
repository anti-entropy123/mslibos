use core::{alloc::Layout, borrow::Borrow, mem::size_of};

use alloc::rc::Rc;

use crate::libos::libos;

pub type FaaSFuncResult<T> = Result<DataBuffer<T>, ()>;

pub struct DataBuffer<T> {
    inner: Rc<T>,
    data_size: usize,
}

impl<T> DataBuffer<T> {
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
            data_size: size_of::<T>(),
        }
    }

    pub fn from_buffer() -> Option<Self> {
        let raw_ptr = libos!(access_buffer());

        raw_ptr.map(|raw_ptr| Self {
            inner: unsafe { Rc::clone(&*(raw_ptr as *mut Rc<T>)) },
            data_size: size_of::<T>(),
        })
    }
}

impl<T> Default for DataBuffer<T>
where
    T: Default,
{
    fn default() -> DataBuffer<T> {
        Self::new()
    }
}

impl<T> core::ops::Deref for DataBuffer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        assert_eq!(size_of::<Self::Target>(), self.data_size);
        self.inner.borrow()
    }
}

impl<T> core::ops::DerefMut for DataBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert_eq!(size_of::<Self::Target>(), self.data_size);
        Rc::get_mut(&mut self.inner).unwrap()
    }
}

impl<T> From<T> for DataBuffer<T>
where
    T: Default,
{
    fn from(value: T) -> Self {
        let mut t = DataBuffer::<T>::default();
        *t = value;
        t
    }
}
