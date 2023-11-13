pub struct Solution {}

impl Solution {
    pub fn sort_vowels2(s: String) -> String {
        use std::collections::HashSet;
        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
        let mut all_vowels: Vec<_> = s.chars().filter(|c| vowels.contains(&c)).collect();
        all_vowels.sort_unstable();

        let (mut pos, mut res) = (0, String::with_capacity(s.len()));
        for c in s.chars() {
            if vowels.contains(&c) {
                res.push(all_vowels[pos]);
                pos += 1;
            } else {
                res.push(c);
            }
        }

        return res;
    }

    pub fn sort_vowels(s: String) -> String {
        use std::collections::VecDeque;

        let mut vowes: VecDeque<char> = VecDeque::with_capacity(s.len());
        {
            let mut stack: Vec<VecDeque<char>> = vec![VecDeque::new(); 10];
            for ch in s.chars() {
                match ch {
                    'A' => stack[0].push_back(ch),
                    'E' => stack[1].push_back(ch),
                    'I' => stack[2].push_back(ch),
                    'O' => stack[3].push_back(ch),
                    'U' => stack[4].push_back(ch),
                    'a' => stack[5].push_back(ch),
                    'e' => stack[6].push_back(ch),
                    'i' => stack[7].push_back(ch),
                    'o' => stack[8].push_back(ch),
                    'u' => stack[9].push_back(ch),
                    _ => {}
                }
            }
            for list in stack.into_iter().as_mut_slice() {
                vowes.append(list);
            }
        }

        if vowes.len() < 2 {
            return s;
        }

        let mut out: String = String::with_capacity(s.len());
        for ch in s.chars().into_iter() {
            match ch {
                'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => {
                    out.push(vowes.pop_front().unwrap());
                }
                _ => {
                    out.push(ch);
                }
            }
        }

        out
    }
}