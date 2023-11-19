// In src/Container/lists_test.rs
// containers/lists_test.rs

#[cfg(test)]
pub mod list_tests {
    use super::super::SinglyLinkedList;

    // SinglyLinkedList tests
    #[test]
    pub fn test_singly_linked_list() {
        push_test();
        append_test();
        pop_test();
    }




    // TESTS AREA FOR SINGLY LINKED LIST //
    #[test]
    fn push_test() {
        // Your push test implementation goes here
        let mut list = SinglyLinkedList::new_empty();
        list.push(42);

        // Add assertions to test the behavior
        assert_eq!(list.node_count, 1);
        assert_eq!(list.total_size_bytes, std::mem::size_of::<i32>());
    }

    #[test]
    fn append_test() {
        // Your append test implementation goes here
        let mut list = SinglyLinkedList::new_empty();
        list.append(42);

        // Add assertions to test the behavior
        assert_eq!(list.node_count, 1);
        assert_eq!(list.total_size_bytes, std::mem::size_of::<i32>());
    }

    #[test]

    fn pop_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.push(42);
        list.push(43);
        list.push(44);

        // Add assertions to test the behavior
        assert_eq!(list.pop(), Some(44));
        assert_eq!(list.pop(), Some(43));
        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.pop(), None);
    }

}
