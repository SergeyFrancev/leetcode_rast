pub struct Solution {

}

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let mut idx = k;
        let mut count_swap = 0;
        while idx >= 2 {
          let mut n = 1;
          while i32::pow(2, n + 1) < idx {
            n = n + 1;
          }
          idx = idx - i32::pow(2, n);
          count_swap = count_swap + 1
        }
        let mut out = match idx % 2 == 0 {
            true => 1,
            false => 0,
        };

        if count_swap % 2 == 1 {
            out = (out + 1) % 2
        }

        out
    }
}