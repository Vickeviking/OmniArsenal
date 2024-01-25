
pub struct PriorityQueue<T> {
    heap: Vec<T>,
    compare: fn(&T, &T) -> bool,
}