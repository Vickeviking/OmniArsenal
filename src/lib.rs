
// In omni_arsenal/src/lib.rs

pub mod containers;



#[cfg(test)]
mod tests {
    use crate::containers::lists_test;
    #[test]
    pub fn test_all_lists() {
        lists_test::list_tests::test_array_list();
        lists_test::list_tests::test_singly_linked_list();
    }

}

#[cfg(test)]
pub fn run_tests() {
    tests::test_all_lists();
}

// This ensures the function is available even without the test feature
#[cfg(not(test))]
pub fn run_tests() {
    println!("You successfully ran the OmniArsenalLib");
}

