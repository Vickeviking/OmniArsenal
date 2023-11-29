use std::rc::{Rc, Weak};
use std::cell::RefCell;

/*

1 - - - - - 4        
1 - - - 3 - 4 
1 - 2 - 3 - 4 
1 - 2 - 3 - 4
*/

// start tower must be of max height
// end tower must be of max height
// max height changes explicitly or implicitly to keep log(n) insertions
// a height of new node is determined by a coin flip, if heads, increase height

#[derive(Debug)]


struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
    down: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct SkipList<T> {
    max_height: usize,
    start_tower: Option<Rc<RefCell<Node<T>>>>, // to be able to start at top, this is first el
    end_tower: Option<Rc<RefCell<Node<T>>>>,  // we need to be able to connect all nodes to something
}
