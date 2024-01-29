/***
 *  Binary Search Tree
 *  Sorted
 *  Left < Root < Right
 *  O(log n) for search, insert, delete
 *  O(n) for traversal
 * 
 * Upsides:
 * - Simple implementation
 * - Efficient and fast search
 * - Traversal allows for different orderings
 * - Great for large amount of unsorted data
 * 
 * Downsides:
 * - Worst-case performans is that of linked list
 * - Unbalanced trees are easy to create
 * - Unbalanced trees cant be repaired without rebuilding
 * - Recursive algos can cause stack overflower when unbalanced.
 */
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type NodeLink<T> = Option<Rc<RefCell<Node<T>>>>;


#[derive(Debug, PartialEq)]
pub struct Node<T> {
    data: T,
    left: NodeLink<T>,
    right: NodeLink<T>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

#[derive(PartialEq)]
pub struct BinarySearchTree<T: PartialOrd + Default + Clone + Debug> {
    pub root: Link<T>,
}

impl<T: PartialOrd + Default + Clone + Debug> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
        }
    }
    pub fn insert(&mut self, data: T) {
        let node = Node {
            data,
            left: None,
            right: None,
        };
        if self.root.is_none() {
            self.root = Some(Rc::new(RefCell::new(node)));
        } else {
            Self::insert_node(&mut self.root, node.data);
        }
    }

    // recursive helper function for insert
    fn insert_node(node: &mut NodeLink<T>, data: T) {
        if let Some(n) = node {
            // Check if the current node is a leaf
            if n.borrow().is_leaf() {
                // Insert the value to the left if it's smaller, otherwise insert to the right
                if data < n.borrow().data {
                    n.borrow_mut().left = Some(Rc::new(RefCell::new(Node::new(data))));
                } else {
                    n.borrow_mut().right = Some(Rc::new(RefCell::new(Node::new(data))));
                }
            } else {
                // Traverse further to find the appropriate position for the value
                if data < n.borrow().data {
                    // lower than current node, go left
                    // if left is None, insert here
                    if n.borrow().left.is_none() {
                        n.borrow_mut().left = Some(Rc::new(RefCell::new(Node::new(data))));
                    } else {
                        // otherwise, keep traversing
                        Self::insert_node(&mut n.borrow_mut().left, data);
                    }
                } else {
                    // higher than current node, go right
                    // if right is None, insert here
                    if n.borrow().right.is_none() {
                        n.borrow_mut().right = Some(Rc::new(RefCell::new(Node::new(data))));
                    } else {
                        // otherwise, keep traversing
                        Self::insert_node(&mut n.borrow_mut().right, data);
                    }
                }
            }
        } else {
            panic!("Node doesn't exist");
        }
    }

    pub fn search(&self, value: T) -> bool {
        Self::search_node(&self.root, value)
    }

    /// Searches for a node with the specified target data in a binary search tree.
    /// # Arguments
    /// * `node` - A reference to the root node of the binary search tree.
    /// * `target_data` - The target data to search for in the binary search tree.
    /// # Returns
    /// Returns `true` if a node with the target data is found in the binary search tree, otherwise `false`.
    fn search_node(node: &NodeLink<T>, target_data: T) -> bool {
        if let Some(n) = node {
            if target_data == n.borrow().data {
                true
            } else if target_data < n.borrow().data {
                Self::search_node(&n.borrow().left, target_data)
            } else {
                Self::search_node(&n.borrow().right, target_data)
            }
        } else {
            false
        }
    }

    // Finds node, its parent, and whether it's a right or left child
    fn delete_helper(
        target_data: T,
        mut node: NodeLink<T>,
        mut parent: NodeLink<T>,
        mut is_right: bool,
    ) -> (NodeLink<T>, NodeLink<T>, bool) {
        while let Some(n) = node {
            if target_data == n.borrow().data {
                return (Some(n.clone()), parent, is_right);
            } else if target_data < n.borrow().data {
                parent = Some(n.clone());
                node = n.borrow().left.clone();
            } else {
                parent = Some(n.clone());
                node = n.borrow().right.clone();
                is_right = true;
            }
        }

        (None, None, false)
    }

    fn find_min(node: NodeLink<T>) -> NodeLink<T> {
        let mut node = node;
        while let Some(n) = node {
            if n.borrow().left.is_none() {
                return Some(n.clone());
            }
            node = n.borrow().left.clone();
        }
        None
    }

    // Returns the number of children a node has
    fn num_children(node: NodeLink<T>) -> usize {
        let mut count = 0;
        if let Some(n) = node {
            if n.borrow().left.is_some() {
                count += 1;
            }
            if n.borrow().right.is_some() {
                count += 1;
            }
        }
        count
    }

    // Deletes a node with no children or one child
    fn delete_max_one_child_node(node: NodeLink<T>, parent: NodeLink<T>, is_right: bool, num_child_nodes: usize) -> T {
        if let Some(n) = node {
           if let Some(p) = parent {
            if num_child_nodes == 0 { // leaf node
                if is_right {
                    p.borrow_mut().right = None;
                } else {
                    p.borrow_mut().left = None;
                }
            } else if num_child_nodes == 1 { // node with one child
                if is_right {
                    p.borrow_mut().right = if n.borrow().left.is_some() {
                        n.borrow().left.clone()
                    } else {
                        n.borrow().right.clone()
                    };
                } else {
                    p.borrow_mut().left = if n.borrow().left.is_some() {
                        n.borrow().left.clone()
                    } else {
                        n.borrow().right.clone()
                    };
                }
            }
           } else {
             panic!("root node should have been deleted earlier");
           }
            // return the data
            n.borrow().data.clone()

        } else {
            panic!("node doesn't exist")
        }
    }

    fn delete_2children_node(node: NodeLink<T>, _parent: NodeLink<T>, _is_right: bool) -> T {
        // find presuccessor
        let presuccessor = Self::find_min(node.clone().as_ref().unwrap().borrow().right.clone());
        // take the presuccessor's data, swap it with the node's data
        let presuccessor_data = presuccessor.clone().unwrap().borrow().data.clone();
        let node_data = node.clone().as_ref().unwrap().borrow().data.clone(); // save node's data
        node.clone().unwrap().borrow_mut().data = presuccessor_data.clone(); // overwrite node's data with presuccessor's data
        presuccessor.clone().as_ref().unwrap().borrow_mut().data = node_data.clone(); // overwrite presuccessor's data with node's data
        // delete the presuccessor node as a node with 0 or 1 children
        Self::delete_max_one_child_node(presuccessor.clone(), node.clone(), true, Self::num_children(presuccessor.clone()))

    }

    pub fn delete(&mut self, data: T) -> Option<T> {
        let (node, parent, is_right) = Self::delete_helper( data, self.root.clone(), None, false);
        let num_child_nodes = Self::num_children(node.clone());
        if parent.is_none() && num_child_nodes <= 1 {
            let node_data = node.clone().unwrap().borrow().data.clone();
            self.root = if node.clone().unwrap().borrow().left.is_some() {
                node.clone().unwrap().borrow().left.clone()
            } else {
                node.clone().unwrap().borrow().right.clone()
            };
            return Some(node_data);
        }
        match num_child_nodes {
            0 | 1 => Some(Self::delete_max_one_child_node( node, parent, is_right, num_child_nodes)),
            2 => Some(Self::delete_2children_node(node, parent, is_right)),
            _ => None,
        }
    }

    // traversals 
    pub fn in_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            Self::in_order_traverse_node(&Some(root.to_owned()), &mut result);
        }
        result
    }

    fn in_order_traverse_node(node: &NodeLink<T>, result: &mut Vec<T>) {
        if let Some(n) = node {
            if let Some(ref left) = &n.borrow().left {
                Self::in_order_traverse_node(&Some(left.to_owned()), result);
            }
            result.push(n.borrow().data.clone());
            if let Some(ref right) = &n.borrow().right {
                Self::in_order_traverse_node(&Some(right.to_owned()), result);
            }
        }
    }

    pub fn pre_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            Self::pre_order_traverse_node(&Some(root.to_owned()), &mut result);
        }
        result
    }

    fn pre_order_traverse_node(node: &NodeLink<T>, result: &mut Vec<T>) {
        if let Some(n) = node {
            result.push(n.borrow().data.clone());
            if let Some(ref left) = &n.borrow().left {
                Self::pre_order_traverse_node(&Some(left.to_owned()), result);
            }
            if let Some(ref right) = &n.borrow().right {
                Self::pre_order_traverse_node(&Some(right.to_owned()), result);
            }
        }
    }

    pub fn post_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            Self::post_order_traverse_node(&Some(root.to_owned()), &mut result);
        }
        result
    }

    fn post_order_traverse_node(node: &NodeLink<T>, result: &mut Vec<T>) {
        if let Some(n) = node {
            if let Some(ref left) = &n.borrow().left {
                Self::post_order_traverse_node(&Some(left.to_owned()), result);
            }
            if let Some(ref right) = &n.borrow().right {
                Self::post_order_traverse_node(&Some(right.to_owned()), result);
            }
            result.push(n.borrow().data.clone());
        }
    }

    fn tree_printer_traverse_helper(sb: &mut String, padding: &str, pointer: &str, node: &Option<Rc<RefCell<Node<T>>>>) {
        if let Some(inner) = node {
            let node = inner.borrow();
            sb.push_str(padding);
            sb.push_str(pointer);
            sb.push_str(&format!("{:?}", node.data));
            sb.push('\n');
    
            let padding_filler = if pointer == "└── " { "    " } else { "│   " };
            let padding = format!("{}{}", padding, padding_filler);
    
            let pointer_for_right = "└── ";
            let pointer_for_left = if node.right.is_some() { "├── " } else { "└── " };
    
            Self::tree_printer_traverse_helper(sb, &padding, pointer_for_left, &node.left);
            Self::tree_printer_traverse_helper(sb, &padding, pointer_for_right, &node.right);
        }
    }
    
    pub fn balance(&mut self) {
        let sorted_vec = self.in_order_traversal();
        self.root = None;
        let new_root = Self::balance_helper(&sorted_vec, 0, sorted_vec.len() - 1);
        self.root = new_root;
    }
    
    fn balance_helper(sorted_vec: &[T], start: usize, end: usize) -> Link<T> {
        if start > end {
            None
        } else {
            let mid = (start + end) / 2;
            let mut new_node = Node::new(sorted_vec[mid].clone());
            
            // Check if mid is greater than zero before subtracting 1
            if mid > 0 {
                new_node.left = Self::balance_helper(sorted_vec, start, mid - 1);
            }
            
            new_node.right = Self::balance_helper(sorted_vec, mid + 1, end);
            Some(Rc::new(RefCell::new(new_node)))
        }
    }

}

