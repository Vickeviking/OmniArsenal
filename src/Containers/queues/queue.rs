
/***
 * Queue
 * 
 * A queue is a FIFO (first in, first out) data structure.
 * functions:
 * - enqueue: add an element to the end of the queue O(1)
 * - dequeue: remove an element from the front of the queue O(1)
 * - peek: get the element at the front of the queue O(1)
 * - is_empty: check if the queue is empty O(1)
 */
use std::collections::VecDeque;
use std::iter::FromIterator;


pub struct Queue<T> {
    queue: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new_empty() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn new(data: T) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(data);
        Self { queue }
    }

    pub fn enqueue(&mut self, data: T) {
        self.queue.push_back(data);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }
}

impl<T> FromIterator<T> for Queue<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let queue = VecDeque::from_iter(iter);
        Self { queue }
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.queue.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;
    use std::iter::FromIterator;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new_empty();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_from_iterator() {
        let queue = Queue::from_iter(vec![1, 2, 3]);
        assert_eq!(queue.queue, vec![1, 2, 3]);
    }

    #[test]
    fn test_into_iterator() {
        let queue = Queue::from_iter(vec![1, 2, 3]);
        let vec: Vec<_> = queue.into_iter().collect();
        assert_eq!(vec, vec![1, 2, 3]);
    }
}