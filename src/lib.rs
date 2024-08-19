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
pub fn square_array(arr: Vec<i32>) -> Vec<i32>{
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
    // The current implementation has a complexity of O(n)
    // because it iterates through the array once
    if arr.is_empty() {
        return vec![];
    }
    let mut new_arr: Vec<i32> = vec![0; arr.len()];
    let mut a = 0;
    let mut z = arr.len() - 1;
    for i in (0..arr.len()).rev(){
        if arr[a].abs() > arr[z].abs() {
            new_arr[i] = arr[a].abs().pow(2);
            a += 1;
        } else {
            new_arr[i] = arr[z].abs().pow(2);
            z = z.saturating_sub(1);
        }
    }
    new_arr
}
///
/// monotonic_array checks if a given vector of integers is monotonic.
/// A vector is considered monotonic if it is either entirely non-increasing or non-decreasing.
///
/// # Arguments
/// * `arr` - A vector of integers
///
/// # Returns
/// * `bool` - Returns `true` if the vector is monotonic, otherwise `false`
///
/// # Example
/// ```rust
/// use dsa::monotonic_array;
/// let arr = vec![1, 2, 2, 3];
/// assert_eq!(monotonic_array(arr), true);
/// let arr = vec![6, 5, 4, 4];
/// assert_eq!(monotonic_array(arr), true);
/// let arr = vec![1, 3, 2];
/// assert_eq!(monotonic_array(arr), false);
/// ```
///
/// # Complexity
/// The current implementation has a complexity of O(n)
pub fn monotonic_array(arr: Vec<i32>) -> bool {
    let mut md = true;
    let mut mi = true;
    for i in 1..arr.len() {
        if arr[i] > arr[i-1] {
            md = false;
        } else if arr[i] < arr[i-1] {
            mi = false;
        }
    }
    mi || md
}
/// rotate_array rotates a given vector of integers to the right by k steps.
/// The rotation is done in-place and does not return anything.
///
/// # Arguments
/// * `arr` - A mutable reference to a vector of integers
/// * `k` - The number of steps to rotate the vector by
///
/// # Example
/// ```rust
/// use dsa::rotate_array;
/// let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
/// rotate_array(&mut arr, 3);
/// assert_eq!(arr, vec![5, 6, 7, 1, 2, 3, 4]);
/// ```
///
/// # Complexity
/// The current implementation has a complexity of O(n)
/// because it iterates through the array once
pub fn rotate_array(arr: &mut Vec<i32>, k: i32) {
    if k != 0{
        // This is the old version of the function which has a time complexity of O(n)
        // but is wildly inefficient
        //
        // let mut new_arr: Vec<i32> = vec![0; arr.len()];
        // new_arr.splice(..=rotate_by, arr[rotate_by - 1..].to_vec());
        // new_arr.splice(rotate_by.., arr[..rotate_by - 1].to_vec());

        // arr.clear();
        // arr.append(&mut new_arr)
        let rotation = k as usize % arr.len();
        arr.reverse();
        arr[..rotation].reverse();
        arr[rotation..].reverse();
    }
}