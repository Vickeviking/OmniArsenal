// In src/Container/trees_test.rs

#[cfg(test)]
pub mod tree_tests {
    use super::super::binary_search_tree;

    pub fn test_binary_search_tree() {
        test_bst_insert();
        test_bst_traversal();
        test_bst_search();
        test_delete_root();
        test_delete_leaf();
        test_delete_node_with_one_child();
        test_delete_node_with_two_children();
        test_delete_node_in_large_tree();

    }

    // TEST AREA FOR BST
    #[test]
    fn test_bst_insert() {
        /***
         *    5
         *  3   7
         * 2 4 6 8
         */
        let mut bst = binary_search_tree::BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);

        let pre_order_vec: Vec<i32> = bst.in_order_traversal();
        dbg!(bst);
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

        let mut bst = binary_search_tree::BinarySearchTree::new();
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
    fn test_bst_traversal() {
        /***
         *    5
         *  3   7
         * 2 4 6 8
         */
        let mut bst = binary_search_tree::BinarySearchTree::new();
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
    fn test_bst_search() {
        /***
         *    5
         *  3   7
         * 2 4 6 8
         */
        let mut bst = binary_search_tree::BinarySearchTree::new();
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
    fn test_delete_root() {
        let mut bst = binary_search_tree::BinarySearchTree::new();
        bst.insert(10);
        assert_eq!(bst.delete(10), 10);
        assert_eq!(bst.root, None);
    }

    #[test]
    fn test_delete_leaf() {
        let mut bst = binary_search_tree::BinarySearchTree::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        assert_eq!(bst.delete(5), 5);
        // Check that 5 is no longer in the tree
    }

    #[test]
    fn test_delete_node_with_one_child() {
        let mut bst = binary_search_tree::BinarySearchTree::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(12);
        assert_eq!(bst.delete(15), 15);
        // Check that 15 is replaced with 12 in the tree
    }

    #[test]
    fn test_delete_node_with_two_children() {
        let mut bst = binary_search_tree::BinarySearchTree::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(12);
        bst.insert(20);
        assert_eq!(bst.delete(15), 15);
        assert_eq!(bst.search(10), true);
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(15), false); // 15 should be deleted
        assert_eq!(bst.search(12), true);
        assert_eq!(bst.search(20), true);
    }

    #[test]
    fn test_delete_node_in_large_tree() {
        let mut bst = binary_search_tree::BinarySearchTree::new();
        for i in 1..100 {
            bst.insert(i);
        }
        assert_eq!(bst.delete(50), 50);
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
    fn test_bst_balance() {
        let mut bst = binary_search_tree::BinarySearchTree::new();
        for i in 1..25 {
            bst.insert(i);
        }
        assert_eq!(bst.delete(10), 10);
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
}
    


