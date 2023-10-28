use std::cmp::{min, max};

pub struct Solution {}

impl Solution {
    fn min(nums: &Vec<i32>, range: &(usize, usize)) -> i32 {
        let mut m = 20_000;
        for idx in range.0..=range.1 {
            if idx < nums.len() {
                m = min(m, nums[idx])
            }
        }
        m
    }

    fn calc_score(nums: &Vec<i32>, range: &(usize, usize)) -> i32 {
        ((range.1 + 1 - range.0) as i32) * Solution::min(nums, range)
    }

    fn slice_by_min(nums: &Vec<i32>, range: (usize, usize)) -> Vec<(usize, usize)> {
        let mut out: Vec<(usize, usize)> = Vec::from([]);
        if range.1 + 1 - range.0 <= 1 {
            return out;
        }
        let min = Solution::min(nums, &range);
        let mut part: (usize, usize) = (0, 0);
        
        for idx in range.0..=range.1 {
            let item = nums[idx];
            if item == min {
                if idx > 0 && idx - 1 >= part.0 {
                    part.1 = idx - 1;
                    out.push(part);
                }
                if idx + 1 < nums.len() {
                    part = (idx + 1, 0)
                }
            } else if idx == range.1 && idx >= part.0 {
                part.1 = idx;
                out.push(part)
            }
        }

        
        out
    }

    fn split_by_min(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut out: Vec<Vec<i32>> = Vec::from([]);
        if nums.len() == 0 {
            return out;
        }
        let min = nums.iter().min().unwrap().clone();
        let mut part: Vec<i32> = Vec::from([]);
        
        for i in nums.into_iter() {
            if i == min {
                if part.len() > 0 {
                    out.push(part);
                    part = Vec::from([]);
                }
            } else {
                part.push(i)
            }
        }

        if part.len() > 0 {
            out.push(part)
        }
        out
    }

    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let mut stack: Vec<(usize, usize)> = Vec::from([(0, nums.len() - 1)]);
        let mut max_score = 0;
        while stack.len() > 0 {
            let next_range = stack.pop().unwrap();
            let score = Solution::calc_score(&nums, &next_range);
            max_score = max(max_score, score);
            for range in Solution::slice_by_min(&nums, next_range) {
                if range.0 <= k as usize && range.1 >= k as usize {
                    stack.push(range)
                }
            }
            println!("- {:?} => {score}/{max_score}", next_range);
        }
        max_score
    }
}

#[cfg(test)]
mod subset_sum {
    use super::Solution;

    #[test]
    fn valid_path_is_ok() {
        let res = Solution::split_by_min(Vec::from([1,2,1,1,2]));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(res, Vec::from([[2], [2]]));
    }

    #[test]
    fn case_slice1() {
        let res = Solution::slice_by_min(&Vec::from([1,2,1,1,2]), (0,4));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(res, Vec::from([(1,1), (4,4)]));
    }

    #[test]
    fn case_slice2() {
        let res = Solution::slice_by_min(&Vec::from([2,2,2,2,2]), (0,4));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(res, Vec::from([]));
    }

    #[test]
    fn case_slice3() {
        let res = Solution::slice_by_min(&Vec::from([2,2,1,2,2]), (0,4));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(res, Vec::from([(0,1), (3,4)]));
    }

    #[test]
    fn case_slice4() {
        let res = Solution::slice_by_min(&Vec::from([2,2,1,2,1]), (0,4));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(res, Vec::from([(0,1), (3,3)]));
    }

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