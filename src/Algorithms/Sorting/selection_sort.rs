use std::ops::Index;

pub fn selectionsort<T, O>(container: &mut T)
where
    T: Index<usize, Output = O> + AsMut<[O]> + AsRef<[O]> + PartialOrd + Sized,
    O: PartialOrd,
{
    let len = container.as_ref().len();

    for i in 0..len {
        let mut min = i;
        for j in i + 1..len {
            if container.as_ref()[j] < container.as_ref()[min] {
                min = j;
            }
        }
        if min != i {
            container.as_mut().swap(i, min);
        }
    }
}



// **********  TESTS ********** //


#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn test_selection_sort() {
        let mut arr = [5, 4, 3, 2, 1];
        selectionsort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    // test 10000 elements
    #[test]
    fn test_selection_sort_10000() {
        // make a vector of 10000 elements with collect()
        let mut arr: Vec<_> = (0..10000).collect();
        // shuffle the vector
        arr.shuffle(&mut thread_rng());
        // sort the vector
        selectionsort(&mut arr);
        // assert that the vector is sorted
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }


}