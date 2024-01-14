// In src/Container/lists_test.rs
// containers/lists_test.rs

#[cfg(test)]
pub mod list_tests {

    use super::super::super::containers::lists::array_list::ArrayList;
    use super::super::super::containers::lists::singly_linked_list::SinglyLinkedList;
    use super::super::super::containers::lists::doubly_linked_list::DoublyLinkedList;
    
    // SinglyLinkedList tests

    #[test]
    pub fn test_array_list() {
        array_list_append_test();
        array_list_prepend_test();
        array_list_growth_test();
        array_list_with_capacity_test();
        array_list_pop_test();
        array_list_get_test();
        array_list_get_mut_test();
        test_len_is_empty_and_clear();
        array_list_pop_at_test();
        array_list_set_test();
        test_insert_at();
        test_array_list_iteration();
    }

    #[test]
    pub fn test_singly_linked_list() {
        prepend_test();
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

    #[test]
    pub fn test_doubly_linked_list() {
        doubly_list_prepend_test();
        doubly_list_append_test();
        doubly_list_pop_front_test();
        doubly_list_pop_back_test();
        doubly_list_clear_test();
        doubly_list_peek_front_test();
        doubly_list_iterator_test();
    }

    // TEST AREA FOR ARRAY LIST STARTS //

    #[test]
    fn array_list_set_test() {
        let mut arr = ArrayList::<i32>::new();
        arr.append(42);
        arr.append(43);
        arr.append(44);

        // Add assertions to test the behavior
        assert_eq!(arr.set(0, 43), Some(42));
        assert_eq!(arr.get(0), Some(&43));
        assert_eq!(arr.set(2, 30), Some(44));
        assert_eq!(arr.get(2), Some(&30));
        assert_eq!(arr.set(3, 30), None);

    }

    #[test]
    fn array_list_append_test() {
        let mut arr = ArrayList::<i32>::new();
        assert_eq!(arr.length, 0);
        arr.append(42);
        assert_eq!(arr.length, 1);
    }

    #[test]
    fn array_list_prepend_test() {
        let mut arr = ArrayList::<i32>::new();
        assert_eq!(arr.length, 0);

        // Prepend an element
        arr.prepend(42);
        assert_eq!(arr.length, 1);
        assert_eq!(arr.get(0), Some(&42));
        // Prepend another element
        arr.prepend(41);
        assert_eq!(arr.length, 2);
        assert_eq!(arr.get(0), Some(&41));
        assert_eq!(arr.get(1), Some(&42));
        // Prepend another element
        arr.prepend(40);
        assert_eq!(arr.length, 3);
        assert_eq!(arr.get(0), Some(&40));
        assert_eq!(arr.get(1), Some(&41));
        assert_eq!(arr.get(2), Some(&42));
        // Prepend another element
        arr.prepend(39);
        assert_eq!(arr.length, 4);
        assert_eq!(arr.get(0), Some(&39));
        assert_eq!(arr.get(1), Some(&40));
        assert_eq!(arr.get(2), Some(&41));
        assert_eq!(arr.get(3), Some(&42));

    }

    #[test]
    fn test_insert_at() {
        let mut arr = ArrayList::<i32>::new();

        // Append 10 items
        for i in 1..11 {
            arr.append(i);
        }

        // Insert at index 0
        arr.insert_at(0, 0);
        assert_eq!(arr.len(), 11);
        assert_eq!(arr.get(0), Some(&0)); // First item should now be 0
        assert_eq!(arr.get(1), Some(&1)); // Second item should now be 1

        // Insert at index 5
        arr.insert_at(5, 50);
        assert_eq!(arr.len(), 12);
        assert_eq!(arr.get(5), Some(&50)); // The item at index 5 should be 50
        assert_eq!(arr.get(6), Some(&5)); // The item at index 6 should be 6 (the original item at index 5)

        // Insert at last index

        print!("arr: {:?}", arr);
        arr.insert_at(11, 100);
        print!("arr: {:?}", arr);
        assert_eq!(arr.len(), 13);
        assert_eq!(arr.get(11), Some(&100)); // The item at index 11 should be 100
        assert_eq!(arr.get(12), Some(&10)); // The item at index 12 should be 10

        // Try to insert at an out-of-bounds index
        arr.insert_at(100, 200);
        assert_eq!(arr.len(), 13); // Length should not have changed
    }

    #[test]
    fn array_list_with_capacity_test() {
        let mut arr = ArrayList::<i32>::with_capacity(10);
        assert_eq!(arr.length, 0);
        arr.append(42);
        assert_eq!(arr.length, 1);
    }

    #[test]
    fn array_list_growth_test() {
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

    #[test]
    fn array_list_pop_test() {
        let mut arr = ArrayList::<i32>::new();
        arr.append(42);
        arr.append(43);
        arr.append(44);

        // Add assertions to test the behavior
        assert_eq!(arr.pop(), Some(44));
        assert!(arr.length == 2);
        assert_eq!(arr.pop(), Some(43));
        assert_eq!(arr.pop(), Some(42));
        assert!(arr.length == 0);
        assert_eq!(arr.pop(), None);

    }

    #[test]
    fn array_list_get_test() {
        let mut arr = ArrayList::<i32>::new();
        arr.append(42);
        arr.append(43);
        arr.append(44);

        // Add assertions to test the behavior
        assert_eq!(arr.get(0), Some(&42));
        assert_eq!(arr.get(1), Some(&43));
        assert_eq!(arr.get(2), Some(&44));
        assert_eq!(arr.get(3), None);
    }

    #[test]
    fn array_list_get_mut_test() {
        let mut arr = ArrayList::<i32>::new();
        arr.append(42);
        arr.append(43);
        arr.append(44);

        // Add assertions to test the behavior
        assert_eq!(arr.get_mut(0), Some(&mut 42));
        assert_eq!(arr.get_mut(1), Some(&mut 43));
        assert_eq!(arr.get_mut(2), Some(&mut 44));
        assert_eq!(arr.get_mut(3), None);

        // try and change the values
        *arr.get_mut(0).unwrap() = 1;
        *arr.get_mut(1).unwrap() = 2;
        *arr.get_mut(2).unwrap() = 3;

    }

    #[test]
    fn test_len_is_empty_and_clear() {
        let mut arr = ArrayList::<i32>::new();

        // At this point, the array list should be empty
        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());

        // Add some elements to the array list
        arr.append(42);
        arr.append(43);
        arr.append(44);

        // Now the array list should have 3 elements
        assert_eq!(arr.len(), 3);
        assert!(!arr.is_empty());

        // Clear the array list
        arr.clear();

        // The array list should be empty again
        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());
    }

