use std::{ops::Deref, ptr::NonNull};

/// Represents a weakly linked of non-owning pointer to T.
struct WeakNode<T> {
    inner: NonNull<T>,
    next: Option<NonNull<WeakNode<T>>>,
}

impl<T> WeakNode<T> {
    fn new(inner: NonNull<T>) -> NonNull<Self> {
        let node = WeakNode { inner, next: None };
        let node = Box::new(node);
        let raw = Box::into_raw(node);

        unsafe { NonNull::new(raw).unwrap_unchecked() }
    }
}

impl<T> Deref for WeakNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref() }
    }
}

/// Represents a singly linked list of weakly referenced T.
pub struct WeakList<T> {
    head: Option<NonNull<WeakNode<T>>>,
}

unsafe impl<T> Sync for WeakList<T> {}
unsafe impl<T> Send for WeakList<T> {}

impl<T> WeakList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, inner: NonNull<T>) {
        let new_node = WeakNode::new(inner);

        unsafe {
            (*new_node.as_ptr()).next = self.head;
        }

        self.head = Some(new_node);
    }

    pub fn iter(&self) -> impl Iterator<Item = NonNull<T>> {
        WeakListIterator { current: self.head }
    }
}

impl<T> Drop for WeakList<T> {
    fn drop(&mut self) {
        let mut current = self.head;

        while let Some(node_ptr) = current {
            unsafe {
                let node = Box::from_raw(node_ptr.as_ptr());
                current = node.next;
                drop(node);
            }
        }
    }
}

pub struct WeakListIterator<T> {
    current: Option<NonNull<WeakNode<T>>>,
}

impl<T> Iterator for WeakListIterator<T> {
    type Item = NonNull<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node_ptr) = self.current {
            unsafe {
                let node = node_ptr.as_ref();
                self.current = node.next;
                Some(node.inner)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weak_list() {
        struct TestData {
            value: i32,
        }

        let mut list = WeakList::new();

        let data1 = Box::new(TestData { value: 10 });
        let data2 = Box::new(TestData { value: 20 });
        let data3 = Box::new(TestData { value: 30 });

        let ptr1 = NonNull::new(Box::into_raw(data1)).unwrap();
        let ptr2 = NonNull::new(Box::into_raw(data2)).unwrap();
        let ptr3 = NonNull::new(Box::into_raw(data3)).unwrap();

        list.push(ptr1);
        list.push(ptr2);
        list.push(ptr3);

        let mut values = vec![];
        for data_ptr in list.iter() {
            unsafe {
                values.push(data_ptr.as_ref().value);
            }
        }

        assert_eq!(values, vec![30, 20, 10], "Values be in reverse order");

        unsafe {
            drop(Box::from_raw(ptr1.as_ptr()));
            drop(Box::from_raw(ptr2.as_ptr()));
            drop(Box::from_raw(ptr3.as_ptr()));
        }
    }
}
