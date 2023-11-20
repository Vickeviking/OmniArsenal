/*
* ArrayList
* Wrapper that uses arrays under the hood
* push/pop/access has O(1)
* enqueue/dequeue has O(n)
* constructor specifis initial size
*/

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