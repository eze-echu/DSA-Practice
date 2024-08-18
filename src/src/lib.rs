pub fn square_array(arr: Vec<i32>) -> Vec<i32>{
    
    // The following commented code is the first solution
    // It is a brute force attempt that has a complexity of O(n log n)
    // due to the fact that we are iterating through the array and sorting it
    // so O(n) + O(n log n) = O(n log n)
    // Every sorting operation has a complexity of O(n log n)
    // unless it's done while creating the array
    //
    // if arr.len() == 0 {
    //     return vec![];
    // }
    // let mut new_arr: Vec<i32> = arr.iter()
    //     .map(|x| {
    //         if x < &0 {
    //             (x * x) * -1
    //         } else {
    //             x * x
    //         }
    //     }).collect();
    // new_arr.sort();
    
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
