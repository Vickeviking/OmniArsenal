/***
 *  Doubly Linked List
 *  pop_front, pop_back, peek_front, peek_back,append, prepend & clear: O(1) time complexity
 *  available Iterators: into_iter, iter, rev_iter
 * 
 * Upsides:
 * - Low overhead allocation per item (but more than SLL)
 * - Item count is only limited by heap size
 * - Mutation while iterating is possible
 * - Implementation is more complex but fairly simple
 * - Inserts, deletes, and prepends remain efficient
 * - Efficient reversion
 * 
 * Downsides:
 * - Indexing still inefficient
 * - Nodes allocated on heap are not cache friendly
 * - An additional pointer per node is required
 * - Implementation is more complex
 */

use std::cell::{RefCell, Ref};
use std::rc::{Rc, Weak};
use std::fmt;

type Link<T> = Option<Rc<RefCell<Node<T>>>>; // Strong reference to a node
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>; // Weak reference to a node

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
    prev: WeakLink<T>, //weak because it's not an owner
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            data,
            next: None,
            prev: None,
        }))
    }
}

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: WeakLink<T>,
    pub len: u64,
    pub total_size_bytes: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new_empty() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            total_size_bytes: 0,
        }
    }

    pub fn prepend(&mut self, data: T) {
        let new_node = Node::new(data);
        match self.head.take() {
            Some(old_head) => {
                // Set the next node to the old head
                new_node.borrow_mut().next = Some(Rc::clone(&old_head));
                // Set the previous node of the old head to the new node
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
            },
            None => {
                // If the list is empty, the new node is both the head and the tail
                self.tail = Some(Rc::downgrade(&new_node));
            }
        }
        // Update the head to the new node
        self.head = Some(new_node);
        // Increment len and byte size
        self.len += 1;
        self.total_size_bytes += std::mem::size_of::<T>();
    }
    

    pub fn append(&mut self, data: T) {
        let new_node = Node::new(data);
        match self.tail.take() {
            Some(old_tail) => {
                // Upgrade the weak reference to a strong reference to modify the node
                let strong = old_tail.upgrade().unwrap();
                // Set the next node to the new node
                strong.borrow_mut().next = Some(Rc::clone(&new_node));
                // Set the previous node of the new node to the old tail
                new_node.borrow_mut().prev = Some(Rc::downgrade(&strong));
            },
            None => {
                // If the list is empty, the new node is both the head and the tail
                self.head = Some(Rc::clone(&new_node));
            }
        }
        // Update the tail to the new node
        self.tail = Some(Rc::downgrade(&new_node));
        // increment len and byte size
        self.len += 1;
        self.total_size_bytes += std::mem::size_of::<T>();
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // If the list is empty, return None
        self.head.take().map(|old_head| {
            // Set the head to the next node
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // Set the previous node of the new head to None
                    new_head.borrow_mut().prev.take();
                    // Set the head to the new head
                    self.head = Some(new_head);
                },
                None => {
                    // If the list is now empty, set the tail to None
                    self.tail.take();
                }
            }
            // Decrement len and byte size
            self.len -= 1;
            self.total_size_bytes -= std::mem::size_of::<T>();
            // Return the data of the old head
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // If the list is empty, return None
        self.tail.take().map(|old_tail| {
            // Upgrade the weak reference to a strong reference to modify the node
            let strong = old_tail.upgrade().unwrap();
            // Set the tail to the previous node
            match strong.borrow_mut().prev.take() {
                Some(new_tail) => {
                    // Upgrade the weak reference to a strong reference to modify the node
                    let strong = new_tail.upgrade().unwrap();
                    // Set the next node of the new tail to None
                    strong.borrow_mut().next.take();
                    // Set the tail to the new tail
                    self.tail = Some(new_tail);
                },
                None => {
                    // If the list is now empty, set the head to None
                    self.head.take();
                }
            }
            // Decrement len and byte size
            self.len -= 1;
            self.total_size_bytes -= std::mem::size_of::<T>();
            // Return the data of the old tail
            Rc::try_unwrap(strong).ok().unwrap().into_inner().data
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.data)
        })
    }
    
    pub fn clear(&mut self) {
        *self = Self::new_empty();
    }

    pub fn iter(&self) -> DoublyLinkedListIterator<T> {
        DoublyLinkedListIterator {
            current: self.head.clone(),
        }
    }

}

pub struct DoublyLinkedListIterator<T> {
    current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Default> Iterator for DoublyLinkedListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.clone().map(|current| {
            self.current = current.borrow().next.clone();
            std::mem::take(&mut current.borrow_mut().data)
        })
    }
}

impl<T: Default> DoubleEndedIterator for DoublyLinkedListIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.current.clone().map(|current| {
            self.current = current.borrow().prev.as_ref().and_then(|weak| weak.upgrade());
            std::mem::take(&mut current.borrow_mut().data)
        })
    }
}

impl<T: Default> IntoIterator for DoublyLinkedList<T> {
    type Item = T;
    type IntoIter = DoublyLinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        DoublyLinkedListIterator {
            current: self.head,
        }
    }
}

impl<T: fmt::Debug + Default> fmt::Debug for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = self.head.clone();
        let mut list_str = String::from("list = [");

        while let Some(node) = current {
            list_str.push_str(&format!("{:?}, ", node.borrow().data));
            current = node.borrow().next.clone();
        }

        list_str.push_str("]\n");
        list_str.push_str(&format!("length = {}, bytes: {}", self.len, self.total_size_bytes));

        write!(f, "{}", list_str)
    }
}

