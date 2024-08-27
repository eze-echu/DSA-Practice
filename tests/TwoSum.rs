#[cfg(test)]
mod two_sum_test {
    use dsa::*;

    #[test]
    fn test_two_sum() {
        let arr = vec![-1, 2, 10, 11, 15];
        let val = 9;
        let result = two_sum(&arr, val);
        assert_eq!(result, vec![0, 2]);
    }
    #[test]
    fn test_two_sum_negative() {
        let arr = vec![-1, 2, 7, 11, 15, -11];
        let val = -12;
        let result = two_sum(&arr, val);
        assert_eq!(result, vec![0, 5]);
    }
}
