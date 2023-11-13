pub struct Solution {}

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        for i in 0..arr.len() {
            let mut wins = 0;
            if i > 0 && arr[i - 1] < arr[i] {
                wins += 1;
            }
            // if wins == k {
            //   return arr[i]
            // }
            // println!("Target: {} [w:{wins}]", arr[i]);
            for j in (i + 1)..arr.len() {
                if wins == k {
                    return arr[i];
                }
                // println!("- Check: {} : {}", arr[i], arr[j]);
                if arr[i] > arr[j] {
                    wins += 1;
                } else {
                    break;
                }
                // println!("- Result: {} [w:{wins}]", arr[i]);
                
            }
        }
        *arr.iter().max().unwrap()
    }
}
