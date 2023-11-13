use std::{collections::BinaryHeap, cmp::Reverse};

pub struct Solution {

}

impl Solution {
    pub fn eliminate_maximum2(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n = dist.len();
        let mut time_to_city: Vec<f64> = dist.iter().zip(speed.iter())
            .map(|(&d, &s)| d as f64 / s as f64)
            .collect();
        
        time_to_city.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        for i in 0..n {
            if time_to_city[i] <= i as f64 {
                return i as i32;
            }
        }
        
        n as i32
    }

    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut queue:BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for i in 0..dist.len() {
            let turn = (dist[i] as f64 / speed[i] as f64).ceil() as i32;
            queue.push(Reverse(turn))
        }

        let mut counter = 0;
        while queue.len() > 0 {
            let item = queue.pop().unwrap().0;
            if item <= counter {
                return counter;
            }
            counter += 1;
        }

        counter
    }

    pub fn test() {
        // let a = f64::ceil((5 as f64 / 3 as f64));
        let mut out = 0;
        for i in 1..=11 {
            out += i;
        }
        println!("Res: {}", out);
    }
}