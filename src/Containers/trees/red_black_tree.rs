
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

use std::rc::{Rc, Weak};
 use std::cell::RefCell;
 use std::option::Option;
 use std::{cmp, str};
 use core::fmt;
 use std::fmt::Display;
 use std::fmt::Debug;    

 type RBTree<K, V> = Option<Rc<RefCell<Node<K, V>>>>; //possibly None
 type RBTreeWeak<K, V> = Option<Weak<RefCell<Node<K, V>>>>; //possibly None
 type NonNullRBTree<K, V> = Rc<RefCell<Node<K, V>>>; //not None
 type NonNullRBTreeWeak<K, V> = Weak<RefCell<Node<K, V>>>; //not None

 

 #[derive(PartialEq, Eq, Clone)]
 enum Color {
     Black,
     Red,
 }

#[derive(PartialEq, Eq, Clone)]
 enum Direction {
     Left,
     Right,
     Root,
     None,
 }

 //********** Node *****************/
struct Node<K: PartialOrd + PartialEq + Display, V> {
    key: K,
    value: V,
    color: Color,
    direction: Direction,
    left: RBTree<K, V>,
    right: RBTree<K, V>,
    parent: RBTreeWeak<K, V>,
}

impl<K: PartialOrd + PartialEq + Display, V> Node<K, V>{
    //constructors
    fn new_root(key: K, value: V) -> NonNullRBTree<K, V> {
        Rc::new(RefCell::new(Node {
            key: key,
            value: value,
            color: Color::Black,
            direction: Direction::Root,
            left: None,
            right: None,
            parent: None,
        }))
    }
    fn new_leaf(key: K, value: V, parent: NonNullRBTreeWeak<K, V>, direction: Direction) -> NonNullRBTree<K, V> {
        Rc::new(RefCell::new(Node {
            key: key,
            value: value,
            color: Color::Red,
            direction: direction,
            left: None,
            right: None,
            parent: Some(parent),
        }))
    }
    fn new_node(key: K, value: V, parent: NonNullRBTreeWeak<K, V>, direction: Direction,
                color: Color, left: RBTree<K, V>, right: RBTree<K, V>) -> NonNullRBTree<K, V> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            color,
            direction,
            left,
            right,
            parent: Some(parent),
        }))
    }

    fn new_insertion_leaf(key: K, value: V) -> NonNullRBTree<K, V> {
        Rc::new(RefCell::new(Node {
            key: key,
            value: value,
            color: Color::Red,
            direction: Direction::None,
            left: None,
            right: None,
            parent: None,
        }))
    }
    //helpers 
    fn is_left(&self) -> bool {
        self.direction == Direction::Left
    }
    fn is_right(&self) -> bool {
        self.direction == Direction::Right
    }
    fn is_root(&self) -> bool {
        self.direction == Direction::Root
    }
    fn is_red(&self) -> bool {
        self.color == Color::Red
    }
    fn is_black(&self) -> bool {
        self.color == Color::Black
    }
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
    fn left_branch(&self) -> RBTree<K, V> {
        self.left.clone() //increase reference count, doesn't deep copy
    }
    fn right_branch(&self) -> RBTree<K, V> {
        self.right.clone() //increase reference count, doesn't deep copy
    }
    /***
     * get uncle node
     * Returns an RBTree<K, V> either Some | None 
     * Throws error if parent cant be upgraded, 
     * that is if they have been dropped should not happen
     */
    fn get_aunt(&self) -> RBTree<K, V> {
        if let Some(p) = self.parent.as_ref() {
            let p = p.upgrade().expect("parent has been dropped, get_aunt() failed, RBTree");
            let p_borrow = p.borrow();
            // return p sibling
            return p_borrow.get_sibling();
        } else {
            None
        }
    }    
    /***
     * get sibling node
     * Returns an RBTree<K, V> either Some | None 
     * Throws error if parent cant be upgraded, 
     * that is if they have been dropped should not happen
     */
    fn get_sibling(&self) -> RBTree<K, V> {
        if let Some(p) = self.parent.as_ref() {
            let p = p.upgrade().expect("parent has been dropped, get_sibling() failed, RBTree");
            let p_borrow = p.borrow();
            let wanted_side = if p_borrow.is_left() {Direction::Right} else {Direction::Left};
            match wanted_side {
                Direction::Left => p_borrow.right_branch(),
                Direction::Right => p_borrow.left_branch(),
                _ => None,
            }
        } else {
            None
        }
    }

}


