#[cfg(test)]
mod square_array_tests {
    use dsa::*;
    #[test]
    fn test_square_array_basic() {
        let base= [1, 2, 3, 4];
        assert_eq!(vec![1, 4, 9, 16], square_array(&base));
    }
    #[test]
    fn test_square_array_negative() {
        let base= [-4, -3, -2, -1];
        assert_eq!(vec![1, 4, 9, 16], square_array(&base));
    }
    #[test]
    fn test_square_array_mixed() {
        let base= [-4, -3, 0, 1, 2, 3];
        assert_eq!(vec![0, 1, 4, 9, 9, 16], square_array(&base));
    }
    #[test]
    fn test_square_array_zero() {
        let base= [0];
        assert_eq!(vec![0], square_array(&base));
    }
    #[test]
    fn test_square_array_empty() {
        let reference: [i32;0] = [];
        assert_eq!(reference.to_vec(), square_array(&[]));
    }
}