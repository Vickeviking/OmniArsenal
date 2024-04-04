/***
 * Red-Black Tree
 * Self-balancing Binary Search Tree
 * Properties:
 * - Every node is either red or black
 * - Root is black
 * - Red nodes have black children
 * - All paths from a node to its descendant null pointers have the same number of black nodes
 * 
 * Time Complexity:
 * - O(log n) for search, insert, delete
 * - O(n) for traversal
 * 
 * Upsides:
 * - Guaranteed logarithmic time for operations
 * - Efficient and balanced structure
 * - Self-balancing property maintains tree balance during insertions and deletions
 * - Suitable for large datasets with unpredictable distribution
 * 
 * Downsides:
 * - More complex implementation compared to simple binary search trees
 * - Slightly slower in practice due to additional bookkeeping for maintaining balance
 * - Requires extra space to meta data for each node such as color, parent, etc.
 * - May be less cache-friendly than arena allocated implementations
 */

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


#[derive(Clone)]
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
            } else if unwrapped_x.borrow().right.as_ref().unwrap().borrow().is_nill {
                return;
            }
        // y takes x.right
            let mut y: Node<K, V> = unwrapped_x.borrow_mut().right.take();
        // x.right takes y.left, if x is something y is atleast a sentinel, (unwrappable)
            unwrapped_x.borrow_mut().right = y.as_ref().unwrap().borrow_mut().left.take();
        // parent chain B -> X if not NIL
            if unwrapped_x.borrow().right.is_some() && !unwrapped_x.borrow().right.as_ref().unwrap().borrow().is_nill {
                unwrapped_x.borrow().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&unwrapped_x));
                unwrapped_x.borrow().right.as_ref().unwrap().borrow_mut().is_left_child = false;
            }
            // y parent = x parent | y.unwrap safe while x is some 
            y.as_ref().unwrap().borrow_mut().parent = unwrapped_x.borrow_mut().parent.take();
            // if y.parent is nil its now the new root
            y.as_ref().unwrap().borrow_mut().is_left_child = unwrapped_x.borrow().is_left_child;
            let is_nil = unwrapped_x.borrow().key == self.root.as_ref().unwrap().borrow().key;
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
            unwrapped_x.borrow_mut().is_left_child = true;
            y.as_ref().unwrap().borrow_mut().left = Some(unwrapped_x);
            y.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y.as_ref().unwrap()));
        }
    }

    /*  Rotation RIGHT
             X                 Y
            / \               / \
           Y   γ     ==>     α   X
          / \                   / \
         α  β                  β   γ

    */
    pub fn rotate_right(&mut self, x: Node<K, V>) -> () {
        if let Some(unwrapped_x) = x {
            // make sure x is not a nil node 
            if unwrapped_x.borrow().is_nill {
                return;
            } else if unwrapped_x.borrow().left.as_ref().unwrap().borrow().is_nill {
                return;
            }
            // y takes x.left
            let y: Node<K, V> = unwrapped_x.borrow_mut().left.take();
            // x.left takes y.right, if x is something y is atleast a sentinel, (unwrappable)
            unwrapped_x.borrow_mut().left = y.as_ref().unwrap().borrow_mut().right.take();
            // parent chain B -> X if not NIL
            if unwrapped_x.borrow().left.is_some() && !unwrapped_x.borrow().left.as_ref().unwrap().borrow().is_nill {
                unwrapped_x.borrow().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&unwrapped_x));
                unwrapped_x.borrow().left.as_ref().unwrap().borrow_mut().is_left_child = true;
            }
            // y parent = x parent | y.unwrap safe while x is some 
            y.as_ref().unwrap().borrow_mut().parent = unwrapped_x.borrow_mut().parent.take();
            // if y.parent is nil its now the new root
            y.as_ref().unwrap().borrow_mut().is_left_child = unwrapped_x.borrow().is_left_child;
            let is_nil = unwrapped_x.borrow().key == self.root.as_ref().unwrap().borrow().key;
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
            unwrapped_x.borrow_mut().is_left_child = false;
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

    // ###########     traversals       ################



    // Inorder Traversal
    pub fn inorder_key_traversal(&self) -> Vec<K> {
        let mut result: Vec<(K,V)> = Vec::new();
        Self::inorder_traversal_helper(&self.root, &mut result);
        // extract into vec of keys
        let mut key_result: Vec<K> = Vec::new();
        for (key, _) in result {
            key_result.push(key);
        }
        key_result
    }
    pub fn inorder_val_traversal(&self) -> Vec<V> {
        let mut result: Vec<(K, V)> = Vec::new();
        Self::inorder_traversal_helper(&self.root, &mut result);
        let mut value_result: Vec<V> = Vec::new();
        for (_, value) in result {
            value_result.push(value);
        }
        value_result
    }
    pub fn inorder_traversal(&self) -> Vec<(K, V)> {
        let mut result: Vec<(K, V)> = Vec::new();
        Self::inorder_traversal_helper(&self.root, &mut result);
        result
    }
    
    fn inorder_traversal_helper(node: &Node<K, V>, result: &mut Vec<(K, V)>) {
        if let Some(inner) = node {
            let node = inner.borrow();
            if node.is_nill {
                return;
            }
            Self::inorder_traversal_helper(&node.left, result);
            result.push((node.key.clone(), node.val.clone()));
            Self::inorder_traversal_helper(&node.right, result);
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

    
    // Formated as: 
    //   (key, value, parent_value, left_value, right_value, is_nill, is_left_child)
    pub fn get_debug_info_vec(&self) -> Vec<(K, V, V, V, V, bool, bool)> {
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

        vec![(self.key.clone(), self.val.clone(), parent_value, left_value, right_value, self.is_nill, self.is_left_child)]
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

    // ==================================
    // ==       LEFT ROTATION          ==
    // ==================================
    #[test]
    fn test_rotate_left1() {

        // ROTATE LEFT WHERE X IS ROOT
        let mut tree: RbTree<i32, char> = RbTree::new();
        //prepare for rotation left
        tree.insert(10, 'X');
        tree.insert(15, 'B');
        tree.insert(5, 'Y');
        tree.insert(12, 'C');
        tree.insert(20, 'A');
        // rotate
        let x = tree.root.clone();
        tree.rotate_left(x);

        //(key, value, parent_value, left_value, right_value, is_nill, is_left_child)
        let b_info = tree.root.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(15, 'B', char::default(), 'X', 'A', false, false)];
        assert_eq!(b_info, vec);

        let x_info = tree.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(10, 'X', 'B', 'Y', 'C', false, true)];
        assert_eq!(x_info, vec);

        let y_info = tree.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(5, 'Y', 'X', char::default(), char::default(), false, true)];
        assert_eq!(y_info, vec);

        let c_info = tree.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(12, 'C', 'X', char::default(), char::default(), false, false)];
        assert_eq!(c_info, vec);

        let a_info = tree.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(20, 'A', 'B', char::default(), char::default(), false, false)];
        assert_eq!(a_info, vec);

    }

    #[test]
    fn test_rotate_left2() {
        // ROTATE LEFT WHERE X IS NOT ROOT
        let mut tree: RbTree<i32, char> = RbTree::new();
        //prepare for rotation left
        tree.insert(2, 'U');
        tree.insert(10, 'X');
        tree.insert(15, 'B');
        tree.insert(5, 'Y');
        tree.insert(12, 'C');
        tree.insert(20, 'A');
        tree.insert(3, 'T');
        tree.insert(7, 'P');
        tree.insert(11, 'N');
        tree.insert(13, 'M');
        tree.insert(17, 'Q');
        tree.insert(21, 'E');

        // rotate
        let x = tree.root.as_ref().unwrap().borrow().right.clone();
        tree.rotate_left(x);

        //(key, value, parent_value, left_value, right_value, is_nill, is_left_child)
        // lets validate the tree

        let u_info = tree.root.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(2, 'U', char::default(), char::default(), 'B', false, false)];
        assert_eq!(u_info, vec);

        let b_info = tree.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(15, 'B', 'U', 'X', 'A', false, false)];
        assert_eq!(b_info, vec);

        let x_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(10, 'X', 'B', 'Y', 'C', false, true)];
        assert_eq!(x_info, vec);

        let y_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(5, 'Y', 'X', 'T', 'P', false, true)];
        assert_eq!(y_info, vec);

        let c_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(12, 'C', 'X', 'N', 'M', false, false)];
        assert_eq!(c_info, vec);

        let a_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(20, 'A', 'B', 'Q', 'E', false, false)];
        assert_eq!(a_info, vec);

        let t_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(3, 'T', 'Y', char::default(), char::default(), false, true)];
        assert_eq!(t_info, vec);

        let p_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(7, 'P', 'Y', char::default(), char::default(), false, false)];
        assert_eq!(p_info, vec);

        let n_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(11, 'N', 'C', char::default(), char::default(), false, true)];
        assert_eq!(n_info, vec);

        let m_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(13, 'M', 'C', char::default(), char::default(), false, false)];
        assert_eq!(m_info, vec);

        let q_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(17, 'Q', 'A', char::default(), char::default(), false, true)];
        assert_eq!(q_info, vec);

        let e_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(21, 'E', 'A', char::default(), char::default(), false, false)];
        assert_eq!(e_info, vec);

    }

    #[test]
    fn test_rotate_left_3() {
        // Test if we only have a single node in the tree
        let mut tree: RbTree<i32, char> = RbTree::new();
        //prepare for rotation left
        tree.insert(10, 'X');
        // rotate
        let x = tree.root.clone();
        tree.rotate_left(x);
        //(key, value, parent_value, left_value, right_value, is_nill, is_left_child)
        let x_info = tree.root.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(10, 'X', char::default(), char::default(), char::default(), false, false)];
        assert_eq!(x_info, vec);
    }

    #[test]
    fn test_rotate_left_4() {
        // Test if we only have a single node in the tree and a right node
        let mut tree: RbTree<i32, char> = RbTree::new();
        //prepare for rotation left
        tree.insert(10, 'X');
        tree.insert(15, 'B');
        // rotate
        let x = tree.root.clone();
        tree.rotate_left(x);
        //(key, value, parent_value, left_value, right_value, is_nill, is_left_child)
        let b_info = tree.root.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(15, 'B', char::default(), 'X', char::default(), false, false)];
        assert_eq!(b_info, vec);

        let x_info = tree.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(10, 'X', 'B', char::default(), char::default(), false, true)];
        assert_eq!(x_info, vec);


        // test if we only have a single node in the tree and a left node
        let mut tree: RbTree<i32, char> = RbTree::new();
        //prepare for rotation left
        tree.insert(10, 'X');
        tree.insert(5, 'B');
        // rotate
        let x = tree.root.clone();
        tree.rotate_left(x);
        //(key, value, parent_value, left_value, right_value, is_nill, is_left_child)
        let x2_info = tree.root.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(10, 'X', char::default(), 'B', char::default(), false, false)];
        assert_eq!(x2_info, vec);

        let b2_info = tree.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(5, 'B', 'X', char::default(), char::default(), false, true)];
        assert_eq!(b2_info, vec);

        // test if we only have a single node in the tree and a left & right node
        let mut tree: RbTree<i32, char> = RbTree::new();
        //prepare for rotation left
        tree.insert(10, 'X');
        tree.insert(5, 'B');
        tree.insert(15, 'A');
        // rotate
        let x = tree.root.clone();
        tree.rotate_left(x);
        //(key, value, parent_value, left_value, right_value, is_nill, is_left_child)
        let a_info = tree.root.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(15, 'A', char::default(), 'X', char::default(), false, false)];
        assert_eq!(a_info, vec);

        let x_info = tree.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(10, 'X', 'A', 'B', char::default(), false, true)];
        assert_eq!(x_info, vec);

        let b_info = tree.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(5, 'B', 'X', char::default(), char::default(), false, true)];
        assert_eq!(b_info, vec);

    }


    // ==================================
    // ==      Right ROTATION          ==
    // ==================================
    #[test]
    fn test_rotate_right1() {
        let mut tree: RbTree<i32, char> = RbTree::new();

        //prepare for rotation right
        tree.insert(15, 'B');
        tree.insert(10, 'X');
        tree.insert(20, 'A');
        tree.insert(5, 'Y');
        tree.insert(12, 'C');

        // rotate
        let x = tree.root.clone();
        tree.rotate_right(x);

        //(key, value, parent_value, left_value, right_value, is_nill, is_left_child)
        let x_info = tree.root.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(10, 'X', char::default(), 'Y', 'B', false, false)];
        assert_eq!(x_info, vec);

        let y_info = tree.root.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(5, 'Y', 'X', char::default(), char::default(), false, true)];
        assert_eq!(y_info, vec);

        let b_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(15, 'B', 'X', 'C', 'A', false, false)];
        assert_eq!(b_info, vec);

        let c_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(12, 'C', 'B', char::default(), char::default(), false, true)];
        assert_eq!(c_info, vec);

        let a_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(20, 'A', 'B', char::default(), char::default(), false, false)];
        assert_eq!(a_info, vec);

    }

    #[test]
    fn test_rotate_right2() {
        let mut tree: RbTree<i32, char> = RbTree::new();
        //prepare for rotation right
        tree.insert(2, 'U');
        tree.insert(15, 'B');
        tree.insert(10, 'X');
        tree.insert(20, 'A');
        tree.insert(5, 'Y');
        tree.insert(12, 'C');
        tree.insert(17, 'Q');
        tree.insert(21, 'E');
        tree.insert(3, 'T');
        tree.insert(7, 'P');
        tree.insert(11, 'N');
        tree.insert(13, 'M');

        // rotate
        let x = tree.root.as_ref().unwrap().borrow().right.clone();
        tree.rotate_right(x);

        //(key, value, parent_value, left_value, right_value, is_nill, is_left_child)
        // lets validate the tree
        let u_info = tree.root.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(2, 'U', char::default(), char::default(), 'X', false, false)];
        assert_eq!(u_info, vec);

        let x_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(10, 'X', 'U', 'Y', 'B', false, false)];
        assert_eq!(x_info, vec);

        let y_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(5, 'Y', 'X', 'T', 'P', false, true)];
        assert_eq!(y_info, vec);

        let b_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(15, 'B', 'X', 'C', 'A', false, false)];
        assert_eq!(b_info, vec);

        let c_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(12, 'C', 'B', 'N', 'M', false, true)];
        assert_eq!(c_info, vec);

        let a_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(20, 'A', 'B', 'Q', 'E', false, false)];
        assert_eq!(a_info, vec);

        let t_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(3, 'T', 'Y', char::default(), char::default(), false, true)];
        assert_eq!(t_info, vec);

        let p_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(7, 'P', 'Y', char::default(), char::default(), false, false)];
        assert_eq!(p_info, vec);

        let n_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(11, 'N', 'C', char::default(), char::default(), false, true)];
        assert_eq!(n_info, vec);

        let m_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow()
                                                            .right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(13, 'M', 'C', char::default(), char::default(), false, false)];
        assert_eq!(m_info, vec);

        let q_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                            .right.as_ref().unwrap().borrow()
                                                          .left.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(17, 'Q', 'A', char::default(), char::default(), false, true)];
        assert_eq!(q_info, vec);

        let e_info = tree.root.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow()
                                                            .right.as_ref().unwrap().borrow()
                                                          .right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(21, 'E', 'A', char::default(), char::default(), false, false)];
        assert_eq!(e_info, vec);

    }

    #[test]
    fn test_rotate_right3() {
        // one node
        let mut tree: RbTree<i32, char> = RbTree::new();
        //prepare for rotation right
        tree.insert(10, 'X');
        // rotate
        let x = tree.root.clone();
        tree.rotate_right(x);
        //(key, value, parent_value, left_value, right_value, is_nill, is_left_child)
        let x_info = tree.root.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(10, 'X', char::default(), char::default(), char::default(), false, false)];
        assert_eq!(x_info, vec);

    }

    #[test]
    fn test_rotate_right4() {
        //
        // one node + right node
        //
        let mut tree: RbTree<i32, char> = RbTree::new();
        //prepare for rotation right
        tree.insert(10, 'X');
        tree.insert(15, 'B');
        // rotate
        let x = tree.root.clone();
        tree.rotate_right(x);
        //(key, value, parent_value, left_value, right_value, is_nill, is_left_child)
        let x_info = tree.root.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(10, 'X', char::default(), char::default(), 'B', false, false)];
        assert_eq!(x_info, vec);

        let b_info = tree.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(15, 'B', 'X', char::default(), char::default(), false, false)];
        assert_eq!(b_info, vec);
        //
        // one node + left node
        //
        let mut tree: RbTree<i32, char> = RbTree::new();
        //prepare for rotation right
        tree.insert(10, 'X');
        tree.insert(5, 'B');
        // rotate
        let x = tree.root.clone();
        tree.rotate_right(x);
        //(key, value, parent_value, left_value, right_value, is_nill, is_left_child)
        let b_info = tree.root.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(5, 'B', char::default(), char::default(), 'X', false, false)];
        assert_eq!(b_info, vec);

        let x_info = tree.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(10, 'X', 'B', char::default(), char::default(), false, false)];
        assert_eq!(x_info, vec);

        // one node + left + right node
        let mut tree: RbTree<i32, char> = RbTree::new();
        //prepare for rotation right
        tree.insert(10, 'X');
        tree.insert(5, 'B');
        tree.insert(15, 'A');
        // rotate
        let x = tree.root.clone();
        tree.rotate_right(x);
        //(key, value, parent_value, left_value, right_value, is_nill, is_left_child)
        let b_info = tree.root.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(5, 'B', char::default(), char::default(), 'X', false, false)];
        assert_eq!(b_info, vec);

        let x_info = tree.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(10, 'X', 'B', char::default(), 'A', false, false)];
        assert_eq!(x_info, vec);

        let a_info = tree.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().get_debug_info_vec();
        let vec:Vec<(i32, char, char, char, char, bool, bool)>  = vec![(15, 'A', 'X', char::default(), char::default(), false, false)];
        assert_eq!(a_info, vec);
    }
}