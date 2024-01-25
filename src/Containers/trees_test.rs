// In src/Container/trees_test.rs
use rand::seq::SliceRandom; //shuffle a vector
use rand::Rng;

#[cfg(test)]
pub mod tree_tests {

    use rand::seq::SliceRandom;
    use red_black_tree::RedBlackTree;

    use super::super::super::containers::trees::binary_search_tree;
    use super::super::super::containers::trees::red_black_tree;

    pub fn test_binary_search_tree() {
        test_bst_insert();
        test_bst_traversal();
        test_bst_search();
        test_delete_root();
        test_delete_leaf();
        test_delete_node_with_one_child();
        test_delete_node_with_two_children();
        test_delete_node_in_large_tree();
        test_bst_balance();
        // test_iterator();
        // test_into_iterator();
        // test_iterator_rev();
        // test_into_iterator_rev();
    }

    pub fn test_red_black_tree() {
        test_red_black_tree_insert();
        test_red_black_tree_left_right_rotation();
        test_red_black_tree_right_left_rotation();
        test_red_black_tree_right_rotation();
        test_red_black_tree_left_rotation();
        test_red_black_tree_many_rotations();
        test_red_black_tree_heavy_random();
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
        assert_eq!(bst.delete(10), Some(10));
        assert_eq!(bst.root, None);
    }

    #[test]
    fn test_delete_leaf() {
        let mut bst = binary_search_tree::BinarySearchTree::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        assert_eq!(bst.delete(5), Some(5));
        // Check that 5 is no longer in the tree
    }

    #[test]
    fn test_delete_node_with_one_child() {
        let mut bst = binary_search_tree::BinarySearchTree::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(12);
        assert_eq!(bst.delete(15), Some(15));
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
        assert_eq!(bst.delete(15), Some(15));
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
    fn test_bst_balance() {
        let mut bst = binary_search_tree::BinarySearchTree::new();
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
        let mut tree = binary_search_tree::BinarySearchTree::new();
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


    // TEST AREA FOR RED BLACK TREE

    #[test]
    fn test_red_black_tree_insert() {
        let mut rb_tree: RedBlackTree<i32, i32> = red_black_tree::RedBlackTree::new_empty();
        rb_tree.insert(2, 2);
        rb_tree.insert(1, 1);
        rb_tree.insert(3, 3);
        
        assert!(rb_tree.is_a_valid_red_black_tree().is_ok());
        print!("{:?} ", rb_tree);

    }

    #[test]
    fn test_red_black_tree_left_right_rotation() {
         
        let mut rb_tree: RedBlackTree<i32, i32> = red_black_tree::RedBlackTree::new_empty();
        rb_tree.insert(10, 10);
        rb_tree.insert(4, 4);
        rb_tree.insert(8, 8);
        
        assert!(rb_tree.is_a_valid_red_black_tree().is_ok());
        print!("{:?} ", rb_tree);
        
    }

    #[test]
    fn test_red_black_tree_right_left_rotation() {
         
        let mut rb_tree: RedBlackTree<i32, i32> = red_black_tree::RedBlackTree::new_empty();
        rb_tree.insert(5, 5);
        rb_tree.insert(10, 10);
        rb_tree.insert(8, 8);
        
        assert!(rb_tree.is_a_valid_red_black_tree().is_ok());
        print!("{:?} ", rb_tree); 

        // bigger tree with no null nodes
        //                              20
        //                      10              30
        //                  5       15      25      35
        //                                22  27  32  37

        let mut rb_tree: RedBlackTree<i32, i32> = red_black_tree::RedBlackTree::new_empty();

        rb_tree.insert(20, 20);
        rb_tree.insert(10, 10);
        rb_tree.insert(30, 30);
        rb_tree.insert(5, 5);
        rb_tree.insert(15, 15);
        rb_tree.insert(25, 25);
        rb_tree.insert(35, 35);
        rb_tree.insert(22, 22);
        rb_tree.insert(27, 27);
        rb_tree.insert(32, 32);
        rb_tree.insert(37, 37);

        assert!(rb_tree.is_a_valid_red_black_tree().is_ok());
        print!("{:?} ", rb_tree);
        
    }

    #[test]
    fn test_red_black_tree_right_rotation() {
        
        let mut rb_tree: RedBlackTree<i32, i32> = red_black_tree::RedBlackTree::new_empty();
        rb_tree.insert(3, 3);
        rb_tree.insert(2, 2);
        rb_tree.insert(1, 1);
        
        assert!(rb_tree.is_a_valid_red_black_tree().is_ok());
        print!("{:?} ", rb_tree);        
        
    }

    #[test]
    fn test_red_black_tree_left_rotation() {
        
        let mut rb_tree: RedBlackTree<i32, i32> = red_black_tree::RedBlackTree::new_empty();
        rb_tree.insert(1, 1);
        rb_tree.insert(2, 2);
        rb_tree.insert(3, 3);
        
        assert!(rb_tree.is_a_valid_red_black_tree().is_ok());
        print!("{:?} ", rb_tree);
        
    }

    #[test]
    fn test_red_black_tree_many_rotations() {

        // 1-1000
        let mut rb_tree: RedBlackTree<i32, i32> = red_black_tree::RedBlackTree::new_empty();
        for i in 1..1000 {
            rb_tree.insert(i, i);
        }
        assert!(rb_tree.is_a_valid_red_black_tree().is_ok());

        // 1000-1
        let mut rb_tree: RedBlackTree<i32, i32> = red_black_tree::RedBlackTree::new_empty();
        for i in (1..1000).rev() {
            rb_tree.insert(i, i);
        }
        assert!(rb_tree.is_a_valid_red_black_tree().is_ok());        

    }

    #[test]
    fn test_red_black_tree_heavy_random() {
            
        // lets make a list of 1000 non random numbers
        let mut rng = rand::thread_rng();
        let mut vec: Vec<i32> = Vec::new();
        for i in 1..1000 {
            vec.push(i);
        }
        // shuffle the list
        vec.shuffle(&mut rng);

        // insert the shuffled list into the tree
        let mut rb_tree: RedBlackTree<i32, i32> = red_black_tree::RedBlackTree::new_empty();
        for i in vec {
            rb_tree.insert(i, i);
        }
        assert!(rb_tree.is_a_valid_red_black_tree().is_ok());




    }

    #[test]
    fn test_red_black_tree_functional() {
        // insert 1mil random numbers
        // lets make a list of 1mil non random numbers
        let mut rng = rand::thread_rng();
        let mut vec: Vec<i32> = Vec::new();
        for i in 1..1000000 {
            vec.push(i);
        }
        // shuffle the list
        vec.shuffle(&mut rng);

        // insert the shuffled list into the tree
        let mut rb_tree: RedBlackTree<i32, i32> = red_black_tree::RedBlackTree::new_empty();
        for i in vec {
            rb_tree.insert(i, i);
        }
        assert!(rb_tree.is_a_valid_red_black_tree().is_ok());

    }
}
    


