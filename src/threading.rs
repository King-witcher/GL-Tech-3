use std::{
    alloc::Layout,
    ops::{AddAssign, Deref, DerefMut},
    ptr::NonNull,
    sync::RwLock,
};

pub struct GLArcInner<T: Sized> {
    data: T,
    counter: RwLock<u16>,
}

pub struct GLArc<T: Sized> {
    inner: NonNull<GLArcInner<T>>,
}

impl<T: Sized> GLArc<T> {
    pub fn new(data: T) -> Self {
        let inner = GLArcInner {
            data,
            counter: RwLock::new(1),
        };

        let layout = Layout::new::<GLArcInner<T>>();
        unsafe {
            let ptr = std::alloc::alloc(layout) as *mut GLArcInner<T>;
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            std::ptr::write(ptr, inner);

            Self {
                inner: NonNull::new_unchecked(ptr),
            }
        }
    }

    #[inline]
    pub unsafe fn get_mutable_ref(&self) -> &mut T {
        unsafe { &mut (*self.inner.as_ptr()).data }
    }
}

impl<T: Sized> Clone for GLArc<T> {
    fn clone(&self) -> Self {
        let mut lock = unsafe { self.inner.as_ref().counter.write().unwrap() };
        lock.add_assign(1);
        println!("{} instances", lock);

        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Sized> Drop for GLArc<T> {
    fn drop(&mut self) {
        let mut lock = unsafe { self.inner.as_ref().counter.write().unwrap() };
        *lock -= 1;
        let should_deallocate = *lock == 0;
        drop(lock); // Release the lock early

        if should_deallocate {
            unsafe {
                // Properly drop the inner data first
                std::ptr::drop_in_place(self.inner.as_ptr());

                let layout = Layout::new::<GLArcInner<T>>();
                std::alloc::dealloc(self.inner.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T: Sized> Deref for GLArc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.inner.as_ref().data }
    }
}

impl<T: Sized> DerefMut for GLArc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.inner.as_mut().data }
    }
}
