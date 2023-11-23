/*
* ArrayList
* Wrapper that uses arrays under the hood
* append/pop/get has O(1)
* prepend has O(n)
* constructor specifis initial size
*/

//todo add iterator traits, insert at & set

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
        self.length = self.length + 1;
        if self.inner.len() <= self.length {
            self.grow_inner();
        } 
        self.inner[self.tail] = item;
        self.tail = self.tail + 1;
    }

    pub fn prepend(&mut self, item: T) {
        //println!("prepending {item:?} to {self:?}");
        // If the array is empty, append the item instead, avoids shifting 
        if self.length == 0 {
            self.append(item);
            return;
        }

        self.length = self.length + 1;

        if self.inner.len() <= self.length {
            self.grow_inner();
        } 
        //shift all items to the right 0
        self.shift_all_right(0);
        // front now empty, insert item
        self.inner[0] = item;
        // move tail back one
        self.tail = self.tail + 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.tail > 0 {
            let item = mem::replace(&mut self.inner[self.tail - 1], T::default());
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
            let item = mem::replace(&mut self.inner[index], T::default());
            self.shift_all_left(index, 1);
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

    fn shift_all_left(&mut self, start_index: usize, offset: usize) {
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
            self.inner[i] = mem::replace(&mut self.inner[i + 1], T::default());
        }

        self.tail = self.tail - 1;
        self.length = self.length - 1;
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
            self.inner[i + 1] = mem::replace(&mut self.inner[i], T::default());
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


impl<T: fmt::Debug> fmt::Debug for ArrayList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArrayList")
            .field("length", &self.length)
            .field("tail", &self.tail)
            .field("inner", &&self.inner[..])
            .finish()
    }
}