pub struct Solution {}

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        for i in 0..arr.len() {
            let mut wins = 0;
            if i > 0 && arr[i-1] > arr[i] {
                wins += 1;
            }
            for j in (i+1)..arr.len() {
               if arr[j] < arr[i] {
                wins += 1;
               } else {
                break;
               }
               if wins == k {
                return arr[i]
               }
            }
        }
        *arr.iter().max().unwrap()
    }

    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let l = left.iter().max().unwrap_or(&0).to_owned();
        let r = n - right.iter().min().unwrap_or(&n);
        l.max(r)
    }
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut out: Vec<String> = Vec::from([]);
        let mut steam: i32 = 1;
        let mut cursor: usize = 0;

        while steam <= n && cursor < target.len() {
            out.push(String::from("Push"));
            steam += 1;

            if target[cursor] == steam {
                cursor += 1;
            } else {
                out.push(String::from("Pop"));
            }
        }

        out
    }
}
