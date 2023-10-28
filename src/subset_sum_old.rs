pub struct Path {
    length: usize,
    step: usize,
    path: Vec<usize>,
}

impl Path {
    fn new(length: i32, step: i32) -> Path {
        Path{ length: length as usize, step: step as usize, path: Vec::from([1]) }
    }

    fn len(&self) -> usize {
        self.path.len()
    }

    fn inc_node(&mut self, idx: usize) {
        self.path[idx] += 1;
        for j in (idx + 1)..self.len() {
            self.path[j] = 1;
        }
        return;
    }

    fn grow(&mut self) {
        if self.len() < self.length as usize {
            for i in 0..self.len() {
                self.path[i] = 1;
            }
            self.path.push(1);
            return;
        }

        self.path = Vec::from([]);
    }

    fn next(&mut self) {
        let max_step = Vec::from([self.step, self.length - (self.path.len() - 1)]).into_iter().min().unwrap();
        let sum: usize = self.path.iter().sum();

        if sum == self.length {
            let mut max_step_idx: usize = 0;
            let mut max = self.path[0];
            for i in 1..self.path.len() {
                if self.path[i] > max {
                    max_step_idx = i;
                    max = self.path[i];
                }
            }

            if max_step_idx == 0{
                if self.path[0] < max_step {
                    self.inc_node(0);
                } else {
                    self.grow();
                }
            } else {
                self.inc_node(max_step_idx - 1);
            }
            return;
        }

        // Try inc path
        if self.path[0] < max_step {
            for idx in 0..self.path.len() {
                let i = self.path.len() - 1 - idx;
                if self.path[i] < max_step {
                    self.inc_node(i);
                    return;
                }
            }
        }

        //else Try grow
        self.grow();
    }
}

pub struct Solution {}

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut out: i32 = nums.iter().sum();
        println!("Start with: {out}");
        let mut iter_count = 0;
        for i in 0..nums.len() {
            let mut path = Path::new((nums.len() - i) as i32, k);
            while path.len() > 0 {    
                let mut sum = 0;
                let mut curr: usize = i;
                println!("- {:?}", path.path);
                for (idx, i) in path.path.iter().enumerate() {
                    curr = match idx {
                        0 => curr + i - 1,
                        _ => curr + i,
                    };
                    if curr >= nums.len() {
                        break;
                    }
                    sum = sum + nums[curr];
                }
                out = Vec::from([out, sum]).into_iter().max().unwrap();
                println!("* Test: {sum}");
    
                path.next();
                iter_count = iter_count + 1;
            }
        }
        println!("Iterations: {iter_count}");
        return out
    }
}

#[cfg(test)]
mod subset_sum {
    use super::Solution;

    #[test]
    fn valid_path_is_ok() {
        let sum = Solution::constrained_subset_sum(Vec::from([10,2,-10,5,20]), 2);
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, 37);
    }

    #[test]
    fn case3() {
        let sum = Solution::constrained_subset_sum(Vec::from([10,-2,-10,-5,20]), 2);
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, 23);
    }

    #[test]
    fn case4() {
        let sum = Solution::constrained_subset_sum(Vec::from([-7609,249,-1699,2385,9125,-9037,1107,-8228,856,-9526]), 9);
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, 13722);
    }

    #[test]
    fn case5() {
        let sum = Solution::constrained_subset_sum(Vec::from([-8269,3217,-4023,-4138,-683,6455,-3621,9242,4015,-3790]), 1);
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, 16091);
    }
}