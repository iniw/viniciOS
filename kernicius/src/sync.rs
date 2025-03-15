use core::ops::Deref;

pub struct ThreadUnsafeGlobal<T>(T);

unsafe impl<T> Sync for ThreadUnsafeGlobal<T> {}

impl<T> ThreadUnsafeGlobal<T> {
    pub const unsafe fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for ThreadUnsafeGlobal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for ThreadUnsafeGlobal<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
