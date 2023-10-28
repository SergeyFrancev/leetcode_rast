use std::collections::HashMap;

pub struct Solution {}

struct Collect {
    length: usize,
    ends: HashMap<char, usize>,
    modulo: usize,
    step: HashMap<char, usize>,
}

impl Collect {
    fn new() -> Collect {
        let mut collect = Collect {
            modulo: 1_000_000_007,
            length: 1,
            ends: HashMap::from([('a', 1), ('e', 1), ('i', 1), ('o', 1), ('u', 1)]),
            step: HashMap::from([('a', 0), ('e', 0), ('i', 0), ('o', 0), ('u', 0)]),
        };
        collect.ends.shrink_to_fit();
        collect.ends.shrink_to_fit();
        collect
    }

    fn next(&mut self) {
        self.length += 1;
        self.step.insert(
            'a',
            self.ends.get(&'e').unwrap()
                + self.ends.get(&'i').unwrap()
                + self.ends.get(&'u').unwrap(),
        );
        self.step.insert(
            'e',
            self.ends.get(&'a').unwrap() + self.ends.get(&'i').unwrap(),
        );
        self.step.insert(
            'i',
            self.ends.get(&'e').unwrap() + self.ends.get(&'o').unwrap(),
        );
        self.step.insert('o', self.ends.get(&'i').unwrap() + 0);
        self.step.insert(
            'u',
            self.ends.get(&'i').unwrap() + self.ends.get(&'o').unwrap(),
        );
        for (char, count) in self.step.iter() {
            if count > &self.modulo {
                self.ends.insert(*char, count % self.modulo);
            } else {
                self.ends.insert(*char, *count);
            }
        }
    }

    fn total(&self) -> i32 {
        self.ends
            .values()
            .fold(0, |acc, e| {
                let out = acc + e;
                if out > self.modulo {
                    return out % self.modulo;
                }
                out
            })
            .to_owned() as i32
    }
}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(nums.len() / 2 + 1);
        for i in 0..nums.len() {
            match stack.binary_search(&nums[i]) {
                Ok(idx) => {
                    stack.remove(idx);
                }
                Err(idx) => {
                    let new_idx = stack.len();
                    stack.push(nums[i]);
                    stack.swap(idx, new_idx);
                },
            }
        }
        assert_eq!(stack.len(), 1);
        stack[0]
    }

    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut out: Vec<i32> = vec![0; temperatures.len()];
        for start in 0..temperatures.len() - 1 {
            for end in start + 1..temperatures.len() {
                if temperatures[end] > temperatures[start] {
                    out[start] = (end - start) as i32;
                    break;
                }
            }
        }
        out
    }
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut collect = Collect::new();
        while n as usize > collect.length {
            collect.next();
        }
        collect.total()
    }
}

#[cfg(test)]
mod count_vowel_permutation {
    use super::Solution;

    #[test]
    fn case1() {
        let sum = Solution::count_vowel_permutation(1);
        assert_eq!(sum, 5);
    }

    #[test]
    fn case2() {
        let sum = Solution::count_vowel_permutation(2);
        assert_eq!(sum, 10);
    }

    #[test]
    fn case3() {
        let sum = Solution::count_vowel_permutation(1000);
        assert_eq!(sum, 89945857);
    }

    #[test]
    fn case4() {
        let sum = Solution::count_vowel_permutation(5);
        assert_eq!(sum, 68);
    }
}
