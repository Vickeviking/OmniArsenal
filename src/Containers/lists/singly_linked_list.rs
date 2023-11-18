use std::rc::Rc;
use std::cell::RefCell;

type SingleLink<T> = Option<Rc<RefCell<Node<T>>>>;

/// Represents a singly linked list data structure.
pub struct SinglyLinkedList<T> {
    head: SingleLink<T>,
    tail: SingleLink<T>,
    size: usize, // size in bytes
    pub length: u64, // length in nodes
}

/// Represents a node in a singly linked list.
struct Node<T> {
    data: T,
    next: SingleLink<T>,
}


// Implementation for SinglyLinkedList methods
impl<T> SinglyLinkedList<T> {

    pub fn new() -> Self {
        SinglyLinkedList {
            head: None,
            tail: None,
            size: 0,
            length: 0,
        }
    }

    // raises LIFO behaviour, SLL only pops front due to O(1) time complexity
    pub fn push_front(&mut self, data: T) {
        // create a node
        let node = Some(Rc::new(RefCell::new(Node {
            data: data,
            next: None,
        })));
        // match to se if empty or not , if empty set head and tail to node, otherwise set head to node and node.next to old head
        match self.head.take() {
            Some(old_head) => {
                node.as_ref().unwrap().borrow_mut().next = Some(old_head.clone());
                self.head = node; 
            }
            None => {
                self.head = node.clone();
                self.tail = node.clone();
            }
        }
        // increment length(numeric?) and size(bytes)
        self.length += 1;
        self.size += std::mem::size_of::<T>();
    }

    // raises FIFO behaviour, SLL only pushes back due to O(1) time complexity
    pub fn push_back(&mut self, data: T) {
        // create a node
        let node = Some(Rc::new(RefCell::new(Node {
            data: data,
            next: None,
        })));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = node.clone();
                self.tail = node;
            }
            None => {
                self.head = node.clone();
                self.tail = node.clone();
            }
        }

        // increment length(numeric?) and size(bytes)
        self.length += 1;
        self.size += std::mem::size_of::<T>();
    }

    // Pops front , use LIFO or FIFO behaviour depending on push method used
    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }

}



/*TODO: 
    Functions for Singly Linked List:
    peek: Get a reference to the first element without removing it.
    peek_mut: Get a mutable reference to the first element without removing it.
    into_iter: Consume the list, returning an iterator.
    iter: Create an iterator over the elements without consuming the list.
    iter_mut: Create a mutable iterator over the elements without consuming the list.
    rev: Return an iterator over the elements in reverse order.
    rev_mut: Return a mutable iterator over the elements in reverse order.
    len: Get the number of elements in the list.
    is_empty: Check if the list is empty.
    clear: Remove all elements from the list.

    Iterator for Singly Linked List:
    Implement the Iterator trait for your linked list.

    Traits: Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash:
    Implement each of these traits for both SinglyLinkedList and Node. Make sure to consider the specific requirements of each trait.
    
    IntoIterator, Iterator, and Reverse Iterator:
    Implement the IntoIterator trait for your linked list, allowing it to be consumed in a for loop.
    Implement the Iterator trait for your linked list, defining how iteration works.
    Implement a reverse iterator for your linked list, allowing you to iterate in reverse order.

    Tests:
    Write comprehensive tests for each function and trait implementation. Ensure that your linked list behaves as expected in different scenarios.
*/