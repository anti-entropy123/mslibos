use core::{borrow::Borrow, mem::size_of};

use alloc::rc::Rc;

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
        Self {
            inner: Rc::default(),
            data_size: size_of::<T>(),
        }
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
