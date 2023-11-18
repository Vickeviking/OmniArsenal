
// In omni_arsenal/src/lib.rs

pub mod containers;


#[cfg(test)]
mod tests {
    #[test]
    pub fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg(test)]
pub fn run_tests() {
    tests::it_works();
}

// This ensures the function is available even without the test feature
#[cfg(not(test))]
pub fn run_tests() {
    println!("You successfully ran the OmniArsenalLib");
}