impl<T: fmt::Debug + PartialOrd + Default + Clone> fmt::Debug for BinarySearchTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sb = String::new();
        Self::tree_printer_traverse_helper(&mut sb, "", "", &self.root);
        write!(f, "{}", sb)
    }
}

impl<T: PartialOrd + Default + Clone + Debug> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}





// ******************  tests *************************

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn bst_insert() {
        /***
         *    5
         *  3   7
         * 2 4 6 8
         */
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);

        let pre_order_vec: Vec<i32> = bst.in_order_traversal();
        assert_eq!(pre_order_vec, vec![2, 3, 4, 5, 6, 7, 8]);


        // a bigger tree

        /***
         *         ______15______
         *        /              \
         *     __10__            __20__
         *    /      \           /      \
         *    5       12       17       25
         *           /  \     /  \       \
         *           11  14  16  19      30
         */

        let mut bst = BinarySearchTree::new();
        bst.insert(15);
        bst.insert(10);
        bst.insert(20);
        bst.insert(5);
        bst.insert(12);
        bst.insert(17);
        bst.insert(25);
        bst.insert(11);
        bst.insert(14);
        bst.insert(16);
        bst.insert(19);
        bst.insert(30);

        let pre_order_vec: Vec<i32> = bst.in_order_traversal();
        assert_eq!(pre_order_vec, vec![5, 10, 11, 12, 14, 15, 16, 17, 19, 20, 25, 30]);

    }

    #[test]
    fn bst_traversal() {
        /***
         *    5
         *  3   7
         * 2 4 6 8
         */
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);

        let pre_order_vec: Vec<i32> = bst.in_order_traversal();
        assert_eq!(pre_order_vec, vec![2, 3, 4, 5, 6, 7, 8]);

        let pre_order_vec: Vec<i32> = bst.pre_order_traversal();
        assert_eq!(pre_order_vec, vec![5, 3, 2, 4, 7, 6, 8]);

        let pre_order_vec: Vec<i32> = bst.post_order_traversal();
        assert_eq!(pre_order_vec, vec![2, 4, 3, 6, 8, 7, 5]);
    }

    #[test]
    fn bst_search() {
        /***
         *    5
         *  3   7
         * 2 4 6 8
         */
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);
        assert_eq!(bst.search(6), true);
        assert_eq!(bst.search(8), true);
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(9), false);
    }

    #[test]
    fn delete_root() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10);
        assert_eq!(bst.delete(10), Some(10));
        assert_eq!(bst.root, None);
    }

    #[test]
    fn delete_leaf() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        assert_eq!(bst.delete(5), Some(5));
        // Check that 5 is no longer in the tree
    }

    #[test]
    fn del_n_w1_child() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(12);
        assert_eq!(bst.delete(15), Some(15));
        // Check that 15 is replaced with 12 in the tree
    }

    #[test]
    fn del_n_w2_child() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(12);
        bst.insert(20);
        assert_eq!(bst.delete(15), Some(15));
        assert_eq!(bst.search(10), true);
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(15), false); // 15 should be deleted
        assert_eq!(bst.search(12), true);
        assert_eq!(bst.search(20), true);
    }

    #[test]
    fn del_in_large_t() {
        let mut bst = BinarySearchTree::new();
        for i in 1..100 {
            bst.insert(i);
        }
        assert_eq!(bst.delete(50), Some(50));
        // Check that 50 is no longer in the tree
        assert_eq!(bst.search(50), false);
        // Check that all other nodes are still in the tree
        for i in 1..100 {
            if i != 50 {
                assert_eq!(bst.search(i), true);
            }
        }
    }

    #[test]
    fn bst_balance() {
        let mut bst = BinarySearchTree::new();
        for i in 1..25 {
            bst.insert(i);
        }
        assert_eq!(bst.delete(10), Some(10));
        // Check that 50 is no longer in the tree
        assert_eq!(bst.search(10), false);
        // Check that all other nodes are still in the tree
        for i in 1..25 {
            if i != 10 {
                assert_eq!(bst.search(i), true);
            }
        }

        bst.balance();

        assert_eq!(bst.search(10), false);
        // Check that all other nodes are still in the tree
        for i in 1..25 {
            if i != 10 {
                assert_eq!(bst.search(i), true);
            }
        }
    }

    #[test]
    fn test_iterator() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(8);
        tree.insert(2);
        tree.insert(4);
        tree.insert(7);
        tree.insert(9);

        let result: Vec<_> = tree.in_order_traversal();
        assert_eq!(result, vec![2, 3, 4, 5, 7, 8, 9]);
    }
}