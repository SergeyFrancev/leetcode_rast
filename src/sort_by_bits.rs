pub struct Solution {}

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        const MAX: i32 = 10_000;
        arr.sort_by_cached_key(|a| {
            let mut bit_len = 0;
            let mut max_num = 1;
            while a >= &max_num {
                max_num *= 2;
                bit_len += 1;
            }
            (0..(bit_len + 1)).rev().map(|n| (a >> n) & 1).reduce(|x, y| x + y).unwrap() * MAX + a
        });
        // let x = 7;
        // let mut bit_len = 0;
        // let mut max_num = 1;
        // while x >= max_num {
        //     max_num *= 2;
        //     bit_len += 1;
        // }
        // let b: Vec<i32> = (0..bit_len).rev().map(|n| (x >> n) & 1).collect();
        // println!("- {:?}", b);
        arr
        // for x in arr.iter() {
        //     let b: Vec<i32> = (0..32).rev().map(|n| (x >> n) & 1).collect();
        //     println!("- {:?}", b);
        // }
        // let x = 5;
        // let mut b = x >> 0;
        // println!("- {b:b}");
        // // b = x & !(1 << 0);
        // b = x | (1 << 1);
        // println!("- {b:b}");
        // // b = b & 1;
        // // println!("- {b:b}");
        // arr
    }
}