//********** RedBlackTree *****************/
 pub struct RedBlackTree<K: PartialOrd + PartialEq + Display + Debug, V: Clone> {
     root: RBTree<K, V>
 }


impl<K: PartialOrd + PartialEq + Display + Debug, V: Clone> RedBlackTree<K, V> {
    pub fn new_empty() -> Self {
        RedBlackTree {
            root: None,
        }
    }

    fn new_root(&self, key: K, value: V) -> NonNullRBTree<K, V> {
        return Node::new_root(key, value)
    }

    fn cmp_nodes(&self, node1: RBTree<K, V>, node2: RBTree<K, V>) -> bool {
        if node1.is_none() && node2.is_none() {
            false
        } else if node1.is_some() && node2.is_some() {
            let node1 = node1.expect("cmp_nodes() failed, RBTree");
            let node2 = node2.expect("cmp_nodes() failed, RBTree");
            let node1_borrow = node1.borrow();
            let node2_borrow = node2.borrow();
            node1_borrow.key == node2_borrow.key
        } else {
            false
        }
    }

    pub fn is_empty_tree(&self) -> bool {
        return self.root.is_none()
    } 

    fn get_parent(node: &NonNullRBTree<K, V>) -> Option<Rc<RefCell<Node<K, V>>>> {
        node.borrow().parent.as_ref()?.upgrade()
    }

