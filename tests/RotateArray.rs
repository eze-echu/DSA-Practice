#[cfg(test)]
mod tests{
    use dsa::*;

    #[test]
    fn test_basic(){
        let mut result = vec![1,2,3,4,5];
        rotate_array(&mut result, 3);
        assert_eq!(vec![3,4,5,1,2], result)
    }
    #[test]
    fn test_overflow(){
        let mut result = vec![1,2,3,4,5];
        rotate_array(&mut result, 8);
        assert_eq!(vec![3,4,5,1,2], result)
    }
    #[test]
    fn test_zero(){
        let mut result = vec![1,2,3,4,5];
        rotate_array(&mut result, 0);
        assert_eq!(vec![1,2,3,4,5], result)
    }
}