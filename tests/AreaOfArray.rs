#[cfg(test)]
mod test_area_of_array{
    use dsa::*;

    #[test]
    fn test_example1(){
        assert_eq!((12, (1, 4)), calculate_area_of_array(vec![1, 5, 6, 3, 4]))
    }
    #[test]
    fn test_example2(){
        assert_eq!((35, (0, 5)), calculate_area_of_array(vec![10, 6, 5, 6, 5, 7]))
    }
}