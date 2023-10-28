use std::{cmp::{min, max}, ops::Range};

pub struct Solution {}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        // let mut stack: Vec<(usize, usize)> = Vec::from([(0, nums.len() - 1)]);

        let mut max_score = nums[k as usize];
        let mut min_score = nums[k as usize];

        // let sum = Solution::maximum_score(Vec::from([1,4,3,7,4,5]), 3);
        let mut range = Range{start: k as usize, end: k as usize};

        while (range.end - range.start) < nums.len() - 1 {
            let mut right = 0;
            if range.end + 1 < nums.len() {
                right = nums[range.end + 1];
            }
            let mut left = 0;
            if range.start > 0 {
                left = nums[range.start - 1];
            }

            if right >= left {
                while range.end + 1 < nums.len() && nums[range.end + 1] >= right {
                    range.end = range.end + 1;
                }
                min_score = min(right, min_score)
            }
            if left >= right {
                while range.start > 0 && nums[range.start - 1] >= left {
                    range.start = range.start - 1;
                }
                min_score = min(left, min_score)
            }

            let max_range = min_score * (range.end - range.start + 1) as i32;
            max_score = max(max_score, max_range);

            println!("- {:?} [{max_range}/{max_score}]", range);
        }
        max_score
    }
}

#[cfg(test)]
mod subset_sum {
    use super::Solution;

    #[test]
    fn case_max1() {
        let res = Solution::maximum_score(Vec::from([1,4,3,7,4,5]), 3);
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(res, 15);
    }

    #[test]
    fn case_max2() {
        let res = Solution::maximum_score(Vec::from([6569,9667,3148,7698,1622,2194,793,9041,1670,1872]), 5);
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(res, 9732);
    }


}