
pub fn quicksort<T: Ord>(arr: &mut [T]) {
    _quicksort(arr, 0, (arr.len() - 1) as isize);
}

fn _quicksort<T: Ord>(arr: &mut [T], left: isize, right: isize) {
    if left <= right {
        let partition_idx = partition(arr, 0, right);

        _quicksort(arr, left, partition_idx - 1);
        _quicksort(arr, partition_idx + 1, right);
    }
}

fn partition<T: Ord>(arr: &mut [T], left: isize, right: isize) -> isize {
    let pivot = right;
    let mut i: isize = left as isize - 1;

    for j in left..=right - 1 {
        if arr[j as usize] <= arr[pivot as usize] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, pivot as usize);

    i + 1
}

// **********  TESTS ********** //


#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    // write a test with 100000 elements
    #[test]
    fn test_quick_sort_10000() {
        // make a vector of 10000 elements with collect()
        let mut arr: Vec<_> = (0..1000).collect();
        // shuffle the vector
        arr.shuffle(&mut thread_rng());
        // sort the vector
        quicksort(&mut arr);
        // assert that the vector is sorted
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}