    fn rotate_left(&mut self, x: NonNullRBTree<K, V>) {
        print!("rotating: {:?}\n", x.borrow().key);
        let mut x_is_root = false; 
        let y: RBTree<K, V> = x.borrow().right_branch().take();
        {
            x.borrow_mut().right = y.as_ref().unwrap().borrow().left_branch().take();
        }
        if x.borrow().right.is_some() {
            x.borrow().right_branch().as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&x));
        }
        // y.p = x.p
    
        let parent = x.borrow().parent.clone();
        if let Some(ref parent) = parent {
            if self.cmp_nodes(Some(x.clone()), Some(parent.clone().upgrade().expect("rotate_left() failed, RBTree")).unwrap().borrow().right_branch()) {
                parent.clone().upgrade().expect("rotate_left() failed, RBTree").borrow_mut().right = Some(Rc::clone(&y.as_ref().unwrap()));
            } else {
                parent.clone().upgrade().expect("rotate_left() failed, RBTree").borrow_mut().left = Some(Rc::clone(&y.as_ref().unwrap()));
            }
        } else {
            self.root = Some(Rc::clone(&y.as_ref().unwrap()));
            x_is_root = true;
        }
        unsafe {
            let y_mut = &mut *(y.as_ref().unwrap().as_ptr());
            y_mut.left = Some(x.clone());
            if x_is_root {
                y_mut.direction = Direction::Root;
            } else {
                y_mut.direction = x.borrow().direction.clone();
            }
            //y.parent = x.parent
            let x_parent = x.borrow().parent.clone();
            if let Some(ref x_parent) = x_parent {
                y_mut.parent = Some(Rc::downgrade(&x_parent.clone().upgrade().expect("rotate_left() failed, RBTree")));
            } else {
                y_mut.parent = None;
            }
        }
        unsafe {
            let x_mut = &mut *(Rc::clone(&x).as_ptr());
            x_mut.parent = Some(Rc::downgrade(&y.as_ref().unwrap()));
    
            // x is now a left child
            x_mut.direction = Direction::Left;
    
            //if x has a right child, it is now a right child , it was previously a left child
            if x_mut.right.is_some() {
                x_mut.right_branch().as_ref().unwrap().borrow_mut().direction = Direction::Right;
            }
        }
    }

    fn rotate_right(&mut self, y: NonNullRBTree<K, V>) {
        print!("rotating: {:?}\n", y.borrow().key);
        let mut y_is_root = false; 
        let x: RBTree<K, V> = y.borrow().left_branch().take();
        {
            y.borrow_mut().left = x.as_ref().unwrap().borrow().right_branch().take();
        }
        if y.borrow().left.is_some() {
            y.borrow().left_branch().as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&y));
        }
        // x.p = y.p

        let parent = y.borrow().parent.clone();
        if let Some(ref parent) = parent {
            if self.cmp_nodes(Some(y.clone()), Some(parent.clone().upgrade().expect("rotate_right() failed, RBTree")).unwrap().borrow().left_branch()) {
                parent.clone().upgrade().expect("rotate_right() failed, RBTree").borrow_mut().left = Some(Rc::clone(&x.as_ref().unwrap()));
            } else {
                parent.clone().upgrade().expect("rotate_right() failed, RBTree").borrow_mut().right = Some(Rc::clone(&x.as_ref().unwrap()));
            }
        } else {
            self.root = Some(Rc::clone(&x.as_ref().unwrap()));
            y_is_root = true;
        }
        unsafe {
            let x_mut = &mut *(x.as_ref().unwrap().as_ptr());
            x_mut.right = Some(y.clone());
            if y_is_root {
                x_mut.direction = Direction::Root;
            } else {
                x_mut.direction = y.borrow().direction.clone();
            }
            //x.parent = y.parent
            let y_parent = y.borrow().parent.clone();
            if let Some(ref y_parent) = y_parent {
                x_mut.parent = Some(Rc::downgrade(&y_parent.clone().upgrade().expect("rotate_right() failed, RBTree")));
            } else {
                x_mut.parent = None;
            }
        }
        unsafe {
            let y_mut = &mut *(Rc::clone(&y).as_ptr());
            y_mut.parent = Some(Rc::downgrade(&x.as_ref().unwrap()));

            // y is now a right child
            y_mut.direction = Direction::Right;

            //if y has a left child, it is now a left child , it was previously a right child
            if y_mut.left.is_some() {
                y_mut.left_branch().as_ref().unwrap().borrow_mut().direction = Direction::Left;
            }
        }

        // lets debug to see if everything is chained correctly, z.key, z.p.key, z.p.p.key
        
    }

    pub fn insert(&mut self, key: K, value: V) {
        //if empty tree, add root node and make black
        if self.is_empty_tree() {
            self.root = Some(self.new_root(key, value));
            return;
        }   
        let mut y: RBTree<K, V> = None;
        let mut x = self.root.clone(); // incrementing reference count
        while x.is_some() {
            y = x.clone(); // Increments rc 
            if key < x.clone().unwrap().borrow().key {
                x = x.unwrap().borrow().left_branch();
            } else {
                x = x.unwrap().borrow().right_branch();
            }
        }
        // insertion point found x is None, y is parent
        let z = Node::new_insertion_leaf(key, value);
        if y.is_none() {
            self.root = Some(z.clone());
            z.borrow_mut().color = Color::Black;
            z.borrow_mut().direction = Direction::Root;
        } else if z.borrow().key < y.clone().unwrap().borrow().key {
            y.clone().unwrap().borrow_mut().left = Some(z.clone());
            z.borrow_mut().parent = Some(Rc::downgrade(&y.clone().unwrap()));
            z.borrow_mut().direction = Direction::Left;
        } else {
            y.clone().unwrap().borrow_mut().right = Some(z.clone());
            z.borrow_mut().parent = Some(Rc::downgrade(&y.clone().unwrap()));
            z.borrow_mut().direction = Direction::Right;
        }
        // z left & right should be None due to node constructor
        // z color should be red due to node constructor
        self.rb_tree_insert_fixup(z);


    }

    
    fn rb_tree_insert_fixup(&mut self, mut z: NonNullRBTree<K, V>) {
        while z.borrow().parent.as_ref().unwrap().upgrade()
               .expect("rb_tree_insert_fixup() failed, RBTree").borrow().color == Color::Red {
            if z.borrow().parent.as_ref().unwrap().upgrade()
                .expect("rb_tree_insert_fixup() failed, RBTree").borrow().is_left() {
                // z's parent is a left-child, helps us when trying to find its aunt 
                // we know z has a parent, and while z.p is a leftchild we know it has a gp
                if z.borrow().parent.as_ref().unwrap().upgrade()
                    .expect("error in rb-insert_fix").borrow().parent.as_ref().unwrap().upgrade()
                    .expect("error in rb-insert_fix").borrow().right.as_ref().is_some() { // if uncle is red in a safe way
                    if z.borrow().parent.as_ref().unwrap().upgrade()
                        .expect("error in rb-insert_fix").borrow().parent.clone().unwrap().upgrade()
                        .expect("error in rb-insert_fix").borrow().right.clone().unwrap().borrow().is_red() {
                        // perform a color flip, we dont got any references now and can get a mutable one
                        // z.p.color = BLACK
                        z.borrow().parent.clone().unwrap().upgrade()
                                  .expect("error in rb-insert_fix").borrow_mut().color = Color::Black;
                        // y.color = BLACK
                        z.borrow().parent.clone().unwrap().upgrade()
                        .expect("error in rb-insert_fix").borrow().parent.clone().unwrap().upgrade()
                        .expect("error in rb-insert_fix").borrow().right.clone().unwrap().borrow_mut().color = Color::Black;
                        // z.p.p.color = RED
                        z.borrow().parent.clone().unwrap().upgrade()
                        .expect("error in rb-insert_fix").borrow().parent.clone().unwrap().upgrade()
                        .expect("error in rb-insert_fix").borrow_mut().color = Color::Red;
                        // z = z.p.p
                        let new_z = z.borrow().parent.clone().unwrap().upgrade()
                            .expect("error in rb-insert_fix").borrow().parent.clone().unwrap().upgrade()
                            .expect("error in rb-insert_fix");
                        z = new_z;
                    }
                } else if z.borrow().is_right() {
                    print!("left - right rotate\n");

                    let newz = z.borrow().parent.clone().unwrap().upgrade()
                                                         .expect("error in rb-insert_fix");
                    z = newz;
                    self.rotate_left(z.clone());
                } 
                print!("left - left rotate\n,  if only msg its true, otherwise could be left-rigth\n");
                //color flipzz
                let parent = z.borrow().parent.as_ref().unwrap().upgrade().expect("error in rb-insert_fix");
                parent.borrow_mut().color = Color::Black;

                let grandparent = parent.borrow().parent.as_ref().unwrap().upgrade().expect("error in rb-insert_fix");
                grandparent.borrow_mut().color = Color::Red;

                // right rotate z.p.p
                self.rotate_right(z.borrow().parent.clone().unwrap().upgrade()
                .expect("error in rb-insert_fix").borrow().parent.clone().unwrap().upgrade()
                .expect("error in rb-insert_fix").clone())
            } else {
                // z's parent is a right-child, helps us when trying to find its aunt 
                // we know z has a parent, and while z.p is a rightchild we know it has a gp
                if z.borrow().parent.clone().unwrap().upgrade()
                .expect("error in rb-insert_fix").borrow().parent.clone().unwrap().upgrade()
                .expect("error in rb-insert_fix").borrow().left.clone().is_some() { //uncle exists 
                if z.borrow().parent.clone().unwrap().upgrade()
                    .expect("error in rb-insert_fix").borrow().parent.clone().unwrap().upgrade()
                    .expect("error in rb-insert_fix").borrow().left.clone().unwrap().borrow().is_red() {
                    // perform a color flip, we dont got any references now and can get a mutable one
                    // z.p.color = BLACK
                    z.borrow().parent.clone().unwrap().upgrade()
                        .expect("error in rb-insert_fix").borrow_mut().color = Color::Black;
                    // y.color = BLACK
                    z.borrow().parent.clone().unwrap().upgrade()
                    .expect("error in rb-insert_fix").borrow().parent.clone().unwrap().upgrade()
                    .expect("error in rb-insert_fix").borrow().left.clone().unwrap().borrow_mut().color = Color::Black;
                    // z.p.p.color = RED
                    z.borrow().parent.clone().unwrap().upgrade()
                    .expect("error in rb-insert_fix").borrow().parent.clone().unwrap().upgrade()
                    .expect("error in rb-insert_fix").borrow_mut().color = Color::Red;
                    // z = z.p.p
                    let new_z = z.borrow().parent.clone().unwrap().upgrade()
                        .expect("error in rb-insert_fix").borrow().parent.clone().unwrap().upgrade()
                        .expect("error in rb-insert_fix");
                    z = new_z;
                }
                } else if z.borrow().is_left() {
                    print!("right - left rotate\n");
                    let newz = z.borrow().parent.as_ref().unwrap().upgrade()
                        .expect("error in rb-insert_fix");
                    z = newz;
                    self.rotate_right(z.clone());
                }
                print!("right - right rotate\n,  if only msg its true, otherwise could be right-left\n");

                //color flipzz
                let parent = z.borrow().parent.as_ref().unwrap().upgrade().expect("error in rb-insert_fix");
                parent.borrow_mut().color = Color::Black;

                let grandparent = parent.borrow().parent.as_ref().unwrap().upgrade().expect("error in rb-insert_fix");
                grandparent.borrow_mut().color = Color::Red;

                // left rotate z.p.p
                self.rotate_left(z.borrow().parent.as_ref().unwrap().upgrade()
                    .expect("error in rb-insert_fix").borrow().parent
                    .as_ref().unwrap().upgrade().expect("error in rb-insert_fix").clone());
            }
            
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

    fn in_order_traverse_node(node: &RBTree<K, V>, result: &mut Vec<V>) {
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

    fn pre_order_traverse_node(node: &RBTree<K, V>, result: &mut Vec<V>) {
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

    fn post_order_traverse_node(node: &RBTree<K, V>, result: &mut Vec<V>) {
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

//********** Debug *****************/
fn tree_printer_traverse_helper<K: PartialOrd + PartialEq + Display + Debug, V>(
    sb: &mut String,
    padding: &str,
    pointer: &str,
    node: &Option<Rc<RefCell<Node<K, V>>>>,
) {
    if let Some(inner) = node {
        let node = inner.borrow();
        sb.push_str(padding);
        sb.push_str(pointer);
        sb.push_str(&format!("{:?}{}:{}", node.key, if node.color == Color::Red { "R" } else { "B" }, if node.is_left() { "L" } else if node.is_right() { "R" } else { "Root" }));
        sb.push('\n');

        let padding_filler = if pointer == "└── " { "    " } else { "│   " };
        let padding = format!("{}{}", padding, padding_filler);

        let pointer_for_right = "└── ";
        let pointer_for_left = if node.right.is_some() { "├── " } else { "└── " };

        tree_printer_traverse_helper(sb, &padding, pointer_for_left, &node.left);
        tree_printer_traverse_helper(sb, &padding, pointer_for_right, &node.right);
    }
}

impl<K: PartialOrd + PartialEq + Display + Debug, V: Clone> fmt::Debug for RedBlackTree<K, V> {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut sb = String::new();
    tree_printer_traverse_helper(&mut sb, "", "", &self.root);
    write!(f, "{}", sb)
}
}

impl<K: PartialOrd + PartialEq + Display + Debug, V: Clone> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self::new_empty()
    }
}