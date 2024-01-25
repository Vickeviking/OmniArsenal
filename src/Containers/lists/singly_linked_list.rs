/***
 * Singly Linked List
 * prepend, append, pop, peek, is_empy & clear are O(1) time complexity
 * FIFO or LIFO behaviour depending on push method used
 * available Iterators: into_iter, iter. No rev_iter while singly linked
 * 
 * Upsides: 
 * - Low overhead allocation per item
 * - Item count is only limited by heap size
 * - Mutation while iterating is possible
 * - A direction is strictly enforced - there is no going back
 * - Implementation is simple
 * - Efficient append, prepend, delete, and insert operations compared to arrays
 * 
 * Downsides:
 * - Indexing is inefficient, since it must iterate over all nodes
 * - Iteration in general involves a lot of jumping around in heap which takes time and is not cache friendly
 * - Reversing list is very inefficient

*/

use std::rc::Rc;
use std::cell::RefCell;

type SingleLink<T> = Option<Rc<RefCell<Node<T>>>>;

/// Represents a singly linked list data structure.
pub struct SinglyLinkedList<T: Clone> {
    head: SingleLink<T>,
    tail: SingleLink<T>,
    pub total_size_bytes: usize, // size in bytes
    pub len: u64, // length in nodes
}

pub struct SinglyLinkedListIterator<T: Clone> {
    current: SingleLink<T>,
}

/// Represents a node in a singly linked list.
#[derive(Clone)]
struct Node<T: Clone> {
    data: T,
    next: SingleLink<T>,
}


// Implementation for SinglyLinkedList methods
impl<T: Clone> SinglyLinkedList<T> {

    pub fn new_empty() -> Self {
        SinglyLinkedList {
            head: None,
            tail: None,
            total_size_bytes: 0,
            len: 0,
        }
    }

    // raises LIFO behaviour, SLL only pops front due to O(1) time complexity
    pub fn prepend(&mut self, data: T) {
        // create a node
        let new = Node::new(data);

        match self.head.take() {
            Some(old) => new.borrow_mut().next = Some(old.clone()),
            None => self.tail = Some(new.clone())
        }

        // increment length(u64) and size(bytes)
        self.head = Some(new.clone());
        self.len += 1;
        self.total_size_bytes += std::mem::size_of::<T>();
    }

    // raises FIFO behaviour, SLL only pushes back due to O(1) time complexity
    pub fn append(&mut self, data: T) {
        // create a node
        let new = Node::new(data);

        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        }

        // increment length(u64) and size(bytes)
        self.tail = Some(new); //tail point to new "actual" tail
        self.len += 1;
        self.total_size_bytes += std::mem::size_of::<T>();
    }

    // Pops front , use LIFO or FIFO behaviour depending on push method used
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.len -= 1;
            self.total_size_bytes -= std::mem::size_of::<T>();
            Rc::try_unwrap(head)
                .ok()
                .expect("Something went wrong")
                .into_inner()
                .data
        })
    }

    pub fn peek(&self) -> Option<T> { //if some, return cloned data, else return none
        self.head.as_ref().map(|head| {
            head.borrow().data.clone()
        })
    }

    pub fn peek_tail(&self) -> Option<T> { //if some, return cloned data, else return none
        self.tail.as_ref().map(|tail| {
            tail.borrow().data.clone()
        })
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn clear(&mut self) {
        *self = Self::new_empty();
    }


}

impl<T: Clone> Node<T> {
    fn new(data: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node{
            data,
            next: None,
        })) 
    }
}

// Trait implementations

//Iterator, we need to define the associated type Item and the next() method
impl<T: Clone> Iterator for SinglyLinkedListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.data.clone());
                current.next.clone()
            },
            None => None,
        };
        result
    }
    
}

//IntoIterator, we need to define the associated type Item and the into_iter() method
impl<T: Clone> IntoIterator for SinglyLinkedList<T> {
    type Item = T;
    type IntoIter = SinglyLinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        SinglyLinkedListIterator {
            current: self.head,
        }
    }
}

