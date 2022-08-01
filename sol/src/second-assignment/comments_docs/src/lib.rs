#![feature(never_type)]
pub mod compute;
extern crate assert_panic;

/// Add three to the given value and return a [`Option`] type
pub fn add_three(x: i32) -> Option<i32> {
    Some(x + 3)
}


// Also, run these tests using nightly version.
// Cause, I used "never_return" feature.

#[cfg(test)]
mod tests {
    use crate::compute::{add_one, add_two, div, try_div};

    #[test]
    fn add_one_test() {
        assert_eq!(add_one(5), 6)
    }

    #[test]
    fn add_two_test() {
        assert_eq!(add_two(5), 7)
    }

    #[test]
    fn panic_test() {
        // This is for asserting panic.
        // It's an external document that I repo-ed.
        // Is this unnecessary? yes. Do I care? No.

      assert_panic::assert_panic!({div(10, 0);}); 
    }
    
    #[test]
    fn try_div_test() {
        assert_eq!(try_div(10, 0), Err(String::from("Divide-by-zero")))
    }
}


mod a {
    ///  Add four to the given value and return a [`Option`] type
    /// [`crate::MySpecialFormatter`]
    pub fn add_four(x: i32) -> Option<i32> {
        Some(x + 4)
    }
}

struct MySpecialFormatter;

