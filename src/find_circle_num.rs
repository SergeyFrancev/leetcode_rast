pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        // use std::collections::HashSet;

        let mut visited = vec![false; is_connected.len()];

        fn walk(idx: usize, c: &i32, v: &mut Vec<bool>) -> Option<usize> {
            if c == &1 && v[idx] == false {
                v[idx] = true;
                Some(idx)
            } else {
                None
            }
        }

        let mut circles = 0;
        while let Some(cur) =
            visited
                .iter()
                .enumerate()
                .find_map(|(idx, &v)| if v == false { Some(idx) } else { None })
        {
            visited[cur] = true;
            let mut to_visit: HashSet<_> = is_connected[cur]
                .iter()
                .enumerate()
                .filter_map(|(idx, &c)| walk(idx, &c, visited.as_mut()))
                .collect();
            while !to_visit.is_empty() {
                to_visit = to_visit
                    .into_iter()
                    .flat_map(|v| is_connected[v].iter().enumerate())
                    .filter_map(|(idx, &c)| walk(idx, &c, visited.as_mut()))
                    .collect();
            }

            circles += 1;
        }
        circles
    }
}

pub struct Solution2 {}

impl Solution2 {
    fn find_province(provinces: &Vec<HashSet<i32>>, city: &i32) -> Option<usize> {
        for i in 0..provinces.len() {
            if provinces[i].contains(city) {
                return Some(i);
            }
        }
        None
    }

    fn get_province(provinces: &mut Vec<HashSet<i32>>, city: &i32) -> usize {
        match Solution2::find_province(provinces, city) {
            Some(i) => i,
            None => {
                provinces.push(HashSet::from([city.to_owned()]));
                provinces.len() - 1
            }
        }
    }

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut provinces: Vec<HashSet<i32>> = Vec::with_capacity(is_connected.len());
        for i in 0..is_connected.len() {
            let mut province_idx = Solution2::get_province(&mut provinces, &(i as i32));
            for j in (i + 1)..is_connected.len() {
                if i != j && is_connected[i][j] == 1 {
                    match Solution2::find_province(&provinces, &(j as i32)) {
                        Some(connect_idx) => {
                            if connect_idx == province_idx {
                                continue;
                            }
                            let tmp: Vec<i32> = provinces[province_idx].drain().collect();
                            for p in tmp {
                                provinces[connect_idx].insert(p);
                            }
                            provinces[province_idx] = HashSet::from([]);
                            province_idx = connect_idx;
                        }
                        None => {
                            provinces[province_idx].insert(j as i32);
                        }
                    };
                }
            }
        }

        provinces
            .iter()
            .fold(0, |acc, x| acc + (x.len() > 0) as i32)
    }
}

#[cfg(test)]
mod find_circle_num {
    use super::Solution;

    #[test]
    fn case1() {
        let sum = Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]);
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, 2);
    }
}
