
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

pub mod sort {
    pub use super::algorithms::sorting::*;
}