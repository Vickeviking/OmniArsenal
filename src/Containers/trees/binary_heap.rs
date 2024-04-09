/*
    An Arena Allocated Binary Heap,
    Used as both a max heap & min heap depending on provided flag

*/


/*
    Structural notes for self reference:

    Parent = i/2  // rounded down
    Left = 2i
    Right = 21 + 1

*/

pub struct BinaryHeap<T> {
    heap: Vec<T>,
}