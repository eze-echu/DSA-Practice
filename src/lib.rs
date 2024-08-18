/// square_array takes a vector of integers and returns a vector of integers
/// The returned vector contains the squares of the integers in the input vector
/// The returned vector is sorted in ascending order
/// The input vector is sorted and does take into account negatives
///
/// # Arguments
/// * `arr` - A vector of sorted integers
///
/// # Example
/// ```rust
/// use dsa::square_array;
/// let arr = vec![-4, -1, 0, 3, 10];
/// let result = square_array(arr);
/// assert_eq!(result, vec![0, 1, 9, 16, 100]);
/// ```
///
/// # Complexity
/// The current implementation has a complexity of O(n)
/// because it iterates through the array once
//* The following commented code is the first solution
//*     It is a brute force attempt that has a complexity of O(n log n)
//*     due to the fact that we are iterating through the array and sorting it
//*     so O(n) + O(n log n) = O(n log n)
//*     Every sorting operation has a complexity of O(n log n)
//*     unless it's done while creating the array
//*
//*     if arr.len() == 0 {
//*         return vec![];
//*     }
//*     let mut new_arr: Vec<i32> = arr.iter()
//*         .map(|x| {
//*             if x < &0 {
//*                 (x * x) * -1
//*             } else {
//*                 x * x
//*             }
//*         }).collect();
//*     new_arr.sort();
//*
pub fn square_array(arr: Vec<i32>) -> Vec<i32>{
    // The current implementation has a complexity of O(n)
    // because it iterates through the array once
    if arr.is_empty() {
        return vec![];
    }
    let mut new_arr: Vec<i32> = vec![0; arr.len()];
    let mut a = 0;
    let mut z = arr.len() - 1;
    for i in 0..arr.len() {
        if arr[a].abs() > arr[z].abs() {
            new_arr[arr.len() - 1 - i] = arr[a].abs().pow(2);
            a += 1;
        } else {
            new_arr[arr.len() - 1 - i] = arr[z].abs().pow(2);
            z = z.saturating_sub(1);
        }
    }
    new_arr
}
pub fn monotonic_array(){}
