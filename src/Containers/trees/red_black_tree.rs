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

 // BEFORE USE
 // each key must not be a duplicate of an already existing key, if this becomes a problem 
 // use a HashSet to see if key exists already in O(1) 

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::option::Option;
use std::fmt::{self, Debug};
use std::result;

type Node<K, V> = Option<Rc<RefCell<RbNode<K, V>>>>;
type WeakNode<K, V> = Option<Weak<RefCell<RbNode<K, V>>>>;

type NonNullNode<K, V> = Rc<RefCell<RbNode<K, V>>>;
type NonNullWeakNode<K, V> = Weak<RefCell<RbNode<K, V>>>;

pub trait Key: Ord + Default + Debug + Clone {}
impl<T: Ord + Default + Debug + Clone> Key for T {}
pub trait Value: Default + Debug + Clone {}
impl<T: Default + Debug + Clone> Value for T {}



#[derive(Clone, PartialEq)]
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
    pub sentinel_above_root: NonNullNode<K, V>,
    pub root: Node<K, V>,
    pub size: usize,
    pub debug: bool
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


impl<K: Key, V: Value> RbTree<K, V> {

    // new function
    pub fn new() -> RbTree<K, V> {
        let root: Node<K, V> = Some(RbNode::new_nil());
        let sentinel = RbNode::new_nil();
        root.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&sentinel));
        RbTree {
            sentinel_above_root: sentinel,
            root: root,
            size: 1,
            debug: false
        }
    }

    // Rotations 
    /* ============== Rotation LEFT =============
             X                 Y
            / \               / \
           α   Y     ==>     X   γ
              / \           / \
             β   γ         α   β
        Does handle is_left_child flag, but not other meta data as color, 
        while the rotation doesnt know why the rotation is happening.
    */// ==========================================

    pub fn rotate_left(&mut self, x: Node<K, V>) -> () {
        if let Some(unwrapped_x) = x {
            
            // make sure x is not a nil node 
            if unwrapped_x.borrow().is_nill {
                return;
            } else if unwrapped_x.borrow().right.as_ref().unwrap().borrow().is_nill {
                return;
            }
        // y takes x.right
            let y: Node<K, V> = unwrapped_x.borrow_mut().right.take();
        // x.right takes y.left, if x is something y is atleast a sentinel, (unwrappable)
            unwrapped_x.borrow_mut().right = y.as_ref().unwrap().borrow_mut().left.take();
            y.as_ref().unwrap().borrow_mut().left = Some(RbNode::new_nil());
            // establish parent chain from y.left to y
            y.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y.as_ref().unwrap()));

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
                // root is a left child 
                self.root.as_ref().unwrap().borrow_mut().is_left_child = true;
                // we need to update the root parent to the sentinel
                self.root.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&self.sentinel_above_root));
                self.sentinel_above_root.borrow_mut().left = self.root.clone();
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

    /* ============= Rotation RIGHT =========================
             X                 Y
            / \               / \
           Y   γ     ==>     α   X
          / \                   / \
         α  β                  β   γ

         Does handle is_left_child flag, but not other meta data as color, 
         while the rotation doesnt know why the rotation is happening.
    */// ====================================================
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
                // root is a left child
                self.root.as_ref().unwrap().borrow_mut().is_left_child = true;
                // we need to update the root parent to the sentinel
                self.root.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&self.sentinel_above_root));
                // sentinel left child is now the root
                self.sentinel_above_root.borrow_mut().left = self.root.clone();
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


   



    // insertion
    pub fn insert(&mut self, key: K, value: V) {
        let z = RbNode::new(key, value.clone());

        // if root is nill 
        if self.root.as_ref().unwrap().borrow().is_nill {
            self.root = Some(Rc::clone(&z));
            // root is a left child
            self.root.as_ref().unwrap().borrow_mut().is_left_child = true;
            self.root.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&self.sentinel_above_root));
            self.sentinel_above_root.borrow_mut().left = self.root.clone();
            z.borrow_mut().color = Color::BLACK;            
            z.borrow_mut().left = Some(RbNode::new_nil());
            // establish parent chain from z.left to z
            z.borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&z));
            z.borrow_mut().right = Some(RbNode::new_nil());
            // establish parent chain from z.right to z
            z.borrow_mut().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&z));
            self.size += 1;
        } else {
            self.insert_node(Some(z));
        }

        if self.debug {
            println!("after inserting {:?}", value.clone());
            println!("{:?}", self);
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
            // root is a left child
            self.root.as_ref().unwrap().borrow_mut().is_left_child = true;
            // we need to update the root parent to the sentinel
            self.root.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&self.sentinel_above_root));
            // sentinel left child is now the root
            self.sentinel_above_root.borrow_mut().left = self.root.clone();
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
        // establish parent chain from z.left to z
        z.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&z.as_ref().unwrap()));
        z.as_ref().unwrap().borrow_mut().right = Some(RbNode::new_nil());
        // establish parent chain from z.right to z
        z.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&z.as_ref().unwrap()));
        z.as_ref().unwrap().borrow_mut().color = Color::RED;
        self.insert_fixup(z);
    } 

    fn insert_fixup(&mut self, mut z: Node<K, V>) {
        while let Some(zp_weak) = {
            let z_borrow = z.as_ref().unwrap().borrow();
            z_borrow.parent.clone()
        } {
            let z_parent_strong = zp_weak.upgrade().unwrap();
            if z_parent_strong.borrow().color == Color::RED {
                // parent exist and is red
                // if parent is a left child 
                if z_parent_strong.borrow().is_left_child {
                    // we do one thing if it's a left child
                    // if z_parent is root we asign aunt a nil node , else we give it roots right
                    let mut aunt: NonNullNode<K, V> = RbNode::new_nil();
                    if z_parent_strong.borrow().key != self.root.as_ref().unwrap().borrow().key {
                        // zp was not root and there for we can assign atleast the roots nil node to aunt
                        let zpp_weak = z_parent_strong.borrow().parent.clone();
                        let zpp_strong = zpp_weak.as_ref().unwrap().upgrade().unwrap();
                        aunt = zpp_strong.borrow().right.as_ref().unwrap().clone();
                    } else {
                        break; 
                    }
                    // aunt is now something, either a nil or zp's sibling 
                    // CASE 1: AUNT RED
                    if aunt.borrow().color == Color::RED {
                        z_parent_strong.borrow_mut().color = Color::BLACK;
                        aunt.borrow_mut().color = Color::BLACK;
                        let zpp_weak = z_parent_strong.borrow().parent.clone();
                        let mut zpp_strong = zpp_weak.as_ref().unwrap().upgrade();
                        zpp_strong.as_ref().unwrap().borrow_mut().color = Color::RED;
                        z = zpp_strong.take();
                    } else if !z.as_ref().unwrap().borrow().is_left_child {
                        // else if aunt is black && z is right child 
                        z = Some(z_parent_strong.clone());
                        self.rotate_left(z.clone());
                    } else {
                        // if z is a left child we instead want a right rotation but first we recolor
                        z_parent_strong.borrow_mut().color = Color::BLACK;
                        let zpp_weak = z_parent_strong.borrow().parent.clone();
                        let zpp_strong = zpp_weak.as_ref().unwrap().upgrade();
                        zpp_strong.as_ref().unwrap().borrow_mut().color = Color::RED;
                        //right rotate zpp
                        self.rotate_right(zpp_strong.clone());
                    }
                } else {


                    // we do another thing if it's a right child
                    // if z_parent is root we asign aunt a nil node , else we give it roots left
                    let mut aunt: NonNullNode<K, V> = RbNode::new_nil();
                    if z_parent_strong.borrow().key != self.root.as_ref().unwrap().borrow().key {
                        // zp was not root and there for we can assign atleast the roots nil node to aunt
                        let zpp_weak = z_parent_strong.borrow().parent.clone();
                        let zpp_strong = zpp_weak.as_ref().unwrap().upgrade().unwrap();
                        aunt = zpp_strong.borrow().left.as_ref().unwrap().clone();
                    } else {
                        break; 
                    }
                    // aunt is now something, either a nil or zp's sibling 
                    // CASE 1: AUNT RED
                    if aunt.borrow().color == Color::RED {
                        z_parent_strong.borrow_mut().color = Color::BLACK;
                        aunt.borrow_mut().color = Color::BLACK;
                        let zpp_weak = z_parent_strong.borrow().parent.clone();
                        let mut zpp_strong = zpp_weak.as_ref().unwrap().upgrade();
                        zpp_strong.as_ref().unwrap().borrow_mut().color = Color::RED;
                        z = zpp_strong.take();
                    } else if z.as_ref().unwrap().borrow().is_left_child {
                        // else if aunt is black && z is left child 
                        z = Some(z_parent_strong.clone());
                        self.rotate_right(z.clone());
                    } else {
                        // if z is a right child we instead want a left rotation but first we recolor
                        z_parent_strong.borrow_mut().color = Color::BLACK;
                        let zpp_weak = z_parent_strong.borrow().parent.clone();
                        let zpp_strong = zpp_weak.as_ref().unwrap().upgrade();
                        zpp_strong.as_ref().unwrap().borrow_mut().color = Color::RED;
                        //right rotate zpp
                        self.rotate_left(zpp_strong.clone());
                    }
                }
            } else {
                // either parent doesnt exist, or its BLACK
                break;
            }
        };

        self.root.as_ref().unwrap().borrow_mut().color = Color::BLACK;

    }



    // deletion

    // delete helper functions
     // Utility for deletion

     pub fn swapColors(&mut self, x: Node<K, V>, y: Node<K, V>) {
        if let (Some(unwr_x), Some(unwr_y)) = (x, y) {
            let temp = unwr_x.borrow().color.clone();
            unwr_x.borrow_mut().color = unwr_y.borrow().color.clone();
            unwr_y.borrow_mut().color = temp;
        }
    }

    pub fn swapValues(&mut self, x: Node<K, V>, y: Node<K, V>) {
        if let (Some(unwr_x), Some(unwr_y)) = (x, y) {
            // Swap keys
            let temp_key = unwr_x.borrow().key.clone();
            unwr_x.borrow_mut().key = unwr_y.borrow().key.clone();
            unwr_y.borrow_mut().key = temp_key;
    
            // Swap values
            let temp_value = unwr_x.borrow().val.clone();
            unwr_x.borrow_mut().val = unwr_y.borrow().val.clone();
            unwr_y.borrow_mut().val = temp_value;
        }
    }

    // does not take responsibility for updating v.left or v.right, caller has to 
    // set u's children into v subtree before calling if they should be kept.
    fn transplant(&mut self, u: NonNullNode<K, V>, v: NonNullNode<K, V>) {
        if u.borrow().key == self.root.as_ref().unwrap().borrow().key {
            self.root = Some(v.clone());
            // we need to update the root parent to the sentinel
            self.root.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&self.sentinel_above_root));
            // sentinel left child is now the root
            self.sentinel_above_root.borrow_mut().left = self.root.clone();
        } else if u.borrow().is_left_child {
            // u will have a parent while u isnt root
            let u_parent_weak = u.borrow().parent.clone();
            let u_parent_strong = u_parent_weak.unwrap().upgrade();
            u_parent_strong.as_ref().unwrap().borrow_mut().left = Some(v.clone()); 
        } else {
            let u_parent_weak = u.borrow().parent.clone();
            let u_parent_strong = u_parent_weak.unwrap().upgrade();
            u_parent_strong.as_ref().unwrap().borrow_mut().right = Some(v.clone()); 
        }

        // if u was a left child we need to update v's is_left_child flag
        v.borrow_mut().is_left_child = u.borrow().is_left_child;

        // if u not root we also want to set v.p = u.p
        if u.borrow().key != self.root.as_ref().unwrap().borrow().key {
            v.borrow_mut().parent = u.borrow_mut().parent.take();
        }
    }

    pub fn succesor(&self, mut x: Node<K, V>) -> Node<K, V> {
        if x.is_none() {
            return None;
        }
        while x.as_ref().unwrap().borrow().left.is_some() && !x.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().is_nill {
            let new_x = x.as_ref().unwrap().borrow().left.clone();
            x = new_x;
        }
        x
    }

    pub fn replace(&self, x: Node<K, V>) -> Node<K, V>{
        //Finds the replacement node in BST for the given node
        // if x doesnt exist
        if x.is_none() {
            return None;
        }
        // if x.left is not None and x.right is not None:
        if x.as_ref().unwrap().borrow().left.is_some() && !x.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().is_nill 
           && x.as_ref().unwrap().borrow().right.is_some() && !x.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().is_nill 
        {
            return self.succesor(x.as_ref().unwrap().borrow().right.clone());
        }
        // if x.left is None and x.right is None:
        if x.as_ref().unwrap().borrow().left.is_none() || (x.as_ref().unwrap().borrow().left.is_some() && x.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().is_nill) 
           && x.as_ref().unwrap().borrow().right.is_none() || (x.as_ref().unwrap().borrow().right.is_some() && x.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().is_nill) 
        {
            return Some(x.as_ref().unwrap().borrow().left.as_ref().unwrap().clone());
        }

        // if x.left is not None:
        if x.as_ref().unwrap().borrow().left.is_some() && !x.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().is_nill {
            return x.as_ref().unwrap().borrow().left.clone();
        } else {
            return x.as_ref().unwrap().borrow().right.clone();
        }
    }

    fn set_parent_links(&mut self, x: Node<K, V>) {
        // if x has children we need to update their parent links
        if x.is_some() && !x.as_ref().unwrap().borrow().is_nill{
            // we have two unwrappable children
            x.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&x.as_ref().unwrap()));
            x.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&x.as_ref().unwrap()));
            // set their left_child flag
            x.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().is_left_child = false;
            x.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().is_left_child = true;
        }
    }
    
    
    pub fn delete(&mut self, key: K) -> Result<(K, V), ()> {
        let z: Node<K, V> = self.find_node(key);
        if z.is_none() {
            return Err(());
        }
        let found_key = z.as_ref().unwrap().borrow().key.clone();
        let found_val = z.as_ref().unwrap().borrow().val.clone();

        // if z is root and both children are nill
        if z.as_ref().unwrap().borrow().key == self.root.as_ref().unwrap().borrow().key 
           && z.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().is_nill 
           && z.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().is_nill {
            self.root = Some(RbNode::new_nil());
            self.root.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&self.sentinel_above_root));
            self.sentinel_above_root.borrow_mut().left = self.root.clone();
            self.size -= 1;
            return Ok((found_key, found_val));
        }
        
        self.size -= 1;

        self.delete_node(z.unwrap());

        if self.debug {
            println!("after deleting {:?}", found_val.clone());
            println!("{:?}", self);
        }
        return Ok((found_key, found_val));

    }

    // precondition: z is not a sentinel node
    fn delete_node(&mut self, z: NonNullNode<K, V>) {
        let mut y = z.clone();
        let mut y_original_color = y.borrow().color.clone();
        let mut x: Node<K, V> = None;
        if z.borrow().left.is_some() && z.borrow().left.as_ref().unwrap().borrow().is_nill {
            // z has at most one right child
            x = z.borrow().right.clone();
            self.set_parent_links(x.clone());
            self.transplant(z.clone(), x.as_ref().unwrap().clone());
        } else if z.borrow().right.is_some() && z.borrow().right.as_ref().unwrap().borrow().is_nill {
            // z has at most one left child
            x = z.borrow().left.clone();
            self.set_parent_links(x.clone());
            self.transplant(z.clone(), x.as_ref().unwrap().clone());
        } else {
            // z has two children
            
            y = self.succesor(z.borrow().right.clone()).unwrap();
            y_original_color = y.borrow().color.clone();
            x = y.borrow().right.clone();
            self.set_parent_links(Some(y.clone()));

            if y.borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().key == z.borrow().key {
                x.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y));
            } else {
                self.transplant(y.clone(), x.as_ref().unwrap().clone());
                y.borrow_mut().right = z.borrow().right.clone();
                y.borrow().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y));
            }
            self.transplant(z.clone(), y.clone());
            y.borrow_mut().left = z.borrow().left.clone();
            y.borrow().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y));
            y.borrow_mut().color = z.borrow().color.clone();
        }
        if y_original_color == Color::BLACK {
            self.set_parent_links(x.clone());
            self.delete_fixup(x.clone());
        }
    }

    // deletion fix 
    fn delete_fixup(&mut self, z: Node<K, V>) {
        let mut x = z.clone();
        while x.as_ref().unwrap().borrow().color == Color::BLACK && x.as_ref().unwrap().borrow().key != self.root.as_ref().unwrap().borrow().key {
           if x.as_ref().unwrap().borrow().is_left_child {
                let mut w = x.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().right.clone();
                if w.as_ref().unwrap().borrow().is_nill {
                    // w is nil, no sibling to adjust
                    println!("X LEFT CHILD, w is nil, no sibling to adjust");

                    // debug w, x
                    print!("w: ");
                    w.as_ref().unwrap().borrow().print_information();
                    print!("x: ");
                    x.as_ref().unwrap().borrow().print_information();

                    let x_parent_weak = x.as_ref().unwrap().borrow().parent.clone();
                    let x_parent_strong = x_parent_weak.as_ref().unwrap().upgrade().unwrap();

                    // debug x_parent
                    print!("x_parent: ");
                    x_parent_strong.borrow().print_information();

                    x = Some(x_parent_strong.clone());
                } else {
                    if w.as_ref().unwrap().borrow().color == Color::RED {
                        w.as_ref().unwrap().borrow_mut().color = Color::BLACK;
                        x.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow_mut().color = Color::RED;
                        let x_parent_weak = x.as_ref().unwrap().borrow().parent.clone();
                        let x_parent_strong = x_parent_weak.as_ref().unwrap().upgrade().unwrap();
                        self.rotate_left(Some(x_parent_strong.clone()));
                        w = x.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().right.clone();
                    }
                    if w.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().color == Color::BLACK && w.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().color == Color::BLACK {
                        w.as_ref().unwrap().borrow_mut().color = Color::RED;
                        let x_parent_weak = x.as_ref().unwrap().borrow().parent.clone();
                        let x_parent_strong = x_parent_weak.as_ref().unwrap().upgrade().unwrap();
                        x = Some(x_parent_strong.clone());
                    } else {
                        if w.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().color == Color::BLACK {
                            w.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow_mut().color = Color::BLACK;
                            w.as_ref().unwrap().borrow_mut().color = Color::RED;
                            self.rotate_right(w.clone());
                            w = x.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().right.clone();
                        }
                        let x_parent_weak = x.as_ref().unwrap().borrow().parent.clone();
                        let x_parent_strong = x_parent_weak.as_ref().unwrap().upgrade().unwrap();
                        w.as_ref().unwrap().borrow_mut().color = x_parent_strong.borrow().color.clone();
                        x.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow_mut().color = Color::BLACK;
                        w.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().color = Color::BLACK;
                        let x_parent_weak = x.as_ref().unwrap().borrow().parent.clone();
                        let x_parent_strong = x_parent_weak.as_ref().unwrap().upgrade().unwrap();
                        self.rotate_left(Some(x_parent_strong));
                        x = self.root.clone();
                    }
                }
            } else {
                let mut w = x.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().left.clone();
                if w.as_ref().unwrap().borrow().is_nill {
                    // w is nil, no sibling to adjust
                    println!("X RIGHT CHILD, w is nil, no sibling to adjust");

                    // debug w, x
                    print!("w: ");
                    w.as_ref().unwrap().borrow().print_information();
                    print!("x: ");
                    x.as_ref().unwrap().borrow().print_information();

                    let x_parent_weak = x.as_ref().unwrap().borrow().parent.clone();
                    let x_parent_strong = x_parent_weak.as_ref().unwrap().upgrade().unwrap();

                    // debug x_parent
                    print!("x_parent: ");
                    x_parent_strong.borrow().print_information();

                    x = Some(x_parent_strong.clone());
                } else {
                    if w.as_ref().unwrap().borrow().color == Color::RED {
                        w.as_ref().unwrap().borrow_mut().color = Color::BLACK;
                        x.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow_mut().color = Color::RED;
                        let x_parent_weak = x.as_ref().unwrap().borrow().parent.clone();
                        let x_parent_strong = x_parent_weak.as_ref().unwrap().upgrade().unwrap();
                        self.rotate_right(Some(x_parent_strong.clone()));
                        w = x.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().left.clone();
                    }
                    if w.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().color == Color::BLACK && w.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().color == Color::BLACK {
                        w.as_ref().unwrap().borrow_mut().color = Color::RED;
                        let x_parent_weak = x.as_ref().unwrap().borrow().parent.clone();
                        let x_parent_strong = x_parent_weak.as_ref().unwrap().upgrade().unwrap();
                        x = Some(x_parent_strong.clone());
                    } else {
                        if w.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().color == Color::BLACK {
                            w.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().color = Color::BLACK;
                            w.as_ref().unwrap().borrow_mut().color = Color::RED;
                            self.rotate_left(w.clone());
                            w = x.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().left.clone();
                        }
                        let x_parent_weak = x.as_ref().unwrap().borrow().parent.clone();
                        let x_parent_strong = x_parent_weak.as_ref().unwrap().upgrade().unwrap();
                        w.as_ref().unwrap().borrow_mut().color = x_parent_strong.borrow().color.clone();
                        x.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow_mut().color = Color::BLACK;
                        w.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow_mut().color = Color::BLACK;
                        let x_parent_weak = x.as_ref().unwrap().borrow().parent.clone();
                        let x_parent_strong = x_parent_weak.as_ref().unwrap().upgrade().unwrap();
                        self.rotate_right(Some(x_parent_strong));
                        x = self.root.clone();
                    }
                }
            }
        }
        x.as_ref().unwrap().borrow_mut().color = Color::BLACK;
    }
    // utility

    pub fn find_node(&mut self, key: K) -> Node<K, V> {
        fn recursive_helper<K: Key, V:Value>(node: Node<K, V>, key: &K) -> Node<K, V> {
            if let Some(subroot) = node {
                if subroot.borrow().key == *key {
                    // base case we found the node!
                    return Some(subroot.clone());
                } else {
                    // left ok? 
                    if recursive_helper(subroot.borrow().left.clone(), key).is_some(){
                        return recursive_helper(subroot.borrow().left.clone(), key);
                    } 
                    // right ok?
                    if recursive_helper(subroot.borrow().right.clone(), key).is_some(){
                        return recursive_helper(subroot.borrow().right.clone(), key);
                    } 
                    return None;
                }
            } else {
                return None
            }
        }
        recursive_helper(self.root.clone(), &key)
    }

    pub fn in_tree(&mut self, key: K) -> bool {
        self.find_node(key).is_some()
    }

    /*
                    B8
                   /   \
                R4     R12
               /  \   /  \
              B2  B6 B10  B14
                        
     */
    pub fn get_tree1() -> RbTree<i32, i32> {
        let mut tree = RbTree::new();
        tree.insert(2, 2);
        tree.insert(4, 4);
        tree.insert(6, 6);
        tree.insert(8, 8);
        tree.insert(10, 10);
        tree.insert(12, 12);
        tree.insert(14, 14);
        tree.insert(16, 16);
        let _ = tree.delete(16);
        return tree;
    }

    /*
                    B8
                   /   \
                B4     B12
               /  \   /  \
              B2  B6 B10  R16
                          / \
                        B14 R18
                               \
                               R20
     */

    pub fn get_tree2() -> RbTree<i32, i32> {
        let mut tree = RbTree::new();
        tree.insert(2, 2);
        tree.insert(4, 4);
        tree.insert(6, 6);
        tree.insert(8, 8);
        tree.insert(10, 10);
        tree.insert(12, 12);
        tree.insert(14, 14);
        tree.insert(16, 16);
        tree.insert(18, 18);
        tree.insert(20, 20);
        return tree;
    }

    /*
                    B14
                   /   \
                B10     B18
                /  \    /  \
             R6   B12 B16  B20
            /  \
          B4    B8
          /
         R2
     */

    pub fn get_tree3() -> RbTree<i32, i32> {
        let mut tree = RbTree::new();
        tree.insert(20, 20);
        tree.insert(18, 18);
        tree.insert(16, 16);
        tree.insert(14, 14);
        tree.insert(12, 12);
        tree.insert(10, 10);
        tree.insert(8, 8);
        tree.insert(6, 6);
        tree.insert(4, 4);
        tree.insert(2, 2);
        return tree;
    }

    // ######### VALIDATION FUNCTIONS 
    /*
        * 1 - Every node is either red or black
        * 2 - Root is black
        * 3 - Red nodes have black children
        * 4 - All paths from a node to its descendant null pointers have the same number of black nodes
     */

    // * 2 - Root is black
    fn validate_rule_2(&mut self) -> bool {
        if let Some(unwrapped_root) = &self.root {
            if unwrapped_root.borrow().color == Color::RED {
                return false
            } 
        }
        return true; 
    }
    // * 3 - Red nodes have black children
    fn validate_rule_3(&mut self) -> bool {
        fn recursive_helper<K: Key, V: Value>(node: Node<K, V>) -> bool{
            if let Some(unwrapped_node) = node {
                let mut curr_is_ok = true;
                if unwrapped_node.borrow().color == Color::RED {
                    // it must have black kids 
                    // check left
                    if unwrapped_node.borrow().left.is_some() {
                        if unwrapped_node.borrow().left.as_ref().unwrap().borrow().color == Color::RED {
                            curr_is_ok = false; 
                        }
                    }
                    // check right 
                    if unwrapped_node.borrow().right.is_some() {
                        if unwrapped_node.borrow().right.as_ref().unwrap().borrow().color == Color::RED {
                            curr_is_ok = false; 
                        }
                    }
                }

                return curr_is_ok && recursive_helper(unwrapped_node.borrow().left.clone()) && recursive_helper(unwrapped_node.borrow().right.clone())
            }
            return true
        }
        return recursive_helper(self.root.clone())
    }

    // * 4 - All paths from a node to its descendant null pointers have the same number of black nodes
    fn validate_rule_4(&mut self) -> bool{
        fn recursive_helper<K: Key, V: Value>(node: &Node<K, V>) -> (bool, i32) {
            let mut under_is_ok = true; 
            let mut blacks = 0; 
            if let Some(unwrapped_node) = node {
                let left = recursive_helper(&unwrapped_node.borrow().left.clone());
                let right = recursive_helper(&unwrapped_node.borrow().right.clone());
                if left.1 != right.1 || !left.0 || !right.0 {
                    under_is_ok = false;
                }
                if unwrapped_node.borrow().color == Color::BLACK {
                    blacks = left.1 + 1;
                } else {
                    blacks = left.1;
                }
            }
            return (under_is_ok, blacks);
        }

        let (result, _) = recursive_helper(&self.root.clone()); 
        return result;
    }

    pub fn is_valid_tree(&mut self) -> bool {
        return self.validate_rule_2() && self.validate_rule_3() && self.validate_rule_4();
    }

    // debug if a refcell is being borrowed and if so print if borrowed mut or immutable by trying to borrow
    fn print_borrow_state(&self, node: Option<&Rc<RefCell<RbNode<K, V>>>>) {
        match node.as_ref().unwrap().try_borrow_mut() {
            Ok(_) => println!("not borrowed already"),
            Err(_) => {
                match node.as_ref().unwrap().try_borrow()  {
                    Ok(_) => println!("immutably borrowed"),
                    Err(_) => println!("mutably borrowed")
                }
            }
        }
    }


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

    pub fn get_uncle(&self) -> Node<K,V> {
        if let Some(parent_weak) = &self.parent {
            if let Some(parent) = parent_weak.upgrade() {
                if let Some(gp_weak) = &parent.borrow().parent {
                    if let Some(gp) = gp_weak.upgrade() {
                        if parent.borrow().is_left_child {
                            return gp.borrow().right.clone();
                        } else {
                            return gp.borrow().left.clone();
                        }
                    }
                }
            }
        }
        return None;
    }

    pub fn get_sibling(&self) -> Node<K,V> {
        if let Some(parent_weak) = &self.parent {
            if let Some(parent) = parent_weak.upgrade() {
                if self.is_left_child {
                    return parent.borrow().right.clone();
                } else {
                    return parent.borrow().left.clone();
                }
            }
        }
        return None;
    }

    pub fn has_red_child(&self) -> bool {
        if self.left.is_some() && !self.left.as_ref().unwrap().borrow().is_nill && self.left.as_ref().unwrap().borrow().color == Color::RED {
            return true;
        }
        if self.right.is_some() && !self.right.as_ref().unwrap().borrow().is_nill && self.right.as_ref().unwrap().borrow().color == Color::RED {
            return true;
        }
        return false;
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
        Self::tree_printer_traverse_helper(&mut sb, "", "", &Some(self.sentinel_above_root.clone()));
        write!(f, "{}", sb)
    }
}

