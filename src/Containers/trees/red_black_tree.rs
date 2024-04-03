use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::option::Option;
use std::fmt::{self, Debug};

use super::arena_red_black_tree::RedBlackTree;

type Node<K, V> = Option<Rc<RefCell<RbNode<K, V>>>>;
type WeakNode<K, V> = Option<Weak<RefCell<RbNode<K, V>>>>;

type NonNullNode<K, V> = Rc<RefCell<RbNode<K, V>>>;
type NonNullWeakNode<K, V> = Weak<RefCell<RbNode<K, V>>>;

pub trait Key: Ord + Default + Debug + Clone {}
impl<T: Ord + Default + Debug + Clone> Key for T {}
pub trait Value: Default + Debug + Clone {}
impl<T: Default + Debug + Clone> Value for T {}


enum Color {
    RED, 
    BLACK
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::RED => write!(f, "RED"),
            Color::BLACK => write!(f, "BLACK")
        }
    }
}

pub struct RbTree<K: Key, V: Value> {
    pub root: Node<K, V>,
    pub size: usize
}

pub struct RbNode<K: Key, V: Value> {
    pub val: V,
    pub key: K,
    color: Color,
    pub left: Node<K, V>,
    pub right: Node<K, V>,
    pub parent: WeakNode<K, V>,
    is_nill: bool,
    is_left_child: bool,
}




// --------  Some info about RB-Tree ----------------

/*
    Rules of a rb-tree

    1. Every node is either red or black 
    2. The root is black 
    3. Every Leaf (NIL) is black (If node is none, its black)
    4. If a node is red booth children is black 
    5. All paths from root to leaf contains same amount of blacks

*/


impl<K: Key, V: Value> RbTree<K, V> {

    // new function
    pub fn new() -> RbTree<K, V> {
        RbTree {
            root: Some(RbNode::<K, V>::new_nil()),
            size: 0
        }
    }

    // Rotations 
    /*  Rotation LEFT
             X                 Y
            / \               / \
           α   Y     ==>     X   γ
              / \           / \
             β   γ         α   β

    */

