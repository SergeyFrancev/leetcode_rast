pub struct Solution {}

impl Solution {

    pub fn find_array(mut pref: Vec<i32>) -> Vec<i32> {
        let mut left = pref[0];
        for i in 1..pref.len() {
            if i > 1 {
                left ^= pref[i-1]
            }
            pref[i] = left ^ pref[i];
        }

        pref
    }

    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut zero = 1;
        let mut len = 1;
        let mut out = vec![0, n + 1];

        for i in 1..(n as usize)+1 {
            if ((i - 1) >> 0) & 1 == 0 {
                // If i ends on 0
                zero -= 1;
            } else {
                // If i ends on 1
                if zero == 0 {
                    // if max 1
                    len += 1;
                    zero = len - 1;
                } else {

                }
            }
            out[i] = len - zero;
        }

        out
    }

    pub fn get_row(row_index: i32) -> Vec<i32> {
        use std::mem::swap;

        let mut out = Vec::with_capacity(row_index as usize);
        let mut next = Vec::with_capacity(row_index as usize);
        out.push(1);
        next.push(1);

        while (out.len() - 1) < row_index as usize {
            out.push(0);
            next.push(0);
            // let next_len = out.len() + 1;
            for i in 0..next.len() {
                if i == 0 || i == next.len() - 1 {
                    next[i] = 1;
                } else {
                    next[i] = out[i-1] + out[i];
                }
            }
            println!("- {:?}", next);
            swap(&mut out, &mut next);
        }

        out
    }
}
