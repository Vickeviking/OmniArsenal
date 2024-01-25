use std::ops::Index;
pub fn merge_sort<T, O>(container: &mut T)
where
    T: Index<usize, Output = O> + AsMut<[O]> + AsRef<[O]> + PartialOrd + Sized,
    O: PartialOrd,
{
    let len = container.as_ref().len();

    for i in 0..len {
        for j in 0..len - i - 1 {
            if container.as_ref()[j] > container.as_ref()[j + 1] {
                // Swap elements if they are in the wrong order
                container.as_mut().swap(j, j + 1);
            }
        }
    }
}



// **********  TESTS ********** //


#[cfg(test)]
mod tests {
    use super::*;

}