//formatting trait
use std::fmt;
impl<T: Clone + fmt::Debug> fmt::Debug for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SinglyLinkedList")
            .field("total_size_bytes", &self.total_size_bytes)
            .field("node_count", &self.len)
            .finish()
    }
}



// **********  TESTS ********** //
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepend_test() {
        // Your push test implementation goes here
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);

        // Add assertions to test the behavior
        assert_eq!(list.len, 1);
        assert_eq!(list.total_size_bytes, std::mem::size_of::<i32>());
    }

    #[test]
    fn append_test() {
        // Your append test implementation goes here
        let mut list = SinglyLinkedList::new_empty();
        list.append(42);

        // Add assertions to test the behavior
        assert_eq!(list.len, 1);
        assert_eq!(list.total_size_bytes, std::mem::size_of::<i32>());
    }

    #[test]

    fn pop_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);

        // Add assertions to test the behavior
        assert_eq!(list.pop(), Some(44));
        assert_eq!(list.pop(), Some(43));
        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn peek_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);

        // Add assertions to test the behavior
        assert_eq!(list.peek(), Some(42));
    }

    #[test]
    fn peek_tail_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);

        // Add assertions to test the behavior
        assert_eq!(list.peek_tail(), Some(42));
    }

    #[test]

    fn is_empty_test() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);

        // Add assertions to test the behavior
        assert_eq!(list.is_empty(), false);

        let list2: SinglyLinkedList<i32>   = SinglyLinkedList::new_empty();
        assert_eq!(list2.is_empty(), true);
    }

    #[test]
    fn clear_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);

        // Add assertions to test the behavior
        assert_eq!(list.is_empty(), false);
        list.clear();
        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn iterator_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(44));
        assert_eq!(iter.next(), Some(43));
        assert_eq!(iter.next(), Some(42));
        assert_eq!(iter.next(), None);

        let mut list2 = SinglyLinkedList::new_empty();
        list2.prepend(42);
        list2.prepend(43);
        list2.prepend(44);

        for (i, val) in list2.into_iter().enumerate() {
            assert_eq!(val, 44 - i as i32);
        }
    }

    #[test]
    fn formatting_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);
        assert_eq!(format!("{:?}", list), "SinglyLinkedList { total_size_bytes: 12, node_count: 3 }");
    }

    #[test]
    fn big_list_test() {
        let mut list = SinglyLinkedList::new_empty();
        for i in 0..1000 {
            list.prepend(i);
        }
        assert_eq!(list.len, 1000);
        assert_eq!(list.total_size_bytes, 1000 * std::mem::size_of::<i32>());

        for i in 0..1000 {
            assert_eq!(list.pop(), Some(999 - i));
        }

        assert_eq!(list.len, 0);

        for i in 0..1000 {
            list.append(i);
        }
        assert_eq!(list.len, 1000);

        for i in 0..1000 {
            assert_eq!(list.pop(), Some(i));
        }

        assert_eq!(list.len, 0);

        list.prepend(42);
        list.prepend(43);
        list.prepend(44);
        assert!(list.is_empty() == false);
        assert!(list.peek() == Some(44));
        assert!(list.peek_tail() == Some(42));
        assert!(list.pop() == Some(44));
        assert!(list.peek() == Some(43));
        assert!(list.peek_tail() == Some(42));
        assert!(list.pop() == Some(43));
        assert!(list.peek() == Some(42));
        assert!(list.peek_tail() == Some(42));
        assert!(list.pop() == Some(42));
        assert!(list.peek() == None);
        assert!(list.peek_tail() == None);
        assert!(list.is_empty() == true);
        assert!(list.pop() == None);

        list.prepend(42);
        list.prepend(43);
        list.prepend(44);
        assert!(list.is_empty() == false);
        list.clear();
        assert!(list.is_empty() == true);

    }

    
}