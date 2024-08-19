#[cfg(test)]
mod square_array_tests {
    use dsa::*;
    #[test]
    fn test_square_array_basic() {
        let base: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(vec![1, 4, 9, 16], square_array(base));
    }
    #[test]
    fn test_square_array_negative() {
        let base: Vec<i32> = vec![-4, -3, -2, -1];
        assert_eq!(vec![1, 4, 9, 16], square_array(base));
    }
    #[test]
    fn test_square_array_mixed() {
        let base: Vec<i32> = vec![-4, -3, 0, 1, 2, 3];
        assert_eq!(vec![0, 1, 4, 9, 9, 16], square_array(base));
    }
    #[test]
    fn test_square_array_zero() {
        let base: Vec<i32> = vec![0];
        assert_eq!(vec![0], square_array(base));
    }
    #[test]
    fn test_square_array_empty() {
        let reference: Vec<i32> = vec![];
        assert_eq!(reference, square_array(vec![]));
    }
}