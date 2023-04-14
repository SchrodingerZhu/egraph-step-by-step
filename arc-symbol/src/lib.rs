use std::sync::Arc;

#[repr(transparent)]
#[derive(Debug)]
pub struct Symbol<T>(Arc<T>);

impl<T> Clone for Symbol<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Symbol<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(value))
    }
}

impl<T> PartialEq for Symbol<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for Symbol<T> {}

impl<T> std::hash::Hash for Symbol<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.0).hash(state)
    }
}




