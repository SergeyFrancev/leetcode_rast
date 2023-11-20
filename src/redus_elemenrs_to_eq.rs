pub struct Solution {}

impl Solution {
    pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
        let mut counter: Vec<u16> = vec![0; 5 * 10_000 + 1];
        let mut min: u16 = 5 * 10_000 + 1;
        let mut max: u16 = 0;
        for i in nums.drain(..) {
            counter[i as usize] += 1;
            min = u16::min(min, i as u16);
            max = u16::max(max, i as u16);
        }
        counter.truncate((max + 1) as usize);

        let mut out: i32 = 0;
        let mut next_step = (min..=max)
            .rev()
            .into_iter()
            .filter(|i| counter[*i as usize] > 0);
        let mut max = next_step.next().unwrap();
        let mut count = counter[max as usize];

        while let Some(to) = next_step.next() {
            out += count as i32;
            count += counter[to as usize];
            max = to;
        }

        out as i32
    }
}
