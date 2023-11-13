pub struct Path {
    length: i32,
    step: i32,
    path: Vec<usize>,
}

impl Path {
    pub fn new(length: i32, step: i32) -> Path {
        Path{ length, step, path: Vec::from([1]) }
    }

    pub fn len(&self) -> usize {
        self.path.len()
    }

    pub fn next(&mut self) {
        // Try inc path
        for i in (self.len() - 1)..=0 {
            if self.path[i] < self.step as usize {
                self.path[i] += 1;
                for j in i + 1..self.len() {
                    self.path[j] = 1;
                }
                return;
            }
        }

        //else Try grow
        if self.len() < self.length as usize {
            for i in 0..self.len() {
                self.path[i] = 1;
            }
            self.path.push(1);
            return;
        }

        self.path = Vec::from([]);
    }
}

pub struct Solution {}

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut out: i32 = nums.iter().sum();
        let path = Path::new(nums.len() as i32, k);
        while path.len() > 0 {    
            let mut sum = 0;
            let mut curr: usize = 0;
            for i in path.path.iter() {
                curr = match curr {
                    0 => curr + i - 1,
                    _ => curr + i,
                };
                // curr = idx;
                if curr >= nums.len() {
                    break;
                }
                sum = sum + nums[curr];
            }
            out = Vec::from([out, sum]).into_iter().max().unwrap();
        }
        return out
    }

    pub fn backspace_compare(s: String, t: String) -> bool {
        fn apply_back(s: String) -> String {
            let mut out = String::from("");
            for ch in s.chars() {
                if ch == '#' {
                    out.pop();
                } else {
                    out.push(ch);
                }
            }
            out
        }

        apply_back(s) == apply_back(t)
    }

    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut top: i32 = 0;
        {
            let from = relations.iter().map(|x| x[0]).collect::<Vec<i32>>();
            let to = relations.iter().map(|x| x[1]).collect::<Vec<i32>>();
            for t in to {
                if !from.contains(&t) {
                    top = t;
                    break;
                }
            }
        }

        let mut qe: Vec<(i32, i32)> = Vec::new();
        qe.push((top, time[(top - 1) as usize]));
        let mut tail: Vec<i32> = Vec::new();
        while qe.len() > 0 {
            let item = qe.pop().unwrap();
            let mut has_rel = false;
            for rel in relations.iter() {
                match item.0 == rel[1] {
                    true => {
                        has_rel = true;
                        qe.push((rel[0], item.1 + time[(rel[0] - 1) as usize]))
                    }
                    false => {}
                }
            }
            if !has_rel {
                tail.push(item.1)
            }
        }
        tail.iter().max().unwrap().clone()
    }
}
