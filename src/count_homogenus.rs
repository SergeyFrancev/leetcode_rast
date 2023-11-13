pub struct Solution {}

struct LitCounter {
    lit: char,
    count: i32,
}

impl LitCounter {
    fn new() -> Self {
        LitCounter { lit: '_', count: 0}
    }
}

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        Self::calc_count(s) as i32
    }

    fn calc_count(s: String) -> i64 {
        s.chars().into_iter().scan(LitCounter::new(), |acc, ch| {
            match ch == acc.lit {
                true => {
                    acc.count += 1;
                    Some(acc.count as i64)
                },
                false => {
                    acc.lit = ch;
                    acc.count = 1;
                    Some(1i64)
                },
            }
        }).sum::<i64>() % (1_000_000_007i64)
    }


    fn count_variants(len: i32) -> i32 {
        Self::modulo(len * (len + 1) / 2)
    }

    fn modulo(num: i32) -> i32 {
        let modulo = i32::pow(10, 9) + 7;
        if num > modulo {
            num % modulo
        } else {
            num
        }
    }

    pub fn count_homogenous_old(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        // let mut out = 0;
        let mut prev = s.chars().collect::<Vec<char>>()[0];
        let mut counter = 0;
        let mut out = s.chars().into_iter().fold(0, |acc, x| {
            if x == prev {
                counter += 1;
                return acc;
            }
            let summ = acc + Self::count_variants(counter);
            prev = x;
            counter = 1;

            Self::modulo(summ)
        });
        out += Self::count_variants(counter);
        Self::modulo(out)
    }
}

#[cfg(test)]
mod count_variants {
    use super::Solution;

    #[test]
    fn case1() {
        let sum = Solution::count_variants(5);
        assert_eq!(sum, 15);
    }

    #[test]
    fn case2() {
        let sum = Solution::count_variants(1);
        assert_eq!(sum, 1);
    }

    #[test]
    fn case3() {
        let sum = Solution::count_homogenous(String::from("abbcccaa"));
        assert_eq!(sum, 13);
    }
}
