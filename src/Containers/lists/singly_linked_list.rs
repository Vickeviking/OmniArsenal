use std::rc::Rc;
use std::cell::RefCell;

type SingleLink<T> = Option<Rc<RefCell<Node<T>>>>;

/// Represents a singly linked list data structure.
pub struct SinglyLinkedList<T> {
    head: SingleLink<T>,
    tail: SingleLink<T>,
    pub total_size_bytes: usize, // size in bytes
    pub node_count: u64, // length in nodes
}

/// Represents a node in a singly linked list.

#[derive(Clone)]
struct Node<T> {
    data: T,
    next: SingleLink<T>,
}


// Implementation for SinglyLinkedList methods
impl<T> SinglyLinkedList<T> {

    pub fn new_empty() -> Self {
        SinglyLinkedList {
            head: None,
            tail: None,
            total_size_bytes: 0,
            node_count: 0,
        }
    }

    // raises LIFO behaviour, SLL only pops front due to O(1) time complexity
    pub fn push(&mut self, data: T) {
        // create a node
        let new = Node::new(data);

        match self.head.take() {
            Some(old) => new.borrow_mut().next = Some(old.clone()),
            None => self.tail = Some(new.clone())
        }

        self.head = Some(new.clone());
        self.node_count += 1;
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

        // increment length(numeric?) and size(bytes)
        self.tail = Some(new); //tail point to new "actual" tail
        self.node_count += 1;
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
            self.node_count -= 1;
            self.total_size_bytes -= std::mem::size_of::<T>();
            Rc::try_unwrap(head)
                .ok()
                .expect("Something went wrong")
                .into_inner()
                .data
        })
    }

}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node{
            data: data,
            next: None,
        })) 
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