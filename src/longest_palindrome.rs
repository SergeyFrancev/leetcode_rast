use std::ops::Range;

pub struct Solution {}

impl Solution {
    pub fn max_palindrome(s: &Vec<char>, mut range: Range<usize>) -> Range<usize> {
        while range.start > 0 && range.end < s.len() {
            if s[range.start - 1] == s[range.end] {
                range.start -= 1;
                range.end += 1;
            } else {
                break;
            }
        }
        range
    }

    pub fn max_range(range1: Range<usize>, range2: Range<usize>) -> Range<usize> {
        if range2.end - range2.start > range1.end - range1.start {
            return range2;
        }
        range1
    }

    pub fn longest_palindrome(s: String) -> String {
        let mut max_range = Range { start: 0, end: 1 };
        // let mut cursor: usize = 0;
        let chars: Vec<char> = s.chars().collect();
        for cursor in 0..s.len() * 2 {
            let range = Solution::max_palindrome(&chars, cursor / 2..cursor / 2 + cursor % 2);
            max_range = Solution::max_range(max_range, range);
        }
        s[max_range].to_string()
    }
}

#[cfg(test)]
mod longest_palindrome {
    use super::Solution;

    #[test]
    fn case1() {
        let sum = Solution::longest_palindrome(String::from("babad"));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, String::from("bab"));
    }

    #[test]
    fn case2() {
        let sum = Solution::longest_palindrome(String::from("cbbd"));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, String::from("bb"));
    }

    #[test]
    fn case3() {
        let sum = Solution::longest_palindrome(String::from("cbbdqwertytrewq"));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, String::from("qwertytrewq"));
    }

    #[test]
    fn case4() {
        let sum = Solution::longest_palindrome(String::from("bb"));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, String::from("bb"));
    }

    #[test]
    fn case5() {
        let sum = Solution::longest_palindrome(String::from("ccc"));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, String::from("ccc"));
    }
}
