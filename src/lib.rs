// *** Container Modules ***
pub mod containers {
    pub mod lists;
    pub mod trees;
    pub mod stacks;
    pub mod queues;
}

// *** Algorithms Modules ***
pub mod algorithms {
    pub mod sorting;
}

// Re-export for easier access
pub use containers::{
    lists::{
        self, 
        array_list::{ArrayList, ArrayListIterator}, 
        doubly_linked_list::{DoublyLinkedList, DoublyLinkedListIterator}, 
        singly_linked_list::{SinglyLinkedList, SinglyLinkedListIterator}
    },
    trees::{
        self, 
        binary_search_tree::{BinarySearchTree}, 
        red_black_tree::{}
    },
    stacks::{
        self, 
        stack::{Stack}
    },
    queues::{
        self, 
        queue::{Queue}, 
        priority_queue::{PriorityQueue}
    }

};

pub use algorithms::{
    sorting::{
        self, 
        bubble_sort::bubble_sort, 
        heap_sort::heap_sort, 
        insertion_sort::insertion_sort, 
        merge_sort::merge_sort, 
        quick_sort::quick_sort, 
        selection_sort::selection_sort
    },
};
