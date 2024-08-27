#[cfg(test)]
mod isomorphic_array_tests {
    use dsa::*;

    #[test]
    fn test_failure_different() {
        assert!(!isomorphic_array("ababr", "pqrqo"))
    }
    #[test]
    fn test_failure_repeated() {
        assert!(!isomorphic_array("ababr", "eoeoo"))
    }
    #[test]
    fn test_success() {
        assert!(isomorphic_array("green", "abccd"))
    }
}
