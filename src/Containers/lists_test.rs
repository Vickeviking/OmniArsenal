// In src/Container/lists_test.rs
// containers/lists_test.rs

#[cfg(test)]
pub mod list_tests {
    use std::ops::Range;

    use super::super::SinglyLinkedList;
    use super::super::ArrayList;

    // SinglyLinkedList tests

    #[test]
    pub fn test_array_list() {
        array_list_append_test();
        array_growth_test();
    }

    #[test]
    pub fn test_singly_linked_list() {
        push_test();
        append_test();
        pop_test();
        peek_test();
        peek_tail_test();
        is_empty_test();
        clear_test();
        iterator_test();
        formatting_test();
        big_list_test();
    }

    // TEST AREA FOR ARRAY LIST STARTS //

    #[test]
    fn array_list_append_test() {
        let mut arr = ArrayList::<i32>::new();
        assert_eq!(arr.length, 0);
        arr.append(42);
        assert_eq!(arr.length, 1);
    }

    #[test]
    fn array_growth_test() {
        let mut arr = ArrayList::<i32>::new();

        // should force a grow to 10
        arr.append(1);
        arr.append(2);
        arr.append(3);
        arr.append(4);
        arr.append(5);

        // should force a grow to 20
        arr.append(-1);
        arr.append(-2);
        arr.append(-3);
        arr.append(-4);
        arr.append(-5);
        arr.append( -6);

        //should force a grow to 40
        for i in 0..10 {
            arr.append(i);
        }




    }

    // TEST AREA FOR ARRAY LIST ENDS //


    // TESTS AREA FOR SINGLY LINKED LIST STARTS //
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
    #[test]
    fn peek_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.push(42);

        // Add assertions to test the behavior
        assert_eq!(list.peek(), Some(42));
    }

    #[test]
    fn peek_tail_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.push(42);
        list.push(43);
        list.push(44);

        // Add assertions to test the behavior
        assert_eq!(list.peek_tail(), Some(42));
    }

    #[test]

    fn is_empty_test() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new_empty();
        list.push(42);
        list.push(43);
        list.push(44);

        // Add assertions to test the behavior
        assert_eq!(list.is_empty(), false);

        let list2: SinglyLinkedList<i32>   = SinglyLinkedList::new_empty();
        assert_eq!(list2.is_empty(), true);
    }

    #[test]
    fn clear_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.push(42);
        list.push(43);

        // Add assertions to test the behavior
        assert_eq!(list.is_empty(), false);
        list.clear();
        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn iterator_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.push(42);
        list.push(43);
        list.push(44);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(44));
        assert_eq!(iter.next(), Some(43));
        assert_eq!(iter.next(), Some(42));
        assert_eq!(iter.next(), None);

        let mut list2 = SinglyLinkedList::new_empty();
        list2.push(42);
        list2.push(43);
        list2.push(44);

        for (i, val) in list2.into_iter().enumerate() {
            assert_eq!(val, 44 - i as i32);
        }
    }

    #[test]
    fn formatting_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.push(42);
        list.push(43);
        list.push(44);
        assert_eq!(format!("{:?}", list), "SinglyLinkedList { total_size_bytes: 12, node_count: 3 }");
    }

    #[test]
    fn big_list_test() {
        let mut list = SinglyLinkedList::new_empty();
        for i in 0..1000 {
            list.push(i);
        }
        assert_eq!(list.node_count, 1000);
        assert_eq!(list.total_size_bytes, 1000 * std::mem::size_of::<i32>());

        for i in 0..1000 {
            assert_eq!(list.pop(), Some(999 - i));
        }

        assert_eq!(list.node_count, 0);

        for i in 0..1000 {
            list.append(i);
        }
        assert_eq!(list.node_count, 1000);

        for i in 0..1000 {
            assert_eq!(list.pop(), Some(i));
        }

        assert_eq!(list.node_count, 0);

        list.push(42);
        list.push(43);
        list.push(44);
        assert!(list.is_empty() == false);
        assert!(list.peek() == Some(44));
        assert!(list.peek_tail() == Some(42));
        assert!(list.pop() == Some(44));
        assert!(list.peek() == Some(43));
        assert!(list.peek_tail() == Some(42));
        assert!(list.pop() == Some(43));
        assert!(list.peek() == Some(42));
        assert!(list.peek_tail() == Some(42));
        assert!(list.pop() == Some(42));
        assert!(list.peek() == None);
        assert!(list.peek_tail() == None);
        assert!(list.is_empty() == true);
        assert!(list.pop() == None);

        list.push(42);
        list.push(43);
        list.push(44);
        assert!(list.is_empty() == false);
        list.clear();
        assert!(list.is_empty() == true);

    }

    // TEST AREA FOR SINGLY LINKED LIST ENDS //

}
