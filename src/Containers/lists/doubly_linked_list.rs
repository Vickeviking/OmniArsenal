use std::{ptr, fmt::Debug};

// Not idiomatic at all

// make Node<T> short for DoublyNode<T>
type Node<T> = DoublyNode<T>; // make Node<T>

#[derive(Debug)]
pub struct DoublyNode<T> {
    data: T,
    next: *mut Node<T>, 
    prev: *mut Node<T>, 
}

impl<T> Node<T> {
    fn new(data: T) -> *mut Self {
        Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }))
    }
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    size_in_bytes: usize,
    len: u64,
}

impl<T: Debug> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            size_in_bytes: 0,
            len: 0,
        }
    }

    pub fn append(&mut self, data: T) -> *mut Node<T> {
        let new_node = Node::new(data);
 
        unsafe {
            if self.head.is_null() {
                self.head = new_node;
                self.tail = new_node;
            } else {
                (*self.tail).next = new_node;
                (*new_node).prev = self.tail;
            }
            self.tail = new_node;
        }
        self.size_in_bytes += std::mem::size_of::<T>();
        self.len += 1;
        new_node
    }

    pub fn pop_front(&self, data: T) {
        unimplemented!()
    }

    pub fn pop_back(&self, data: T) {
        unimplemented!()
    }

    pub fn remove(&mut self, target: *mut Node<T>) {

        // we need to iterate through head find target, disect and chain togheter
        unsafe {
            // head or not
            if (*target).prev.is_null() { 
                self.head = (*target).next;
            } else {
                //we rewire to skip over this node
                (*(*target).prev).next = (*target).next;
            }

            if(*target).next.is_null() {
                self.tail = (*target).prev;
            } else {
                (*(*target).next).prev = (*target).prev;
            }

            // convert to drop, that has valid drop semantics
            Box::from_raw(target);
        }
    }

    fn prepend(&mut self, data: T) {
        unimplemented!()
    }

    pub fn debug_dump(&self) {
        unsafe {
            let mut current = self.head;
            print!("{:?} \n", self);
            print!("[ ");
            while !current.is_null() {
                print!("{:?} ", (*current).data);
                current = (*current).next;
            }
            print!("]\n");
        }
    }
}
