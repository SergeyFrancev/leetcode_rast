pub struct Solution {}

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut vowes: String = String::with_capacity(s.len());
        let v = String::from("uoieaUOIEA");

        {
            let mut stack: Vec<String> = vec![String::new(); 10];
            for ch in s.chars() {
                let pos = v.find(ch);
                match pos {
                    Some(v) => stack[v].push(ch),
                    None => {},
                }
            }
            for list in stack.into_iter().as_mut_slice() {
                vowes.push_str(list);
            }
        }

        if vowes.len() < 2 {
            return s;
        }

        let mut out: String = String::with_capacity(s.len());
        for ch in s.chars().into_iter() {
            match vowes.contains(ch) {
                true => out.push(vowes.pop().unwrap()),
                _ => out.push(ch)
            }
        }

        out
    }
}