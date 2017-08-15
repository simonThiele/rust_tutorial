// functiontests start with ///

/// # Examples
///
/// ```
// need to import because the test will be wrapped in a fn main() {...}
/// use adder::add;
/// assert_eq!(add(1, 4), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// test cfg is not compiled within our lib
#[cfg(test)]
mod adder {
    use super::add;

    // add helper methods here if needed

    #[test]
    fn should_add() {
        assert_eq!(add(1, 4), 5);
    }

    #[test]
    #[should_panic]
    fn throw_panic() {
        // throws a panic
        assert!(false);
    }
}
