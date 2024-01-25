

//uses the following methods:
// - append O(1)
// - pop O(1)
// - peek O(1)

struct Node<T> {
    value: Option<T>, 
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            value: Some(data),
            next: None
        }
    } 
}


pub struct Stack<T> {
    root: Option<Box<Node<T>>>
}

impl<T> Stack<T> {

    pub fn new_empty() -> Self {
        Self {
            root: None
        }
    }

    pub fn new(data: T) -> Self {
        Self {
            root: Some(Box::new(Node::new(data)))
        }
    }

    pub fn append(&mut self, data: T) {
        let mut new_node = Node::new(data);
        new_node.next = self.root.take();
        self.root = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.root.is_some() {
            let mut old_root = self.root.take().unwrap();
            self.root = old_root.next.take();
            old_root.value
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.root.is_some() {
            self.root.as_ref().unwrap().value.as_ref()
        } else {
            None
        }
    }
}

impl<T> Iterator for Stack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_stack = Stack::new_empty();
        for i in iter {
            new_stack.append(i);
        }
        new_stack
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let s: Stack<i32> = Stack::new_empty();
        assert!(s.root.is_none());
    }

    #[test]
    fn test_new() {
        let s = Stack::new(1);
        assert!(s.root.is_some());
        assert_eq!(s.root.unwrap().value, Some(1));
    }

    #[test]
    fn test_append() {
        let mut s = Stack::new(1);
        s.append(2);
        assert_eq!(*s.peek().unwrap(), 2);
    }

    #[test]
    fn test_pop() {
        let mut s = Stack::new(1);
        s.append(2);
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut s = Stack::new(1);
        s.append(2);
        assert_eq!(*s.peek().unwrap(), 2);
        s.pop();
        assert_eq!(*s.peek().unwrap(), 1);
    }

    #[test]
    fn test_iterator() {
        let mut s = Stack::new(1);
        s.append(2);
        s.append(3);
        let mut iter = s.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_from_iterator() {
        let s = Stack::from_iter(vec![1, 2, 3]);
        assert_eq!(s.peek(), Some(&3));
    }

    #[test]
fn test_comprehensive() {
    // Create a new stack with a single element
    let mut s = Stack::new(1);
    assert_eq!(*s.peek().unwrap(), 1);

    // Append several elements
    for i in 2..=10 {
        s.append(i);
        assert_eq!(*s.peek().unwrap(), i);
    }

    // Pop all elements and check that they are in the correct order
    for i in (1..=10).rev() {
        assert_eq!(s.pop(), Some(i));
    }

    // Check that the stack is now empty
    assert_eq!(s.pop(), None);

    // Create a new stack from an iterator
    let s = Stack::from_iter(11..=20);

    // Check that the top of the stack is correct
    assert_eq!(s.peek(), Some(&20));

    // Convert the stack into an iterator and check that it yields the correct elements
    let mut iter = s.into_iter();
    for i in (11..=20).rev() {
        assert_eq!(iter.next(), Some(i));
    }

    // Check that the iterator is now exhausted
    assert_eq!(iter.next(), None);
}

}