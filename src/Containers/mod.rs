// In src/Container/mod.rs

pub mod lists {
    pub mod singly_linked_list;
    pub mod doubly_linked_list;
    pub mod skip_list;

    pub use singly_linked_list::SinglyLinkedList;
}

pub mod trees {
    pub mod binary_search_tree;  // Implementation of a binary search tree
    pub mod red_black_tree;       // Implementation of a red-black tree
    pub mod heap;                 // Implementation of a heap
    pub mod trie;                 // Implementation of a trie
    pub mod b_tree;               // Implementation of a B-tree

}

pub use lists::*;
pub use trees::*;

// Implementation of a singly linked list