    pub fn rotate_left(&mut self, x: Node<K, V>) -> () {
        if let Some(mut unwrapped_x) = x {
            // make sure x is not a nil node 
            if unwrapped_x.borrow().is_nill {
                return;
            }
        // y takes x.right
            let mut y: Node<K, V> = unwrapped_x.borrow_mut().right.take();
        // x.right takes y.left, if x is something y is atleast a sentinel, (unwrappable)
            unwrapped_x.borrow_mut().right = y.as_ref().unwrap().borrow_mut().left.take();
        // parent chain B -> X if not NIL
            if unwrapped_x.borrow().right.is_some() && !unwrapped_x.borrow().right.as_ref().unwrap().borrow().is_nill {
                unwrapped_x.borrow().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&unwrapped_x));
            }
            // y parent = x parent | y.unwrap safe while x is some 
            y.as_ref().unwrap().borrow_mut().parent = unwrapped_x.borrow_mut().parent.take();
            // if y.parent is nil its now the new root
            y.as_ref().unwrap().borrow_mut().is_left_child = unwrapped_x.borrow().is_left_child;
            let is_nil = unwrapped_x.borrow().key == self.root.as_ref().unwrap().borrow().key;
            println!("is nill {}", is_nil);
            if is_nil {
                self.root = y.clone();
            } else if unwrapped_x.borrow().is_left_child {
                let y_parent_weak = y.as_ref().unwrap().borrow().parent.clone();
                let y_parent_strong = y_parent_weak.as_ref().unwrap().upgrade().unwrap();
                let mut y_parent = y_parent_strong.borrow_mut();
                y_parent.left = y.clone();
            } else {
                let y_parent_weak = y.as_ref().unwrap().borrow().parent.clone();
                let y_parent_strong = y_parent_weak.as_ref().unwrap().upgrade().unwrap();
                let mut y_parent = y_parent_strong.borrow_mut();
                y_parent.right = y.clone();

            }
            y.as_ref().unwrap().borrow_mut().left = Some(unwrapped_x);
            y.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y.as_ref().unwrap()));
        }
    }

    pub fn rotate_right(&mut self, x: Node<K, V>) -> () {
        if let Some(mut unwrapped_x) = x {
            // make sure x is not a nil node 
            if unwrapped_x.borrow().is_nill {
                return;
            }
            // y takes x.left
            let mut y: Node<K, V> = unwrapped_x.borrow_mut().left.take();
            // x.left takes y.right, if x is something y is atleast a sentinel, (unwrappable)
            unwrapped_x.borrow_mut().left = y.as_ref().unwrap().borrow_mut().right.take();
            // parent chain B -> X if not NIL
            if unwrapped_x.borrow().left.is_some() && !unwrapped_x.borrow().left.as_ref().unwrap().borrow().is_nill {
                unwrapped_x.borrow().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&unwrapped_x));
            }
            // y parent = x parent | y.unwrap safe while x is some 
            y.as_ref().unwrap().borrow_mut().parent = unwrapped_x.borrow_mut().parent.take();
            // if y.parent is nil its now the new root
            y.as_ref().unwrap().borrow_mut().is_left_child = !unwrapped_x.borrow().is_left_child;
            let is_nil = unwrapped_x.borrow().key == self.root.as_ref().unwrap().borrow().key;
            println!("is nill {}", is_nil);
            if is_nil {
                self.root = y.clone();
            } else if !unwrapped_x.borrow().is_left_child {
                let y_parent_weak = y.as_ref().unwrap().borrow().parent.clone();
                let y_parent_strong = y_parent_weak.as_ref().unwrap().upgrade().unwrap();
                let mut y_parent = y_parent_strong.borrow_mut();
                y_parent.right = y.clone();
            } else {
                let y_parent_weak = y.as_ref().unwrap().borrow().parent.clone();
                let y_parent_strong = y_parent_weak.as_ref().unwrap().upgrade().unwrap();
                let mut y_parent = y_parent_strong.borrow_mut();
                y_parent.left = y.clone();
            }
            y.as_ref().unwrap().borrow_mut().right = Some(unwrapped_x);
            y.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y.as_ref().unwrap()));
        }
    }


    // recolor


    // insertion
    pub fn insert(&mut self, key: K, value: V) {
        let z = RbNode::new(key, value);

        // if root is nill 
        if self.root.as_ref().unwrap().borrow().is_nill {
            self.root = Some(Rc::clone(&z));
            z.borrow_mut().color = Color::BLACK;
            z.borrow_mut().left = Some(RbNode::new_nil());
            z.borrow_mut().right = Some(RbNode::new_nil());
            self.size += 1;
        } else {
            self.insert_node(Some(z));
        }
    }

    fn insert_node(&mut self, z: Node<K, V>) {

        self.size += 1;
        let mut y = Some(RbNode::<K, V>::new_nil());
        let mut x = self.root.clone();
        while !Rc::clone(&x.as_ref().unwrap()).borrow().is_nill {
            y = Some(Rc::clone(&x.unwrap()));
        
            if z.as_ref().unwrap().borrow().key < y.as_ref().unwrap().borrow().key {
                // x = x.left
                let mut new_x = y.as_ref().unwrap().borrow_mut().left.take();
                x = new_x.take();
                y.as_ref().unwrap().borrow_mut().left = x.clone(); // update y's left child
            } else {
                // x = x.right
                let mut new_x = y.as_ref().unwrap().borrow_mut().right.take();
                x = new_x.take();
                y.as_ref().unwrap().borrow_mut().right = x.clone(); // update y's right child
            }
        }
        // y is now parent of z
        z.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y.as_ref().unwrap()));
        // if y is nill then the new node is root
        if y.as_ref().unwrap().borrow_mut().is_nill {
            self.root = z.clone();
        } else if z.as_ref().unwrap().borrow().key < y.as_ref().unwrap().borrow().key  {
            // z is left child 
            z.as_ref().unwrap().borrow_mut().is_left_child = true;
            y.as_ref().unwrap().borrow_mut().left = z.clone(); 
        } else {
            // z is right child 
            z.as_ref().unwrap().borrow_mut().is_left_child = false;
            y.as_ref().unwrap().borrow_mut().right = z.clone(); 
        }
        z.as_ref().unwrap().borrow_mut().left = Some(RbNode::new_nil());
        z.as_ref().unwrap().borrow_mut().right = Some(RbNode::new_nil());
        z.as_ref().unwrap().borrow_mut().color = Color::RED;
        // TODO: Call rb insert fixup
    } 


    // insertion fix 


    // delete

    // deletion fix 


    // debug 
    fn tree_printer_traverse_helper(sb: &mut String, padding: &str, pointer: &str, node: &Option<Rc<RefCell<RbNode<K, V>>>>) {
        if let Some(inner) = node {
            let node = inner.borrow();
            sb.push_str(padding);
            sb.push_str(pointer);
            // Check if node is NIL and print NIL, else print value and color
            if node.is_nill {
                sb.push_str("(NIL)");
            } else {
                sb.push_str(&format!("({:?},{:?})", node.val, node.color));
            }
            sb.push('\n');
    
            let padding_filler = if pointer == "└── " { "    " } else { "│   " };
            let padding = format!("{}{}", padding, padding_filler);
    
            let pointer_for_right = "└── ";
            let pointer_for_left = if node.right.is_some() { "├── " } else { "└── " };
    
            Self::tree_printer_traverse_helper(sb, &padding, pointer_for_left, &node.left);
            Self::tree_printer_traverse_helper(sb, &padding, pointer_for_right, &node.right);
        }
    }
}

