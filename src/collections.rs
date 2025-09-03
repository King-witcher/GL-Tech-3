use std::sync::{Arc, RwLock};

pub struct List<T> {
    pub first: Option<Arc<RwLock<ListNode<T>>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { first: None }
    }

    pub fn add(&mut self, value: T) {
        let new_node = Arc::new(RwLock::new(ListNode {
            value: Arc::new(value),
            next: self.first.clone(),
        }));
        self.first = Some(new_node);
    }

    fn find_and_raise<P>(predicate: P) -> Option<Arc<RwLock<T>>>
    where
        P: Fn(f32) -> f32,
    {
    }

    // Highly optimizable
    fn raise(&mut self, prev: Arc<RwLock<ListNode<T>>>) {
        let mut prev_rw = prev.write().expect("failed to get write lock");
        let current = prev_rw.next.clone().expect("called raise on last item");
        let mut current_rw = current.write().expect("failed to get write lock");
        prev_rw.next = current_rw.next.clone();
        current_rw.next = self.first.clone();
        self.first = Some(current.clone());
    }
}

struct ListNode<T> {
    pub value: Arc<T>,
    pub next: Option<Arc<RwLock<ListNode<T>>>>,
}

impl<T> ListNode<T> {
    pub fn new(value: Arc<T>) -> Self {
        Self { value, next: None }
    }
}
