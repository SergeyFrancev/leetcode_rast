pub struct Solution {

}

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut bottom_mans: HashSet<i32> = HashSet::from([]);
        for i in 0..n {
            if inform_time[i as usize] == 0 {
                bottom_mans.insert(manager[i as usize]);
            }
        }
        let mut delay_msg = vec![-1; manager.len()];
        let mut max_time = inform_time[head_id as usize];
        for m in bottom_mans.drain() {
            let mut to = m;
            let mut sum = 0;
            let mut path: Vec<i32> = Vec::from([]);
            while to != -1 {
                if delay_msg[to as usize] != -1 {
                  sum += delay_msg[to as usize];
                  break;
                }
                sum += inform_time[to as usize];
                path.push(to);
                to = manager[to as usize];
            }
            let mut total = match to {
                -1 => 0,
                _ => delay_msg[to as usize],
            };
            while path.len() > 0 {
                let item = path.pop().unwrap();
                delay_msg[item as usize] = total;
                total += inform_time[item as usize];
            }
            max_time = i32::max(max_time, sum);
            println!("DM: {:?}", delay_msg);
        }
        max_time
    }
}