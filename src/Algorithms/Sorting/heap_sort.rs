
use std::ops::Index;
pub fn heapsort<T, O>(container: &mut T)
where
    T: Index<usize, Output = O> + AsMut<[O]> + AsRef<[O]> + PartialOrd + Sized,
    O: PartialOrd,
{
    let len = container.as_ref().len();

    for i in (0..len / 2).rev() {
        heapify(container, len, i);
    }

    for i in (0..len).rev() {
        container.as_mut().swap(0, i);
        heapify(container, i, 0);
    }
}

fn heapify<T, O>(container: &mut T, n: usize, i: usize)
where
    T: Index<usize, Output = O> + AsMut<[O]> + AsRef<[O]> + PartialOrd + Sized,
    O: PartialOrd,
{
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && container.as_ref()[left] > container.as_ref()[largest] {
        largest = left;
    }

    if right < n && container.as_ref()[right] > container.as_ref()[largest] {
        largest = right;
    }

    if largest != i {
        container.as_mut().swap(i, largest);
        heapify(container, n, largest);
    }
}



// **********  TESTS ********** //


#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    // write a test
    #[test]
    fn test_heap_sort() {
        let mut arr = [5, 4, 3, 2, 1];
        heapsort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    // test 300 elements
    #[test]
    fn test_heap_sort_300() {
        // make a vector of 300 elements with collect()
        let mut arr: Vec<_> = (0..300).collect();
        // shuffle the vector
        arr.shuffle(&mut thread_rng());
        // sort the vector
        heapsort(&mut arr);
        // assert that the vector is sorted
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    // test 10000 elements
    #[cfg(feature = "heavy_test")]
    #[test]
    fn test_heap_sort_10000() {
        // make a vector of 10000 elements with collect()
        let mut arr: Vec<_> = (0..10000).collect();
        // shuffle the vector
        arr.shuffle(&mut thread_rng());
        // sort the vector
        heapsort(&mut arr);
        // assert that the vector is sorted
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

}