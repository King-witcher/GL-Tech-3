use std::{
    alloc::Layout,
    ops::{AddAssign, Deref, DerefMut},
    ptr::NonNull,
    sync::RwLock,
};

pub struct MutarcInner<T: Sized> {
    data: T,
    counter: RwLock<u16>,
}

pub struct Mutarc<T: Sized> {
    inner: NonNull<MutarcInner<T>>,
}

impl<T: Sized> Mutarc<T> {
    pub fn new(data: T) -> Self {
        let inner = MutarcInner {
            data,
            counter: RwLock::new(1),
        };

        let layout = Layout::new::<MutarcInner<T>>();
        unsafe {
            let ptr = std::alloc::alloc(layout) as *mut MutarcInner<T>;
            std::ptr::write(ptr, inner);

            Self {
                inner: NonNull::new_unchecked(ptr),
            }
        }
    }
}

impl<T: Sized> Clone for Mutarc<T> {
    fn clone(&self) -> Self {
        let mut lock = unsafe { self.inner.as_ref().counter.write().unwrap() };
        lock.add_assign(1);

        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Sized> Drop for Mutarc<T> {
    fn drop(&mut self) {
        let mut lock = unsafe { self.inner.as_ref().counter.write().unwrap() };
        *lock -= 1;
        let should_deallocate = *lock == 0;
        drop(lock); // Release the lock early

        if should_deallocate {
            unsafe {
                // Properly drop the inner data first
                std::ptr::drop_in_place(self.inner.as_ptr());

                let layout = Layout::new::<MutarcInner<T>>();
                std::alloc::dealloc(self.inner.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T: Sized> Deref for Mutarc<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &self.inner.as_ref().data }
    }
}

impl<T: Sized> DerefMut for Mutarc<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.inner.as_mut().data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref() {
        let arc = Mutarc::new(5);
        assert_eq!(*arc, 5);
    }

    #[test]
    fn clone() {
        let arc1 = Mutarc::new(10);
        let mut arc2 = arc1.clone();
        *arc2 = 20;
        assert_eq!(*arc1, 20);
        assert_eq!(*arc1, *arc2);
    }
}
