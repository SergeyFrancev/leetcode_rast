use criterion::{Criterion, black_box, criterion_group, criterion_main};

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

        let mut vowes: Vec<char> = Vec::with_capacity(s.len());
        {
            let mut stack: Vec<Vec<char>> = vec![Vec::new(); 10];
            for ch in s.chars() {
                match ch {
                    'A' => stack[9].push(ch),
                    'E' => stack[8].push(ch),
                    'I' => stack[7].push(ch),
                    'O' => stack[6].push(ch),
                    'U' => stack[5].push(ch),
                    'a' => stack[4].push(ch),
                    'e' => stack[3].push(ch),
                    'i' => stack[2].push(ch),
                    'o' => stack[1].push(ch),
                    'u' => stack[0].push(ch),
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
                    out.push(vowes.pop().unwrap());
                }
                _ => {
                    out.push(ch);
                }
            }
        }

        out
    }

    pub fn sort_vowels3(s: String) -> String {
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

    pub fn sort_vowels4(s: String) -> String {
        let vowes = String::from("uoieaUOIEA");

        let mut store: String = String::with_capacity(s.len());
        let mut consonsnts: Vec<(String, usize)> = Vec::new();
        {
            let mut cursor: (String, usize) = (String::new(), 0);
            let mut stack: Vec<String> = vec![String::new(); 10];
            for ch in s.chars().into_iter() {
                let pos = vowes.find(ch);
                match pos {
                    Some(v) => {
                        stack[v].push(ch);
                        cursor.1 += 1;
                    },
                    None => {
                        if cursor.1 > 0 {
                            consonsnts.push(cursor);
                            cursor = (String::new(), 0);
                        }
                        cursor.0.push(ch);
                    },
                }
            }
            if cursor.0.len() > 0 || cursor.1 > 0 {
                consonsnts.push(cursor);
            }
            for list in stack.into_iter().as_mut_slice() {
                store.push_str(list);
            }
        }

        if store.len() < 2 {
            return s;
        }

        let mut out: String = String::with_capacity(s.len());
        while consonsnts.len() > 0 {
            let curr = consonsnts.pop().unwrap();
            out.push_str(&curr.0);
            for i in 0..curr.1 {
                out.push(store.pop().unwrap());
            }
        }
        // for cons in consonsnts.into_iter() {
        //     out.push_str(&cons.0);
        //     for i in 0..cons.1 {
        //         out.push(store.pop().unwrap());
        //     }
        // }

        out
    }
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("my only string", |b| {
        b.iter(|| {
            let s = black_box(String::from("KeIZLJPAaBAFSuraCXHPzxlgPVnHZNrG"));
            Solution::sort_vowels3(s);
        })
    });
    c.bench_function("wiyh sort", |b| {
        b.iter(|| {
            let s = black_box(String::from("KeIZLJPAaBAFSuraCXHPzxlgPVnHZNrG"));
            Solution::sort_vowels2(s);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);