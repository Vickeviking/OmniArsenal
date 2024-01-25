/***
 * ArrayList
 * Wrapper that uses arrays under the hood
 * append/pop/get has O(1)
 * prepend has O(n)
 * constructor specifies initial size
 * 
 * Upsides:
 * - Speed: Arrays/slices make things really fast
 * - Simple and fast element access
 * - Clear ownership structures
 * - Fast append and iteration
 * - Very CPU cache friendly
 * 
 * Downsides:
 * - Operations other than append require shifting elements
 * - Growth is expensive (requires copying to new array)
 * - A single large chunk of memory is required
 * - Size is limited by usize type, which differs from platform to platform
 * - Growth speed decreases with size
 */

//todo add iterator traits

use std::fmt::Debug;
use std::mem;
use core::alloc::Layout;
use std::alloc::alloc;
use std::ptr;
use std::fmt;
//fake array
pub struct ArrayList<T> {
    pub length: usize,
    inner: Box<[T]>,
    tail: usize,
}

pub struct ArrayListIterator<'a, T> {
    current: usize,
    list: &'a ArrayList<T>,
}

impl<T: Default + Debug> ArrayList<T> {
    pub fn new() -> ArrayList<T> {
        let layout = Layout::array::<T>(5).unwrap();
        let ptr = unsafe { alloc(layout) } as *mut T;
        for i in 0..5 {
            unsafe { ptr::write(ptr.add(i), T::default()) };
        }
        let slice = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, 5)) };
        ArrayList {
            length: 0,
            inner: slice,
            tail: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> ArrayList<T> {
        let layout = Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { alloc(layout) } as *mut T;
        for i in 0..capacity {
            unsafe { ptr::write(ptr.add(i), T::default()) };
        }
        let slice = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, capacity)) };
        ArrayList {
            length: 0,
            inner: slice,
            tail: 0,
        }
    }

    fn grow_inner(&mut self) {
        let len = self.inner.len();
        let layout = Layout::array::<T>(len * 2).unwrap();
        let ptr = unsafe { alloc(layout) } as *mut T;
        let mut new_inner = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, len * 2)) };
        unsafe { 
            ptr::copy(
                self.inner.as_ptr(), 
                new_inner.as_mut_ptr(), 
                len
            );
        }
        self.inner = new_inner;
    }

    pub fn append(&mut self, item: T) {
        //println!("appending {item:?} to {self:?}");
        self.length += 1;
        if self.inner.len() <= self.length {
            self.grow_inner();
        } 
        self.inner[self.tail] = item;
        self.tail += 1;
    }

    pub fn prepend(&mut self, item: T) {
        //println!("prepending {item:?} to {self:?}");
        // If the array is empty, append the item instead, avoids shifting 
        if self.length == 0 {
            self.append(item);
            return;
        }

        self.length += 1;

        if self.inner.len() <= self.length {
            self.grow_inner();
        } 
        //shift all items to the right 0
        self.shift_all_right(0);
        // front now empty, insert item
        self.inner[0] = item;
        // move tail back one
        self.tail += 1;
    }

    pub fn set(&mut self, index: usize, el: T) -> Option<T> {
        // replace mem at index with el and return replaced mem
        if index >= self.length {
            return None; //can't set an out of bounds index
        }
        let item: T = mem::replace(&mut self.inner[index], el);
        Some(item)
    }

    pub fn insert_at( &mut self, index: usize, el: T)  {
        // insert el at index and return replaced mem
        if index >= self.length {
            return ; //can't insert at out of bounds index
        };
        // shift all items to the right
        self.shift_slice_right(index);
        // insert el at just freed index
        self.inner[index] = el;
        // move tail back one and increment length
        self.tail += 1;
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.tail > 0 {
            let item = std::mem::take(&mut self.inner[self.tail - 1]);
            //println!("removing item {:?}", item);
            self.tail -= 1;
            self.length -= 1;
            Some(item)
        } else {
            None
        }
    }

    pub fn pop_at(&mut self, index: usize) -> Option<T> {
        if index < self.length {
            let item = std::mem::take(&mut self.inner[index]);
            self.shift_all_left(index);
            self.tail -= 1;
            self.length -= 1;
            Some(item)
        } else {
            None //can't pop an out of bounds index
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match index < self.length {
            true => Some(&self.inner[index]),
            false => None,
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match index < self.length {
            true => Some(&mut self.inner[index]),
            false => None,
        }
    }

    fn shift_all_left(&mut self, start_index: usize) {
        // Ensure there is enough space in the inner array
        while self.inner.len() <= self.tail + 1 {
            self.grow_inner();
        }
    
        // If the array is empty, there's nothing to shift
        if self.length == 0 {
            return;
        }
    
        // Shift elements to the left
        for i in start_index..self.length {
            self.inner[i] = std::mem::take(&mut self.inner[i + 1]);
        }
    }

    fn shift_slice_right(&mut self, start_index: usize) {
        if self.length == self.inner.len() {
            self.grow_inner();
        }
        if self.length == 0 {
            return;
        }
        for i in (start_index..=self.length).rev() {
            self.inner[i + 1] = std::mem::take(&mut self.inner[i]);
        }
    }

    fn shift_all_right(&mut self, offset: usize) {
        // Ensure there is enough space in the inner array
        while self.inner.len() <= self.tail + offset {
            self.grow_inner();
        }
    
        // If the array is empty, there's nothing to shift
        if self.length == 0 {
            return;
        }
    
        // Shift elements to the right
        for i in (offset..self.length).rev() {
            self.inner[i + 1] = std::mem::take(&mut self.inner[i]);
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn clear(&mut self) {
        for i in 0..self.length {
            self.inner[i] = T::default();
        }
        self.length = 0;
        self.tail = 0;
    }



}

impl<'a, T: Default + Debug> Iterator for ArrayListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.list.length {
            let item = &self.list.inner[self.current];
            self.current += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a, T: Default + Debug> IntoIterator for &'a ArrayList<T> {
    type Item = &'a T;
    type IntoIter = ArrayListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayListIterator {
            current: 0,
            list: self,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for ArrayList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArrayList")
            .field("length", &self.length)
            .field("tail", &self.tail)
            .field("inner", &&self.inner[..])
            .finish()
    }
}
impl<T: Default + Debug> Default for ArrayList<T> {
    fn default() -> Self {
        Self::new()
    }
}



// **********  TESTS ********** //

#[cfg(test)]
mod tests {
    use super::*;

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
}