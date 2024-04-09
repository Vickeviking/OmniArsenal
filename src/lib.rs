// *** Container Modules ***
pub mod containers {
    pub mod lists;
    pub mod trees;
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
        binary_search_tree::BinarySearchTree, 
        red_black_tree::RedBlackTree,
        binary_heap::BinaryHeap
    }

};

pub use algorithms::{
    sorting::{
        self, 
        bubble_sort::bubblesort, 
        heap_sort::heapsort, 
        insertion_sort::insertionsort, 
        merge_sort::mergesort, 
        quick_sort::quicksort, 
        selection_sort::selectionsort
    },
};