    #[test]
    fn array_list_pop_at_test() {
        let mut arr = ArrayList::<i32>::new();

        // Append 100 items
        for i in 1..101 {
            arr.append(i);
            assert_eq!(arr.len(), i as usize); // Assert length after each append
        }

        // Check the first item
        assert_eq!(arr.get(0), Some(&1));

        // Check the last item
        assert_eq!(arr.get(99), Some(&100));

        // Pop at index 0
        assert_eq!(arr.pop_at(0), Some(1));
        assert_eq!(arr.len(), 99);
        assert_eq!(arr.get(0), Some(&2)); // First item should now be 2

        // Pop at index 50
        assert_eq!(arr.pop_at(50), Some(52)); // The item at index 50 should be 52 (since we already removed the first item)
        assert_eq!(arr.len(), 98);

        // Pop at last index
        assert_eq!(arr.pop_at(97), Some(100)); // The last item should be 100
        assert_eq!(arr.len(), 97);
    }


    #[test]
    fn test_array_list_iteration() {
        let mut arr = ArrayList::<i32>::new();

        // Append 10 items
        for i in 1..11 {
            arr.append(i);
        }

        // Iterate over the array list and check the items
        let mut index = 1;
        for item in &arr {
            assert_eq!(*item, index);
            index += 1;
        }

        // Check that the iteration covered all items
        assert_eq!(index, 11);

        // Insert an item at index 5
        arr.insert_at(5, 50);

        // Iterate over the array list again and check the items
        let expected_items = vec![1, 2, 3, 4, 5, 50, 6, 7, 8, 9, 10];
        let mut index = 0;
        for item in &arr {
            assert_eq!(*item, expected_items[index]);
            index += 1;
        }

        // Check that the iteration covered all items
        assert_eq!(index, expected_items.len());
    }
    // TEST AREA FOR ARRAY LIST ENDS //



    // TEST AREA FOR DOUBLY LINKED LIST STARTS //

    
    #[test]
    fn doubly_list_prepend_test() {
        let mut list = DoublyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);
    
