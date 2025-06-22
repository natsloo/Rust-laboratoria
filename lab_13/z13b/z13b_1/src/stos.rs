use crate::node::Node;

struct Stos<T> {
    head: Option<Box<Node<T>>>
}

impl<T: std::fmt::Debug> Stos<T> {
    fn new() -> Self {
        Self {
            head: None
        }
    }
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    fn top(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.value()),
            None => None
        }
    }
    fn pop(&mut self) {
        match self.head.take() {
            Some(node) => self.head = node.previous(),
            None => {}
        }
    }
    fn push(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        match self.head.take() {
            Some(node) => new_node.set_previous(*node),
            None => {}
        }
        self.head = Some(new_node)
    }
}

#[cfg(test)]
mod tests {
    use super::Stos;

    #[test]
    fn test_push_and_top_i32() {
        let mut stos = Stos::new();
        assert!(stos.is_empty());

        stos.push(1);
        assert!(!stos.is_empty());
        assert_eq!(stos.top(), Some(&1));

        stos.push(2);
        assert_eq!(stos.top(), Some(&2));
    }

    #[test]
    fn test_pop_and_is_empty_i32() {
        let mut stos = Stos::new();
        assert_eq!(stos.pop(), ());

        stos.push(10);
        stos.push(20);

        assert_eq!(stos.top(), Some(&20));
        stos.pop();
        assert_eq!(stos.top(), Some(&10));
        assert_eq!(stos.is_empty(), false);
        stos.pop();
        assert_eq!(stos.is_empty(), true);
    }

    #[test]
    fn test_string() {
        let mut stos = Stos::new();
        assert_eq!(stos.pop(), ());
        assert!(stos.is_empty());
        stos.push("S");
        stos.push("T");
        stos.push("O");
        stos.push("S");
        assert!(!stos.is_empty());
        assert_eq!(stos.top(), Some(&"S"));
        stos.pop();
        stos.pop();
        assert_eq!(stos.top(), Some(&"T"));
        assert!(!stos.is_empty());
        stos.pop();
        assert_eq!(stos.top(), Some(&"S"));
        stos.pop();
        assert!(stos.is_empty());
    }
}
