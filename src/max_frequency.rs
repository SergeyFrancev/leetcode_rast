pub struct Solution {}

// struct Bucket {
//     start: usize,
//     end: usize,
//     data: Vec<(usize, usize)>,
//     water_flow: usize,
// }

// impl Bucket {
//     fn new(data: Vec<(usize, usize)>) -> Self {
//         Self {
//             data,
//             start: 0,
//             end: 0,
//             water_flow: data[0].0,
//         }
//     }
// }

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut counters: Vec<usize> = vec![0; 10_000 + 1];
        for i in nums {
            counters[i as usize] += 1;
        }
        let mut data: Vec<(usize, usize)> = counters
            .into_iter()
            .enumerate()
            .filter(|(val, count)| count > &0)
            .collect();

        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut elements = data[start].1;
        let mut out = elements;
        let mut curry = k;

        loop {
            let new_end = end + 1;
            if new_end >= data.len() {
                break;
            }

            let diff = data[new_end].0 - data[end].0;

            // Try move end to right
            if (diff * elements) as i32 <= curry {
                curry -= (diff * elements) as i32;
                elements += data[new_end].1;
                out = usize::max(out, elements);
                end = new_end;
                continue;
            }

            // Try decress count minimal value
            let left_volume = (data[end].0 - data[start].0) * data[start].1;
            curry += left_volume as i32;
            elements -= data[start].1;

            if (diff * elements) as i32 >= curry {
                start += 1;
            } else {
                let left_curry = curry as usize - (diff * elements);
                let new_diff = data[new_end].0 - data[start].0;
                let new_start_count = ((left_curry / new_diff) as f64).floor() as usize;
                if new_start_count > 0 {
                    curry -= (diff * elements) as i32;
                    elements += new_start_count + data[new_end].1;
                    data[start].1 = new_start_count;
                    curry -= (new_start_count * new_diff) as i32;
                    out = usize::max(out, elements);
                    end = new_end;
                } else {
                    start += 1;
                }
            }
        }

        out as i32
    }
}