// testing area 

#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use super::*;
    // ==================================
    // ==         Utility              ==
    // ==================================

    #[test]
    fn test_find_node(){
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.insert(10, 'A');
        tree.insert(5, 'B');
        tree.insert(15, 'C');
        let b: Node<i32, char> = tree.find_node(5);
        assert!(b.is_some());
        assert_eq!(b.unwrap().borrow_mut().val, 'B');
    }

    #[test]
    fn test_in_tree(){
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.insert(10, 'A');
        tree.insert(5, 'B');
        tree.insert(15, 'C');
        let b = tree.in_tree(5);
        assert!(b);
    }

    // ==================================
    // ==         Insertion            ==
    // ==================================
    #[test]
    fn test_insertion_1() {
        // a valid Binary tree insertion, Triangle !
        // no changes should be needed, Testing the early edge cases of insertion 
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.debug = true; 
        tree.insert(10, 'A');
        tree.insert(5, 'B');
        tree.insert(15, 'C');

        tree.sentinel_above_root.borrow_mut().print_information();

        assert!(tree.is_valid_tree());
    }

    #[test]
    fn test_insertion_2() {

        /*
                   A
                  /
                 B
                /
               C
         */
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.debug = true;
        tree.insert(10, 'A');
        tree.insert(5, 'B');
        tree.insert(3, 'C');

        assert!(tree.is_valid_tree());

        /*
            A
             \
              B
               \
                C
         */
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.debug = true;
        tree.insert(10, 'A');
        tree.insert(15, 'B');
        tree.insert(20, 'C');

        assert!(tree.is_valid_tree());
    }

    #[test]
    fn test_insertion_3() {
        // case 1 z's uncle/aunt is red as well as z's parent
        /*
                    G              G
                   / \            / \
                  P   U    or    P  U
                /                    \
               z                      z
         */
        // when zp is a left child
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.debug = true;
        tree.insert(10, 'G');   //black
        tree.insert(5, 'P');    //red
        tree.insert(15, 'U');   //red
        tree.insert(2, 'Z');     // red, VIOLATION of property 4 , case 1
        assert!(tree.is_valid_tree());
        // when zp is a right child
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.debug = true;
        tree.insert(10, 'G');   //black
        tree.insert(5, 'P');    //red
        tree.insert(15, 'U');   //red
        tree.insert(20, 'Z');   // red, VIOLATION of property 4 , case 1
        assert!(tree.is_valid_tree());
    }

    

    #[test]
    fn test_insertion_4() {
        // case 2 Aunt is black and z is a right child
         /*
                    G                       G              
                   / \                     / \
                  P   U                   P  U
                       \         or      /
                        H               H
                         \               \
                          Z               Z
            G, P, U are black 
            H & Z is red 
            ZP is red, Z's aunt is black (sentinel), Z is a right child(Case 2)
         */

        // when zp is a left child
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.debug = true;
        tree.insert(10, 'G');   //black
        tree.insert(5, 'P');    //black
        tree.insert(15, 'U');   //black
        tree.insert(3, 'H');   //red
        tree.insert(4, 'Z');   // red, VIOLATION of property 4 , case 2
        assert!(tree.is_valid_tree());

        // when zp is a right child
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.debug = true;
        tree.insert(10, 'G');   //black
        tree.insert(5, 'P');    //black
        tree.insert(15, 'U');   //black
        tree.insert(17, 'H');   //red
        tree.insert(18, 'Z');   // red, VIOLATION of property 4 , case 2
    }

    #[test]
    fn test_insertion_5() {
        // case 3 Aunt is black and z is a left child
        /*
                    G                       G              
                   / \                     / \
                  P   U                   P  U
                       \         or      /
                        H               H
                       /               /
                      Z               Z
            G, P, U are black 
            H & Z is red 
            ZP is red, Z's aunt is black (sentinel), Z is a right child(Case 2)
         */

        // when zp is a left child
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.debug = true;
        tree.insert(10, 'G');   //black
        tree.insert(5, 'P');    //black
        tree.insert(15, 'U');   //black
        tree.insert(3, 'H');   //red
        tree.insert(2, 'Z');   // red, VIOLATION of property 4 , case 3
        assert!(tree.is_valid_tree());

        // when zp is a right child
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.debug = true;
        tree.insert(10, 'G');   //black
        tree.insert(5, 'P');    //black
        tree.insert(15, 'U');   //black
        tree.insert(17, 'H');   //red
        tree.insert(16, 'Z');   // red, VIOLATION of property 4 , case 3

    }

    #[test]
    fn test_insertion_6() {
        // big tree insertion test asserting after each insertion
        let mut tree: RbTree<i32, i32> = RbTree::new();

        let vec: Vec<i32> = (0..1000).collect();
        for i in vec {
            tree.insert(i, i);
            assert!(tree.is_valid_tree());
        }

    }

    #[test]
    fn test_insertion_7() {
        // big tree insertion test asserting after each insertion
        let mut tree: RbTree<i32, i32> = RbTree::new();

        let mut vec: Vec<i32> = (0..1000).collect();
        let mut rng = thread_rng();
        vec.shuffle(&mut rng);
        
        for i in vec {
            tree.insert(i, i);
            assert!(tree.is_valid_tree());
        }
    }

    #[test]
    fn test_insertion_8() {
        // super heavy insertion
        // big tree insertion test asserting after each insertion
        let mut tree: RbTree<i32, i32> = RbTree::new();

        let vec: Vec<i32> = (0..90000).collect();
        for i in vec {
            tree.insert(i, i);
        }
        assert!(tree.is_valid_tree());
        // print root sentinel 
        tree.sentinel_above_root.as_ref().borrow().print_information();
    }

    // ==================================
    // ==         Deletion             ==
    // ==================================

    #[test]
    fn test_transplant() {
        //-------becomes new root
        // Create a RedBlackTree and populate it with some nodes
        let mut rb_tree = RbTree::new();
        rb_tree.insert(20, "value20");
        rb_tree.insert(10, "value10");
        rb_tree.insert(30, "value30");
        // Get references to the nodes you want to transplant
        let u = rb_tree.find_node(20).unwrap();
        let v = rb_tree.find_node(30).unwrap();
        // Perform the transplant
        rb_tree.transplant(u.clone(), v.clone());
        assert_eq!(rb_tree.root.as_ref().unwrap().as_ref().borrow().key, v.as_ref().borrow().key);
        // v.parent is none
        assert!(v.as_ref().borrow().parent.is_some());
        assert!(rb_tree.sentinel_above_root.as_ref().borrow().left.as_ref().unwrap().as_ref().borrow().key == v.as_ref().borrow().key);

        //-------right child
        // Create a RedBlackTree and populate it with some nodes
        let mut rb_tree = RbTree::new();
        rb_tree.insert(20, "value20");
        rb_tree.insert(10, "value10");
        rb_tree.insert(30, "value30");
        rb_tree.insert(40, "value40");
        // Get references to the nodes you want to transplant
        let u = rb_tree.find_node(30).unwrap();
        let v = rb_tree.find_node(40).unwrap();
        // Perform the transplant
        rb_tree.transplant(u.clone(), v.clone());
        assert_eq!(rb_tree.root.as_ref().unwrap().as_ref().borrow().right.as_ref().unwrap().as_ref().borrow().key, v.as_ref().borrow().key);
        // v.parent == root
        let v_p_weak = v.as_ref().borrow().parent.clone();
        let v_p_strong = v_p_weak.unwrap().upgrade();
        assert_eq!(rb_tree.root.as_ref().unwrap().as_ref().borrow().key, v_p_strong.as_ref().unwrap().as_ref().borrow().key);

        // ------left child
        // Create a RedBlackTree and populate it with some nodes
        let mut rb_tree = RbTree::new();
        rb_tree.insert(20, "value20");
        rb_tree.insert(10, "value10");
        rb_tree.insert(30, "value30");
        rb_tree.insert(25, "value40");
        // Get references to the nodes you want to transplant
        let u = rb_tree.find_node(30).unwrap();
        let v = rb_tree.find_node(25).unwrap();
        // Perform the transplant
        rb_tree.transplant(u.clone(), v.clone());
        assert_eq!(rb_tree.root.as_ref().unwrap().as_ref().borrow().right.as_ref().unwrap().as_ref().borrow().key, v.as_ref().borrow().key);
        // v.parent == root
        let v_p_weak = v.as_ref().borrow().parent.clone();
        let v_p_strong = v_p_weak.unwrap().upgrade();
        assert_eq!(rb_tree.root.as_ref().unwrap().as_ref().borrow().key, v_p_strong.as_ref().unwrap().as_ref().borrow().key);

    }

    #[test]
    fn test_tree_minimum() {
        // Create a RedBlackTree and populate it with some nodes
        let mut rb_tree = RbTree::new();
        rb_tree.insert(20, "value20");
        rb_tree.insert(10, "value10");
        rb_tree.insert(30, "value30");
        rb_tree.insert(25, "value40");
        rb_tree.insert(40, "value50");
        // Get references to the nodes you want to transplant
        let mut x: Option<Rc<RefCell<RbNode<i32, &str>>>> = rb_tree.find_node(30);
        let mut y: Rc<RefCell<RbNode<i32, &str>>> = rb_tree.succesor(x.clone()).unwrap();
        assert_eq!(y.as_ref().borrow().key, 25);

        // when x is the lowest node
        x = rb_tree.find_node(10);
        y = rb_tree.succesor(x.clone()).unwrap();
        assert_eq!(y.as_ref().borrow().key, 10);

    }


    /*
                    B8
                   /   \
                R4     R12
               /  \   /  \
              B2  B6 B10  B14
                        
     */
    #[test]
    fn test_get_uncle(){
        let mut rb_tree:RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        // 2's uncle is 12
        let z = rb_tree.find_node(2).unwrap();
        let uncle = z.as_ref().borrow().get_uncle();
        assert_eq!(uncle.as_ref().unwrap().as_ref().borrow().key, 12);
        // 6's uncle is 12
        let z = rb_tree.find_node(6).unwrap();
        let uncle = z.as_ref().borrow().get_uncle();
        assert_eq!(uncle.as_ref().unwrap().as_ref().borrow().key, 12);
        // 10's uncle is 4
        let z = rb_tree.find_node(10).unwrap();
        let uncle = z.as_ref().borrow().get_uncle();
        assert_eq!(uncle.as_ref().unwrap().as_ref().borrow().key, 4);
        // 14's uncle is 4
        let z = rb_tree.find_node(14).unwrap();
        let uncle = z.as_ref().borrow().get_uncle();
        assert_eq!(uncle.as_ref().unwrap().as_ref().borrow().key, 4);
        // 12's uncle is None
        let z = rb_tree.find_node(12).unwrap();
        let uncle = z.as_ref().borrow().get_uncle();
        assert!(uncle.is_none());
        // 8's uncle is None
        let z = rb_tree.find_node(8).unwrap();
        let uncle = z.as_ref().borrow().get_uncle();
        assert!(uncle.is_none());
    }

    /*
                    B8
                   /   \
                R4     R12
               /  \   /  \
              B2  B6 B10  B14
                        
     */
    #[test]
    fn test_get_sibling(){
        let mut rb_tree:RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();

        // 2's sibling is 6
        let z = rb_tree.find_node(2).unwrap();
        let sibling = z.as_ref().borrow().get_sibling();
        assert_eq!(sibling.as_ref().unwrap().as_ref().borrow().key, 6);

        // 6's sibling is 2
        let z = rb_tree.find_node(6).unwrap();
        let sibling = z.as_ref().borrow().get_sibling();
        assert_eq!(sibling.as_ref().unwrap().as_ref().borrow().key, 2); 

        // 12's sibling is 4
        let z = rb_tree.find_node(12).unwrap();
        let sibling = z.as_ref().borrow().get_sibling();
        assert_eq!(sibling.as_ref().unwrap().as_ref().borrow().key, 4);

        // 8's sibling is None
        let z = rb_tree.find_node(8).unwrap();
        let sibling = z.as_ref().borrow().get_sibling();
        assert!(sibling.is_none());
    }

    #[test]
    fn test_has_red_child() {
        let mut rb_tree:RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        rb_tree.debug = true;
        rb_tree.insert(1, 1);

        // 14 should have a red child
        let z = rb_tree.find_node(2).unwrap();
        assert!(z.as_ref().borrow().has_red_child()); 
    }

    #[test]
    fn test_deletion_1() {
        // edge case, deleting a node with no children (Only touching red nodes)
        // z is red
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.debug = true;
        tree.insert(10, 'A');
        tree.insert(5, 'B');
        tree.insert(15, 'C');
        let _ = tree.delete(5);
        assert!(tree.is_valid_tree());

        // mirrored
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.debug = true;
        tree.insert(10, 'A');
        tree.insert(5, 'B');
        tree.insert(15, 'C');
        let _ = tree.delete(15);
        assert!(tree.is_valid_tree());

        // if deleting roots only kid 
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.debug = true;
        tree.insert(10, 'A');
        tree.insert(15, 'B');
        let _ = tree.delete(15);
        assert!(tree.is_valid_tree());

        // if deleting root 
        let mut tree: RbTree<i32, char> = RbTree::new();
        tree.debug = true;
        tree.insert(10, 'A');
        let _ = tree.delete(10);
        assert!(tree.is_valid_tree());
        tree.insert(10, 'A');


    }


    /*
                    B8
                   /   \
                R4     R12
               /  \   /  \
              B2  B6 B10  B14
                        
     */
    #[test]
    fn test_deletion_2() {

        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        let _ = tree.delete(2);
        assert!(tree.is_valid_tree());

        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        let _ = tree.delete(6);
        assert!(tree.is_valid_tree());

        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        let _ = tree.delete(10);
        assert!(tree.is_valid_tree());

        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        let _ = tree.delete(14);
        assert!(tree.is_valid_tree());

        // aunt would still have 2 black kids if we instead delete 
        /*
                B2 from here                       B6 from here
                    B8                                B8
                   /   \                            /   \
                R4     R12             or         R4    R12                    and so on
               /  \   /  \                       /  \   /  \
              B2  B6 B10  B14                   B2  B6 B10  B14
             /                                       \
            R1                                       R7
        */

        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        tree.insert(1, 1);
        let _ = tree.delete(2);
        assert!(tree.is_valid_tree());

        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        tree.insert(3, 3);
        let _ = tree.delete(2);
        assert!(tree.is_valid_tree());

        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        tree.insert(5, 5);
        let _ = tree.delete(6);
        assert!(tree.is_valid_tree());

        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        tree.insert(7, 7);
        let _ = tree.delete(6);
        assert!(tree.is_valid_tree());

        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        tree.insert(9, 9);
        let _ = tree.delete(10);
        assert!(tree.is_valid_tree());

        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        tree.insert(11, 11);
        let _ = tree.delete(10);
        assert!(tree.is_valid_tree());

        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        tree.insert(13, 13);
        let _ = tree.delete(14);
        assert!(tree.is_valid_tree());

        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        tree.insert(16, 16);
        let _ = tree.delete(14);
        assert!(tree.is_valid_tree());

        // delete the node with 1 red children while all the other black leafs have one red each 
        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree1();
        tree.insert(1, 1);
        tree.insert(3, 3);
        tree.insert(5, 5);
        tree.insert(7, 7);
        tree.insert(9, 9);
        tree.insert(11, 11);
        tree.insert(13, 13);
        let _ = tree.delete(14);
        assert!(tree.is_valid_tree());


    }

    #[test]
    fn test_deletion_2_2() {
        // case 2 , corner case where z is child of root
        let mut tree: RbTree<i32, i32> = RbTree::new();
        tree.insert(10, 10);
        tree.insert(5, 5);
        tree.insert(15, 15);
        tree.insert(3, 3);
        let _ = tree.delete(3);
        assert!(tree.is_valid_tree());
        // what happens when we delere 15 or 5 now? 
        /*
                 B10
                /  \
              B5    B15
         */

        // delete 15
        tree.debug = true;
        let _ = tree.delete(15);
        assert!(tree.is_valid_tree());
        // delete 5 instead, first lets restore the tree
        tree.insert(15, 15);
        tree.insert(16, 16); //making 10, 5, 15 black to prepare for the deletion of 5
        let _ = tree.delete(16);
        let _ = tree.delete(5);
        assert!(tree.is_valid_tree());
    }

    #[test]
    fn test_deletion_3() {
        //case 3 & 4, 1 of aunt's children is red

        /* Deleting 4 will result in sibling(12/w) having a red right child(10)
                    B8
                   /   \
                B4     B12
               /  \   /  \
              B2  B6 B10  R16
                          / \
                        B14 B18
                               \
                               R20
      */
      let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree2();
      let node = tree.find_node(6).unwrap();
      node.as_ref().borrow().left.as_ref().unwrap().borrow().print_information();
      println!("{:?}", tree);
      let _ = tree.delete(4);
      assert!(tree.is_valid_tree());


      /*    Deleting 18 will result in sibling(10/w) having a red left child(6)
                    B14
                   /   \
                B10     B18
                /  \    /  \
             R6   B12 B16  B20
            /  \
          B4    B8
          /
         R2
    //  */

        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree3();
        let _ = tree.delete(18);
        assert!(tree.is_valid_tree());


    }

    #[test]
    fn test_deletion_4() {
        let mut tree: RbTree<i32, i32> = RbTree::<i32, i32>::get_tree3();
        let _ = tree.delete(6);
        assert!(tree.is_valid_tree());
        let _ = tree.delete(8);
        assert!(tree.is_valid_tree());
        let _ = tree.delete(10);
        assert!(tree.is_valid_tree());
        let _ = tree.delete(18);
        assert!(tree.is_valid_tree());
        let _ = tree.delete(20);
        assert!(tree.is_valid_tree());
        let _ = tree.delete(12);
        assert!(tree.is_valid_tree());
        println!("{:?}", tree);
        assert!(tree.is_valid_tree());
    }

    #[test]
    fn test_deletion_5() {
        let mut tree: RbTree<i32, i32> = RbTree::new();
        let vec: Vec<i32> = (1..10).collect();
        for i in vec {
            tree.insert(i, i);
        }
        let _ = tree.delete(1);
        println!("{:?}", tree);
        let _ = tree.delete(2);
        println!("{:?}", tree);
        let _ = tree.delete(3);
        println!("{:?}", tree);
        let _ = tree.delete(4);
        println!("{:?}", tree);
        let _ = tree.delete(5);
        println!("{:?}", tree);
        let _ = tree.delete(6);
        println!("{:?}", tree);
        let _ = tree.delete(7);
        println!("{:?}", tree);
        let _ = tree.delete(8);
        println!("{:?}", tree);
        let _ = tree.delete(9);
        assert!(tree.is_valid_tree());
        println!("{:?}", tree);

        let mut tree: RbTree<i32, i32> = RbTree::new();
        let vec: Vec<i32> = (1..10).collect();
        for i in vec {
            tree.insert(i, i);
        }
        let _ = tree.delete(9);
        println!("{:?}", tree);
        let _ = tree.delete(8);
        println!("{:?}", tree);
        let _ = tree.delete(7);
        println!("{:?}", tree);
        let _ = tree.delete(6);
        println!("{:?}", tree);
        let _ = tree.delete(5);
        println!("{:?}", tree);
        let _ = tree.delete(4);
        println!("{:?}", tree);
        let _ = tree.delete(3);
        println!("{:?}", tree);
        let _ = tree.delete(2);
        println!("{:?}", tree);
        let _ = tree.delete(1);
        assert!(tree.is_valid_tree());
        println!("{:?}", tree);

        // load the tree again with 50 elements
        let mut tree: RbTree<i32, i32> = RbTree::new();
        let vec: Vec<i32> = (1..5550).collect();
        for i in &vec {
            tree.insert(*i, *i);
        }
        for i in vec {
            let _ = tree.delete(i);
            assert!(tree.is_valid_tree());
        }
        
    }

    #[test]
    fn test_heavy_deletion_test() {
        let mut tree: RbTree<i32, i32> = RbTree::new();
        let mut rng = thread_rng();
        let mut vec: Vec<i32> = (1..10000).collect();
        for i in &vec {
            tree.insert(*i, *i);
        }
        for i in vec {
            let _ = tree.delete(i);
        }
        assert!(tree.is_valid_tree());

        let mut tree: RbTree<i32, i32> = RbTree::new();
        let mut rng = thread_rng();
        let mut vec: Vec<i32> = (1..10000).collect();
        for i in &vec {
            tree.insert(*i, *i);
        }
        for i in 1..vec.len()/3 {
            let _ = tree.delete(((i*2) + 9) as i32);
        }
        assert!(tree.is_valid_tree());
    }

    #[test]
    fn test_random_operations() {
        use rand::seq::SliceRandom;
        let mut tree: RbTree<i32, i32> = RbTree::new();
        let mut rng = rand::thread_rng();

        // Generate a series of random numbers
        let mut vec: Vec<i32> = (1..10000).collect();
        vec.shuffle(&mut rng);

        // Insert the numbers into the tree
        for i in &vec {
            tree.insert(*i, *i);
        }

        // Shuffle the numbers again
        vec.shuffle(&mut rng);

        // Delete the numbers from the tree
        let mut j = 0; 
        for i in &vec {
            let del = tree.delete(*i);
            j = j + 1;
            if j % 100 == 0 {
                println!("Deleted: {}", j);
            }
            assert!(del.is_ok());
        }
        assert!(tree.is_valid_tree());
        //TODO: see that tree.size metadata is updated during insertion and deletion 
        assert!(tree.size == 0);
    }


}