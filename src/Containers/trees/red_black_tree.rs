use std::{
    rc::{Rc, Weak}, 
    cell::RefCell,
    };

/***
 * Red Black Tree
 * 
 * auto sorting tree, 
 * following 3 rules
 * 1. every node is either red or black
 * 2. root is black
 * 3. new node is red
 * 4. every path from root to null node has same number of black nodes
 * 5. no red node has red child
 * 6. null node is black
 * 
 * Pros:
 *  1. auto sorting
 *  2. fast
 *  3. balanced
 *  4. fast insert and delete
 *  5. fast search
 * 
 * Cons:
 *  1. slow than hash table
 *  2. complex
 */
 type strongLink<k, v> = Option<Rc<RefCell<Node<k, v>>>>;
 type weakLink<k, v> = Option<Weak<RefCell<Node<k, v>>>>;

 pub struct Node<k, v> {
    key: k,
    value: v,
    is_black: bool,
    is_left_child: bool,
    left: strongLink<k, v>,
    right: strongLink<k, v>,
    parent: weakLink<k, v>,
 }

 impl<k, v> Node<k, v> {
     pub fn new(key: k, value: v) -> Self {
         Node {
             key,
             value,
             is_black: false,
             is_left_child: false,
             left: None,
             right: None,
             parent: None,
         }
     }
     
 }

 pub struct RedBlackTree<k, v> {
    root: strongLink<k, v>,
    size: usize,
 }

 impl <k: std::cmp::PartialOrd, v> RedBlackTree<k, v> {
     pub fn new() -> Self {
         RedBlackTree {
             root: None,
             size: 0,
         }
     }

     pub fn add(mut self, key:k, value:v) {
        let mut node = Node::new(key, value);
        if self.root.is_none() {
            node.is_black = true; // rule 2
            let size = std::mem::size_of_val(&node);// size of node(key, value etcetera)
            let node = Rc::new(RefCell::new(node));
            self.root = Some(node);
            self.size += size;
            return;
        }
        let size = std::mem::size_of_val(&node);// size of node(key, value etcetera)
        let node = Some(Rc::new(RefCell::new(node)));
        let weak_root = Some(Rc::downgrade(&self.root.as_ref().unwrap()));
        Self::add_recursive(&weak_root, node);
        self.size += size;
     }
     
     fn add_recursive(parent: &weakLink<k, v>, node: strongLink<k, v>) {
        let unwrapped_parent = parent.as_ref().unwrap().upgrade().unwrap();
        if node.as_ref().unwrap().borrow().key > unwrapped_parent.borrow().key { // right
            if unwrapped_parent.borrow().right.is_none() { //base case: no right child
                node.as_ref().unwrap().borrow_mut().is_left_child = false; // its a right child
                node.as_ref().unwrap().borrow_mut().parent = parent.clone(); // set parent link (weak)
                unwrapped_parent.borrow_mut().right = node.clone(); // set down link (strong)
            } else { // recursive case: right child exists
                let next_parent_as_weak = 
                    Some(Rc::downgrade(&unwrapped_parent.borrow().right.as_ref().unwrap()));
                Self::add_recursive(&next_parent_as_weak, node.clone());
            }
        } else { // left
            if unwrapped_parent.borrow().left.is_none() { //base case: no left child
                node.as_ref().unwrap().borrow_mut().is_left_child = true; // its a left child
                node.as_ref().unwrap().borrow_mut().parent = parent.clone(); // set parent link (weak)
                unwrapped_parent.borrow_mut().left = node.clone(); // set down link (strong)
            } else { // recursive case: left child exists
                let next_parent_as_weak = 
                    Some(Rc::downgrade(&unwrapped_parent.borrow().left.as_ref().unwrap()));
                Self::add_recursive(&next_parent_as_weak, node.clone()); 
            }
        }
        // check color of node, if violated, fix it. Rotation or recoloring!
        Self::check_color(node.clone()); 
    }
      
    fn check_color(node: strongLink<k, v>) {
        //not yet implemented
    }
    
 }
