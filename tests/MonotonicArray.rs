#[cfg(test)]
mod tests{
    use dsa::*;
    #[test]
    fn test_basic_increasing(){
        assert_eq!(true, monotonic_array(vec![1, 2, 3, 4]));
    }
    #[test]
    fn test_basic_decreasing(){
        assert_eq!(true, monotonic_array(vec![4, 3, 2, 1]));
    }
    #[test]
    fn test_negative_increasing(){
        assert_eq!(true, monotonic_array(vec![-4, -3, -2, -1]));
    }
    #[test]
    fn test_negative_decreasing(){
        assert_eq!(true, monotonic_array(vec![-1, -2, -3, -4]));
    }
    #[test]
    fn test_mixed(){
        assert_eq!(true, monotonic_array(vec![-4, -3, 0, 1, 2, 3]));
    }
    #[test]
    fn test_zero(){
        assert_eq!(true, monotonic_array(vec![0]));
    }
    #[test]
    fn test_empty(){
        assert_eq!(true, monotonic_array(vec![]));
    }
}