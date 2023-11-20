

pub struct Solution {}

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut counts: Vec<(i32, usize, usize)> = vec![(0, s.len(), 0); 26];

        let bites: Vec<u8> = s.bytes().map(|x| x - b'a').collect();
        for (idx, ch) in bites.iter().enumerate() {
            let mut item = counts[*ch as usize];
            item.0 += 1;
            item.1 = usize::min(item.1, idx);
            item.2 = usize::max(item.2, idx);
            counts[*ch as usize] = item
        }

        let mut out = 0;
        for pair in counts.iter() {
            if pair.0 < 2 {
                continue;
            }
            let mut variants = vec![false; 26];
            for i in (pair.1 + 1)..pair.2 {
                let bit = bites[i] as usize;
                if variants[bit] {
                    continue;
                }
                variants[bit] = true;
                out += 1;
            }
        }
        out
    }

    pub fn count_palindromic_subsequence2(s: String) -> i32 {
        // Find the indexes for the first and last occurrence
        // of each letter
        let mut spans: [Option<(usize, usize)>; 26] = [None; 26];
        for (i, letter) in s.bytes().map(|byte| usize::from(byte - b'a')).enumerate() {
            if let Some((first, _)) = spans[letter] {
                spans[letter] = Some((first, i));
            } else {
                spans[letter] = Some((i, i));
            }
        }

        // For each index i, find all letter spans that contain it
        // Mark the palindrome as seen in a binary bitmap
        let mut seen = [0_u32; 26];
        for (i, middle_letter) in s.bytes().map(|byte| usize::from(byte - b'a')).enumerate() {
            for (outer_letter, &maybe_span) in spans.iter().enumerate() {
                if let Some((left, right)) = maybe_span {
                    if left < i && right > i {
                        println!(" > {:?} [{}]", seen[outer_letter], outer_letter);
                        seen[outer_letter] |= (1 << middle_letter);
                        println!(" < {:?}", seen[outer_letter]);
                    }
                }
            }
        }

        // Count the number of 1 bits in the bitmap
        seen.into_iter().map(|row| row.count_ones()).sum::<u32>() as i32
    }
}
