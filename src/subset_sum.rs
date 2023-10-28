use std::ops::Range;

pub struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let w1: Vec<char> = word1.chars().into_iter().collect();
        let w2: Vec<char> = word2.chars().into_iter().collect();
        let mut out: Vec<char> = Vec::from([]);
        let max_len = usize::max(w1.len(), w2.len());
        for i in 0..max_len {
            if i < w1.len() {
                out.push(w1[i]);
            }
            if i < w2.len() {
                out.push(w2[i]);
            }
        }
        out.into_iter().collect()
        // for i in 
    }

    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut max_sum = i32::MIN;
        let mut cursor = 0;
        // Find start
        {
            let mut only_neg = true;
            let mut max_neg = max_sum;
            for (idx, i) in nums.iter().enumerate() {
                if i >= &0 {
                    only_neg = false;
                    cursor = idx;
                    break;
                } else if only_neg {
                    max_neg = i32::max(max_neg, i + 0);
                }
            }

            if only_neg {
                return max_neg;
            } else {
                max_sum = nums[cursor];
            }
        }
        
        // [100,-10,-10,-10,-2,-2,-10,-100,15,-5,-10,10,2,-10,5,20]
        let mut prev_sum = max_sum;
        while cursor + 1 < nums.len() {
            let range = Range {
                start: cursor + 1,
                end: usize::min(cursor + (k as usize), nums.len() - 1),
            };
            let part = &nums[range.start..=range.end];

            let mut last_idx = range.start;
            let mut part_summ = 0;
            let mut max_negative = i32::MIN;
            let mut only_neg = true;
            for (idx, i) in part.iter().enumerate() {
                if i >= &0 {
                    only_neg = false;
                    last_idx = range.start + idx;
                    part_summ = part_summ + i;
                } else if only_neg && i >= &max_negative {
                    last_idx = range.start + idx;
                    max_negative = i + 0;
                }
            }

            if only_neg {
                part_summ = max_negative
            }

            if part_summ + prev_sum <= part_summ {
                prev_sum = part_summ;
            } else {
                prev_sum = prev_sum + part_summ;
            }
            
            cursor = last_idx;
            println!("-> {cursor}");
            max_sum = i32::max(max_sum, prev_sum);
        }
        max_sum
    }
}

#[cfg(test)]
mod subset_sum {
    use super::Solution;

    #[test]
    fn valid_path_is_ok() {
        let sum = Solution::constrained_subset_sum(Vec::from([10, 2, -10, 5, 20]), 2);
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, 37);
    }

    #[test]
    fn case3() {
        let sum = Solution::constrained_subset_sum(Vec::from([10, -2, -10, -5, 20]), 2);
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, 23);
    }

    #[test]
    fn case4() {
        let sum = Solution::constrained_subset_sum(
            Vec::from([
                -7609, 249, -1699, 2385, 9125, -9037, 1107, -8228, 856, -9526,
            ]),
            9,
        );
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, 13722);
    }

    #[test]
    fn case5() {
        let sum = Solution::constrained_subset_sum(
            Vec::from([
                -8269, 3217, -4023, -4138, -683, 6455, -3621, 9242, 4015, -3790,
            ]),
            1,
        );
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, 16091);
    }

    #[test]
    fn case6() {// 100 - 2 - 10 + 15 + 20
        let sum = Solution::constrained_subset_sum( // 100 - 10 - 2 - 2 - 10 + 15 - 5 + 10 + 2 + 5 + 20
            Vec::from([100,-10,-10,-10,-2,-2,-10,-100,15,-5,-10,10,2,-10,5,20]),
            2,
        );
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, 125);
    }
}
