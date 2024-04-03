use std::ops::Index;
pub fn mergesort<T, O>(container: &mut T)
where
    T: Index<usize, Output = O> + AsMut<[O]> + AsRef<[O]> + PartialOrd + Sized + Clone,
    O: PartialOrd + Clone,
{
    let len = container.as_ref().len();

    if len > 1 {
        let mid = len / 2;
        let mut left = container.as_ref()[0..mid].to_vec();
        let mut right = container.as_ref()[mid..len].to_vec();

        mergesort(&mut left);
        mergesort(&mut right);

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                container.as_mut()[k] = left[i].clone();
                i += 1;
            } else {
                container.as_mut()[k] = right[j].clone();
                j += 1;
            }
            k += 1;
        }

        while i < left.len() {
            container.as_mut()[k] = left[i].clone();
            i += 1;
            k += 1;
        }

        while j < right.len() {
            container.as_mut()[k] = right[j].clone();
            j += 1;
            k += 1;
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
    fn test_merge_sort() {
        let mut arr = [5, 4, 3, 2, 1];
        mergesort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    // test 10000 elements
    #[test]
    fn test_quick_sort_10000() {
        // make a vector of 10000 elements with collect()
        let mut arr: Vec<_> = (0..10000).collect();
        // shuffle the vector
        arr.shuffle(&mut thread_rng());
        // sort the vector
        mergesort(&mut arr);
        // assert that the vector is sorted
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

}