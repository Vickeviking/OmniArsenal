use std::ops::Index;
pub fn insertionsort<T, O>(container: &mut T)    
where
    T: Index<usize, Output = O> + AsMut<[O]> + AsRef<[O]> + PartialOrd + Sized,
    O: PartialOrd,
{
    let len = container.as_ref().len();

    for i in 1..len {
        let mut j = i;
        while j > 0 && container.as_ref()[j - 1] > container.as_ref()[j] {
            container.as_mut().swap(j - 1, j);
            j -= 1;
        }
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
    fn test_insertion_sort() {
        let mut arr = [5, 4, 3, 2, 1];
        insertionsort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    // test 300 elements
    #[test]
    fn test_insertion_sort_300() {
        // make a vector of 300 elements with collect()
        let mut arr: Vec<_> = (0..300).collect();
        // shuffle the vector
        arr.shuffle(&mut thread_rng());
        // sort the vector
        insertionsort(&mut arr);
        // assert that the vector is sorted
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    // test 10000 elements
    #[cfg(feature = "heavy_test")]
    #[test]
    fn test_insertion_sort_10000() {
        // make a vector of 10000 elements with collect()
        let mut arr: Vec<_> = (0..10000).collect();
        // shuffle the vector
        arr.shuffle(&mut thread_rng());
        // sort the vector
        insertionsort(&mut arr);
        // assert that the vector is sorted
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

}