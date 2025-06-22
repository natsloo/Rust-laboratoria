#[derive (Debug)]
pub struct Node<T> {
    value: T,
    previous: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            previous: None
        }
    }
    pub fn value(&self) -> &T {
        &self.value
    }
    pub fn previous(self) -> Option<Box<Node<T>>> {
        self.previous
    }
    pub fn set_previous(&mut self, prev: Node<T>) {
        self.previous = Some(Box::new(prev))
    }
}