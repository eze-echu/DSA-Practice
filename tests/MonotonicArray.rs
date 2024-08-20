#[cfg(test)]
mod monotonic_array_tests {
    use dsa::*;
    #[test]
    fn test_basic_increasing(){
        assert!(monotonic_array(&[1, 2, 3, 4]));
    }
    #[test]
    fn test_basic_decreasing(){
        assert!(monotonic_array(&[4, 3, 2, 1]));
    }
    #[test]
    fn test_negative_increasing(){
        assert!(monotonic_array(&[-4, -3, -2, -1]));
    }
    #[test]
    fn test_negative_decreasing(){
        assert!(monotonic_array(&[-1, -2, -3, -4]));
    }
    #[test]
    fn test_mixed(){
        assert!(monotonic_array(&[-4, -3, 0, 1, 2, 3]));
    }
    #[test]
    fn test_zero(){
        assert!(monotonic_array(&[0]));
    }
    #[test]
    fn test_empty(){
        assert!(monotonic_array(&[]));
    }
}