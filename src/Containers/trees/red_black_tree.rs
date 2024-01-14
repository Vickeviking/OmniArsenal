use std::{
    rc::{Rc, Weak}, 
    cell::RefCell,
    fmt::{self, Debug},
    cmp::Ordering,
    cmp,
    mem, 
    ops::{Deref, DerefMut}, 
    cell::RefMut, 
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

// type alias
 type Tree<K, V> = Option<Rc<RefCell<Node<K, V>>>>;
 type BareTree<K, V> = Rc<RefCell<Node<K, V>>>;
 type WeakTree<K, V> = Option<Weak<RefCell<Node<K, V>>>>;

// to avoid repeating the same type signature
pub trait NodeKey: Clone + Debug + PartialOrd + Ord {}
pub trait NodeValue: Clone + Debug + PartialOrd {}
impl<T: Clone + Debug + PartialOrd + Ord> NodeKey for T {}
impl<T: Clone + Debug + PartialOrd> NodeValue for T {}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Color {
    Red,
    Black,
}

#[derive(PartialEq)]
pub enum RBOperation {
    LeftNode,
    RightNode,
}

 // Represents the direction of rotation
 #[derive(Debug, Clone, Copy)]
 pub enum Rotation {
     Left,
     Right,
 }

 /// ******************************** Node ********************************
#[derive(Clone, Debug)]
 pub struct Node<K: NodeKey, V:NodeValue> {
    key: K,
    value: V,
    color: Color,
    left: Tree<K, V>,
    right: Tree<K, V>,
    parent: WeakTree<K, V>,
 }

 impl <K: NodeKey, V:NodeValue> Node<K, V> {
    pub fn key(&self) -> &K { &self.key }
    pub fn value(&self) -> &V { &self.value }
    pub fn is_black(&self) -> bool { self.color == Color::Black }
    pub fn is_red(&self) -> bool { self.color == Color::Red }
    pub fn is_left_child(&self) -> bool {
        // return true if the node is a left child to its parent
        if let Some(parent_weak) = self.parent.as_ref() {
            if let Some(parent) = parent_weak.upgrade() {
                let parent = parent.borrow();
                if let Some(left) = &parent.left {
                    return left.borrow().key == self.key;
                }
            }
        }
        false
    }
    pub fn left(&self) -> &Tree<K, V> { &self.left }
    pub fn right(&self) -> &Tree<K, V> { &self.right }
    pub fn parent(&self) -> &WeakTree<K, V> { &self.parent }
    pub fn get_parent_color(&self) -> Color {
        if let Some(parent) = &self.parent {
            let parent = parent.upgrade().unwrap();
            let parent = parent.borrow();
            parent.color.clone()
        } else {
            Color::Black
        }
    }
    pub fn parent_is_red(&self) -> bool {
        if let Some(parent) = &self.parent {
            let parent = parent.upgrade().unwrap();
            let parent = parent.borrow();
            parent.color == Color::Red
        } else {
            false
        }
    }
    pub fn get_uncle(&self) -> Option<(Tree<K, V>, RBOperation)> {
        if let Some(parent) = &self.parent {
            let parent = parent.upgrade().unwrap();
            let parent = parent.borrow();
            if let Some(grand_parent) = &parent.parent {
                let grand_parent = grand_parent.upgrade().unwrap();
                let grand_parent = grand_parent.borrow();
                if parent.is_left_child() { // uncle is grandparent's right child
                    let uncle = grand_parent.right.clone();
                    Some((uncle, RBOperation::RightNode))
                } else { // uncle is grandparent's left child
                    let uncle = grand_parent.left.clone();
                    Some((uncle, RBOperation::LeftNode))
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn swap_color(&mut self, node2: &mut Tree<K, V>) {
        let node2 = node2.as_mut().unwrap();
        let mut node2 = node2.borrow_mut();
        mem::swap(&mut self.color, &mut node2.color);
    }

    // Modify methods
    pub fn set_value(&mut self, value: V) { self.value = value; }
    pub fn set_color(&mut self, color: Color) { self.color = color; }
    pub fn make_black(&mut self) { self.color = Color::Black; }
    pub fn make_red(&mut self) { self.color = Color::Red; }
    pub fn set_left(&mut self, left: Tree<K, V>) { self.left = left; }
    pub fn set_right(&mut self, right: Tree<K, V>) { self.right = right; }
    pub fn set_parent(&mut self, parent: WeakTree<K, V>) { self.parent = parent; }
 }

 impl<K: NodeKey, V:NodeValue> PartialOrd for Node<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
 }

 impl<K: NodeKey, V:NodeValue> Ord for Node<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
 }

 impl<K: NodeKey, V:NodeValue> PartialEq for Node<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
 }
 // marker trait that signals that its reflexive, symmetric, and transitive
 impl<K: NodeKey, V:NodeValue> Eq for Node<K, V> {}

 impl<K: NodeKey, V:NodeValue> Node<K, V> {
     pub fn new(key: K, value: V) -> Tree<K, V> {
         Some(Rc::new(RefCell::new(
            Node {
                key,
                value,
                color: Color::Red,
                left: None,
                right: None,
                parent: None,
            }
         )))
     }
     
 }

    /// ******************************** Red Black Tree ********************************
pub struct RedBlackTree<K: NodeKey, V:NodeValue> {
    root: Tree<K, V>,
    size: usize,
}

impl <K: NodeKey, V:NodeValue> RedBlackTree<K, V> {
    pub fn new() -> Self {
         RedBlackTree {
             root: None,
             size: 0,
         }
    }

    // ------------- Insert -------------
    /// Inserts a new key-value pair into the red-black tree.
    /// The tree is then fixed up to maintain red-black tree properties.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the new node to be inserted.
    /// * `value` - The value associated with the key.
    ///
    pub fn insert(&mut self, key: K, value: V) {
        self.size += 1;
        let new_node_option = Node::new(key, value);
        let mut y_option: Tree<K, V> = None;
        let mut x_option = self.root.clone(); 

        // find the right place to insert the new node
        while let Some(x) = x_option {
            y_option = Some(x.clone());
            if new_node_option.clone().unwrap().borrow().key < x.borrow().key {
                x_option = x.borrow().left.clone();
            } else {
                x_option = x.borrow().right.clone();
            }
        }
        
        // link the new node to the tree
        if let Some(y) = y_option {
            if let Some(new_node) = new_node_option.clone() {
                let mut new_node_borrowed = new_node.borrow_mut();
                new_node_borrowed.parent = Some(Rc::downgrade(&y));
            }
            let insert_right;
            let mut y = y.borrow_mut();
            let new_node = new_node_option.clone().unwrap();
            if new_node.borrow().key < y.key {
                insert_right = false;
            } else {
                insert_right = true;
            }
            if insert_right {
                y.right = new_node_option.clone();
            } else {
                y.left = new_node_option.clone();
            }
        } else {
            self.root = new_node_option.clone();
        }

        // if node doesn't have a parent, it is the root node and should be black
        if let Some(node) = &new_node_option {
            if node.borrow().parent.is_none() {
                node.borrow_mut().color = Color::Black;
            }
        }

        // fix if we dont have a grandparent
        if let Some(node) = new_node_option {
            if let Some(parent) = node.borrow().parent.clone() {
                if let Some(grand_parent) = &parent.upgrade().unwrap().borrow().parent.clone() {
                    self.fix_insert(node.clone());
                }
            }
        }
    }

    // Fix up the Red-Black Tree after insertion
    fn fix_insert(&mut self, mut k: Rc<RefCell<Node<K, V>>>) {
        
    }


   // Code for left rotate
    fn rotate_left(&mut self, x: Rc<RefCell<Node<K, V>>>) {
        
    }
    

    // Code for right rotate
    fn rotate_right(&mut self, x: Rc<RefCell<Node<K, V>>>) {
        
    }
    


   // ------------- Delete -------------

    // transplant(T, u, v)
    fn transplant(&mut self, u: BareTree<K, V>, v: Tree<K, V>) {
        unimplemented!()
        /*
        Pseudo code
        1. if u.p == T.nil
        2.   T.root = v
        3. else if u == u.p.left
        4.   u.p.left = v
        5. else u.p.right = v
        6. v.p = u.p
         */
    }

    pub fn delete(&mut self, key: K) -> Option<V> {
        unimplemented!()
        /*
        Pseudo code
        1. y = z
        2. y-original-color = y.color
        3. if z.left == T.nil
        4.   x = z.right
        5.   RB-TRANSPLANT(T, z, z.right)
        6. else if z.right == T.nil
        7.   x = z.left
        8.   RB-TRANSPLANT(T, z, z.left)
        9. else y = TREE-MINIMUM(z.right)
        10.  y-original-color = y.color
        11.  x = y.right
        12.  if y.p == z
        13.    x.p = y
        14.  else RB-TRANSPLANT(T, y, y.right)
        15.    y.right = z.right
        16.    y.right.p = y
        17.  RB-TRANSPLANT(T, z, y)
        18.  y.left = z.left
        19.  y.left.p = y
        20.  y.color = z.color
        21. if y-original-color == BLACK
        22.  RB-DELETE-FIXUP(T, x)
         */ 
    }

    fn delete_fixup(&mut self, node: BareTree<K, V>) {
        unimplemented!()
        /*
        Pseudo code
        1. while x != T.root and x.color == BLACK
        2.   if x == x.p.left
        3.     w = x.p.right
        4.     if w.color == RED
        5.       w.color = BLACK
        6.       x.p.color = RED
        7.       LEFT-ROTATE(T, x.p)
        8.       w = x.p.right
        9.     if w.left.color == BLACK and w.right.color == BLACK
        10.      w.color = RED
        11.      x = x.p
        12.    else if w.right.color == BLACK
        13.      w.left.color = BLACK
        14.      w.color = RED
        15.      RIGHT-ROTATE(T, w)
        16.      w = x.p.right
        17.    w.color = x.p.color
        18.    x.p.color = BLACK
        19.    w.right.color = BLACK
        20.    LEFT-ROTATE(T, x.p)
        21.    x = T.root
        22.  else (same as then clause with "right" and "left" exchanged)
        23. x.color = BLACK
        */
    }



    // ------------- Searches -------------
    pub fn find(&self, key: K) -> Option<V> {
        self.find_r(
            &self.root,
            &key,
        )
    }

    fn find_r(&self, node: &Tree<K, V>, key: &K) -> Option<V> {
        if let Some(n) = node {
            let n = n.borrow();
            match key.cmp(&n.key) {
                Ordering::Less => self.find_r(&n.left, key),
                Ordering::Greater => self.find_r(&n.right, key),
                Ordering::Equal => Some(n.value.clone()),
            }
        } else {
            None
        }
    }

    //  ------------- Traverse -------------
    pub fn in_order_traversal(&self) -> Vec<V> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            Self::in_order_traverse_node(&Some(root.to_owned()), &mut result);
        }
        result
    }

    fn in_order_traverse_node(node: &Tree<K, V>, result: &mut Vec<V>) {
        if let Some(n) = node {
            if let Some(ref left) = &n.borrow().left {
                Self::in_order_traverse_node(&Some(left.to_owned()), result);
            }
            result.push(n.borrow().value.clone());
            if let Some(ref right) = &n.borrow().right {
                Self::in_order_traverse_node(&Some(right.to_owned()), result);
            }
        }
    }

    pub fn pre_order_traversal(&self) -> Vec<V> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            Self::pre_order_traverse_node(&Some(root.to_owned()), &mut result);
        }
        result
    }

    fn pre_order_traverse_node(node: &Tree<K, V>, result: &mut Vec<V>) {
        if let Some(n) = node {
            result.push(n.borrow().value.clone());
            if let Some(ref left) = &n.borrow().left {
                Self::pre_order_traverse_node(&Some(left.to_owned()), result);
            }
            if let Some(ref right) = &n.borrow().right {
                Self::pre_order_traverse_node(&Some(right.to_owned()), result);
            }
        }
    }

    pub fn post_order_traversal(&self) -> Vec<V> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            Self::post_order_traverse_node(&Some(root.to_owned()), &mut result);
        }
        result
    }

    fn post_order_traverse_node(node: &Tree<K, V>, result: &mut Vec<V>) {
        if let Some(n) = node {
            if let Some(ref left) = &n.borrow().left {
                Self::post_order_traverse_node(&Some(left.to_owned()), result);
            }
            if let Some(ref right) = &n.borrow().right {
                Self::post_order_traverse_node(&Some(right.to_owned()), result);
            }
            result.push(n.borrow().value.clone());
        }
    }

    // ------------- Debug -------------
    pub fn is_a_valid_red_black_tree(&self) -> Result<(), &'static str> {
        self.validate_tree(&self.root, None, Color::Black, 0).map(|_| ())
    }
    
    fn validate_tree(
        &self,
        node: &Option<Rc<RefCell<Node<K, V>>>>,
        parent_key: Option<&K>,
        parent_color: Color,
        black_height: usize,
    ) -> Result<usize, &'static str> {
        if let Some(inner) = node {
            let node = inner.borrow();
            
            // Property 1: Every node is either red or black.
            if node.color != Color::Red && node.color != Color::Black {
                return Err("Red-Black Property 1 Violation");
            }
    
            // Property 2: The root is black.
            if parent_key.is_none() && node.color != Color::Black {
                return Err("Red-Black Property 2 Violation");
            }
    
            // Property 3: Red nodes have black children.
            if parent_color == Color::Red && node.color == Color::Red {
                return Err("Red-Black Property 3 Violation");
            }
    
            // Property 4: All paths from any node to its leaves have the same black height.
            let mut black_height_left = black_height;
            let mut black_height_right = black_height;
    
            if node.color == Color::Black {
                black_height_left += 1;
                black_height_right += 1;
            }
    
            // Recursive validation for left and right subtrees.
            let left_result = self.validate_tree(&node.left, Some(&node.key), node.color.clone(), black_height_left)?;
            let right_result = self.validate_tree(&node.right, Some(&node.key), node.color.clone(), black_height_right)?;
    
            // Property 4 validation
            if left_result != right_result {
                return Err("Red-Black Property 4 Violation");
            }
    
            // Return black height for the current subtree.
            Ok(left_result)
        } else {
            // For an empty subtree, return black height.
            Ok(black_height + 1)
        }
    }

 }

 fn tree_printer_traverse_helper<K: NodeKey, V: NodeValue>(
    sb: &mut String,
    padding: &str,
    pointer: &str,
    node: &Option<Rc<RefCell<Node<K, V>>>>,
) {
    if let Some(inner) = node {
        let node = inner.borrow();
        sb.push_str(padding);
        sb.push_str(pointer);
        sb.push_str(&format!("{:?}{}:{}", node.key, if node.color == Color::Red { "R" } else { "B" }, if node.is_left_child() { "L" } else { "R" }));
        sb.push('\n');

        let padding_filler = if pointer == "└── " { "    " } else { "│   " };
        let padding = format!("{}{}", padding, padding_filler);

        let pointer_for_right = "└── ";
        let pointer_for_left = if node.right.is_some() { "├── " } else { "└── " };

        tree_printer_traverse_helper(sb, &padding, pointer_for_left, &node.left);
        tree_printer_traverse_helper(sb, &padding, pointer_for_right, &node.right);
    }
}

impl<K: NodeKey, V: NodeValue> fmt::Debug for RedBlackTree<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sb = String::new();
        tree_printer_traverse_helper(&mut sb, "", "", &self.root);
        write!(f, "{}", sb)
    }
}

impl<K: NodeKey, V: NodeValue> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}