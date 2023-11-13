pub struct Solution {

}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut provinces: Vec<Vec<i32>> = Vec::with_capacity(is_connected.len());
        for i in 0..is_connected.len() {
            
        }
        provinces.len() as i32
    }

    pub fn reverse_words(s: String) -> String {
        let mut out: Vec<String> = Vec::from([]);
        let mut stack = String::from("");
        for ch in s.as_bytes().into_iter() {
            match ch {
                b' ' => {
                    if stack.len() > 0 {
                        out.insert(0, stack.to_owned());
                    }
                    stack.clear();
                },
                _ => {stack.push(ch)},
            }
        }
        if stack.len() > 0 {
            out.insert(0, stack.to_owned());
        }
        out.join(" ")
    }
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(nums.len() / 2 + 1);
        for i in 0..nums.len() {
            match stack.binary_search(&nums[i]) {
                Ok(idx) => {
                    stack.remove(idx);
                }
                Err(idx) => {
                    stack.insert(idx, nums[i]);
                },
            }
            println!("- [{:?}] {:?}", nums[i], stack)
        }
        assert_eq!(stack.len(), 1);
        stack[0]
    }
}


#[cfg(test)]
mod single_number {
    use super::Solution;

    #[test]
    fn case1() {
        let sum = Solution::single_number(Vec::from([1,4,1]));
        assert_eq!(sum, 4);
    }

    #[test]
    fn case3() {
        let sum = Solution::single_number(Vec::from([4,-1, -1]));
        assert_eq!(sum, 4);
    }

    #[test]
    fn case4() {
        let sum = Solution::single_number(Vec::from([-5, 4, -7,-1, -7, -5, -1, 4, 5]));
        assert_eq!(sum, 5);
    }

    #[test]
    fn case2() {
        // let sum = Solution::single_number(Vec::from([354]));
        let sum = Solution::single_number(Vec::from([-336,513,-560,-481,-174,101,-997,40,-527,-784,-283,-336,513,-560,-481,-174,101,-997,40,-527,-784,-283,354]));
        assert_eq!(sum, 354);
    }
}