impl<K: Key, V: Value> RbNode<K, V> {
    // sentinel node 
    pub fn new_nil() -> NonNullNode<K, V> {
        Rc::new(RefCell::new(RbNode {
            val: Default::default(),
            key: Default::default(),
            color: Color::BLACK,
            left: None,
            right: None,
            parent: None,
            is_nill: true,
            is_left_child: false
        }))
    }
    // node with key and Value 
    pub fn new(key: K, val: V) -> NonNullNode<K, V> {
        Rc::new(RefCell::new(RbNode {
            val: val,
            key: key,
            color: Color::RED,
            left: None,
            right: None,
            parent: None,
            is_nill: false,
            is_left_child: false
        }))
    }

    pub fn new_extended(key: K, val: V, color: Color, left: Node<K, V>, right: Node<K, V>, parent: WeakNode<K, V>, is_nill: bool, is_left_child: bool) -> NonNullNode<K, V> {
        Rc::new(RefCell::new(RbNode {
            val: val,
            key: key,
            color: color,
            left: left,
            right: right,
            parent: parent,
            is_nill: is_nill,
            is_left_child: is_left_child
        }))
    }

    pub fn print_information(&self) {
        // print information about the node its parent and children
        let mut parent_value: V = Default::default();
        if let Some(parent_weak) = &self.parent {
            if let Some(parent) = parent_weak.upgrade() {
                parent_value = parent.borrow().val.clone();
            }
        }

        let mut left_value: V = Default::default();
        if let Some(left) = &self.left {
            left_value = left.borrow().val.clone();
        }

        let mut right_value: V = Default::default();
        if let Some(right) = &self.right {
            right_value = right.borrow().val.clone();
        }

        println!("Key: {:?}, Value: {:?}, Color: {:?}, Parent: {:?}, Left: {:?}, Right: {:?}", self.key, self.val, self.color, parent_value, left_value, right_value);

    }

    
}

impl<K: Key, V: Value> fmt::Debug for RbTree<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sb = String::new();
        Self::tree_printer_traverse_helper(&mut sb, "", "", &self.root);
        write!(f, "{}", sb)
    }
}

// testing area 

#[cfg(test)]
mod tests {
    use super::*;


}