
// In omni_arsenal/src/lib.rs


// In lib.rs


// *** Container Modules ***
pub mod containers {
    pub mod lists {
        pub mod array_list;
        pub mod singly_linked_list;
        pub mod doubly_linked_list;

        pub use singly_linked_list::SinglyLinkedList;
        pub use doubly_linked_list::DoublyLinkedList;
        pub use array_list::ArrayList;
    }

    pub mod trees {
        pub mod binary_search_tree;  // Implementation of a binary search tree
        pub mod red_black_tree;       // Implementation of a red-black tree
    
        pub use binary_search_tree::BinarySearchTree;
        pub use red_black_tree::RedBlackTree;
    }

    pub mod lists_test;
    pub mod trees_test;
}

pub mod list_tests {
    pub use super::containers::lists_test::*;
}

pub mod trees_test {
    pub use super::containers::trees_test::*;
}

pub mod list {
    pub use super::containers::lists::*;
}
pub mod tree {
    pub use super::containers::trees::*;
}



// *** Algorithms Modules ***
pub mod algorithms {
    pub mod sorting {
        pub mod bubble_sort;
        pub mod insertion_sort;
        pub mod selection_sort;
        pub mod heap_sort;
        pub mod merge_sort;
        pub mod quick_sort;

        pub use bubble_sort::bubble_sort;
        pub use insertion_sort::insertion_sort;
        pub use selection_sort::selection_sort;
        pub use heap_sort::heap_sort;
        pub use merge_sort::merge_sort;
        pub use quick_sort::quick_sort;
    }

    pub mod sorting_test;
}

pub mod sorting_tests {
    pub use super::algorithms::sorting_test::*;
}

pub mod sort {
    pub use super::algorithms::sorting::*;
}


// *** Math Modules ***
pub mod math {
    pub mod algebra {
        pub mod diophantineEq;
        pub mod algebra_test;

        pub use diophantineEq::DiophantineEq;
    }

    pub mod calculus {
        pub mod calculus_test;
    }

}
pub mod algebra_tests {
    pub use super::math::algebra::algebra_test;
}
pub mod calculus_tests {
    pub use super::math::calculus::calculus_test;
}

// Any other top-level declarations or configurations go here.

#[cfg(test)]
mod tests {
    use super::list_tests;
    use super::trees_test;
    use super::sorting_tests;
    use super::algebra_tests;
    use super::calculus_tests;
    #[test]
    pub fn test_all_lists() {
        
        list_tests::list_tests::test_array_list();
        list_tests::list_tests::test_singly_linked_list();
        list_tests::list_tests::test_doubly_linked_list();
    }

    #[test]
    pub fn test_all_trees() {
        trees_test::tree_tests::test_binary_search_tree();
        trees_test::tree_tests::test_red_black_tree();
    }

    #[test]
    pub fn test_all_algorithms() {
        // mutable vector
        sorting_tests::sorting_tests::test_bubble_sort();
        sorting_tests::sorting_tests::test_insertion_sort();
        sorting_tests::sorting_tests::test_selection_sort();
        sorting_tests::sorting_tests::test_heap_sort();
        sorting_tests::sorting_tests::test_merge_sort();
        sorting_tests::sorting_tests::test_quick_sort();

    }

    #[test]
    pub fn test_all_math() {

    }

    #[test]
    pub fn test_all_calculus() {

    }
}