        assert_eq!(*list.peek_front().unwrap(), 44);
        list.prepend(45);
        assert_eq!(*list.peek_front().unwrap(), 45);
    }
    
    #[test]
    fn doubly_list_append_test() {
        let mut list = DoublyLinkedList::new_empty();
        list.append(42);
        list.append(43);
        list.append(44);
    
        assert_eq!(*list.peek_front().unwrap(), 42);
    }
    
    #[test]
    fn doubly_list_pop_front_test() {
        let mut list = DoublyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);
    
        assert_eq!(list.pop_front(), Some(44));
        assert_eq!(list.pop_front(), Some(43));
        assert_eq!(list.pop_front(), Some(42));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn doubly_list_pop_back_test() {
        let mut list = DoublyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);

        assert_eq!(list.pop_back(), Some(42));
        assert_eq!(list.pop_back(), Some(43));
        assert_eq!(list.pop_back(), Some(44));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn doubly_list_clear_test() {
        let mut list = DoublyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);

        assert_eq!(list.len, 2);
        list.clear();
        assert_eq!(list.len, 0);
    }
    
    #[test]
    fn doubly_list_peek_front_test() {
        let mut list = DoublyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);

        assert_eq!(*list.peek_front().unwrap(), 44);
    }

    #[test]
    fn doubly_list_iterator_test() {
        let mut list = DoublyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);

        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap(), 44);
        assert_eq!(iter.next().unwrap(), 43);
        assert_eq!(iter.next().unwrap(), 42);
        assert!(iter.next().is_none());
    }

    // TEST AREA FOR DOUBLY LINKED LIST ENDS //




    // TESTS AREA FOR SINGLY LINKED LIST STARTS //
    #[test]
    fn prepend_test() {
        // Your push test implementation goes here
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);

        // Add assertions to test the behavior
        assert_eq!(list.len, 1);
        assert_eq!(list.total_size_bytes, std::mem::size_of::<i32>());
    }

    #[test]
    fn append_test() {
        // Your append test implementation goes here
        let mut list = SinglyLinkedList::new_empty();
        list.append(42);

        // Add assertions to test the behavior
        assert_eq!(list.len, 1);
        assert_eq!(list.total_size_bytes, std::mem::size_of::<i32>());
    }

    #[test]

    fn pop_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);

        // Add assertions to test the behavior
        assert_eq!(list.pop(), Some(44));
        assert_eq!(list.pop(), Some(43));
        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn peek_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);

        // Add assertions to test the behavior
        assert_eq!(list.peek(), Some(42));
    }

    #[test]
    fn peek_tail_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);

        // Add assertions to test the behavior
        assert_eq!(list.peek_tail(), Some(42));
    }

    #[test]

    fn is_empty_test() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);

        // Add assertions to test the behavior
        assert_eq!(list.is_empty(), false);

        let list2: SinglyLinkedList<i32>   = SinglyLinkedList::new_empty();
        assert_eq!(list2.is_empty(), true);
    }

    #[test]
    fn clear_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);

        // Add assertions to test the behavior
        assert_eq!(list.is_empty(), false);
        list.clear();
        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn iterator_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(44));
        assert_eq!(iter.next(), Some(43));
        assert_eq!(iter.next(), Some(42));
        assert_eq!(iter.next(), None);

        let mut list2 = SinglyLinkedList::new_empty();
        list2.prepend(42);
        list2.prepend(43);
        list2.prepend(44);

        for (i, val) in list2.into_iter().enumerate() {
            assert_eq!(val, 44 - i as i32);
        }
    }

    #[test]
    fn formatting_test() {
        let mut list = SinglyLinkedList::new_empty();
        list.prepend(42);
        list.prepend(43);
        list.prepend(44);
        assert_eq!(format!("{:?}", list), "SinglyLinkedList { total_size_bytes: 12, node_count: 3 }");
    }

    #[test]
    fn big_list_test() {
        let mut list = SinglyLinkedList::new_empty();
        for i in 0..1000 {
            list.prepend(i);
        }
        assert_eq!(list.len, 1000);
        assert_eq!(list.total_size_bytes, 1000 * std::mem::size_of::<i32>());

        for i in 0..1000 {
            assert_eq!(list.pop(), Some(999 - i));
        }

        assert_eq!(list.len, 0);

        for i in 0..1000 {
            list.append(i);
        }
        assert_eq!(list.len, 1000);

        for i in 0..1000 {
            assert_eq!(list.pop(), Some(i));
        }

        assert_eq!(list.len, 0);

        list.prepend(42);
        list.prepend(43);
        list.prepend(44);
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

        list.prepend(42);
        list.prepend(43);
        list.prepend(44);
        assert!(list.is_empty() == false);
        list.clear();
        assert!(list.is_empty() == true);

    }

    // TEST AREA FOR SINGLY LINKED LIST ENDS //

}
