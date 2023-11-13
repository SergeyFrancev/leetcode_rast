pub struct Solution {}

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        use std::{collections::VecDeque, mem};
        if source == target {
            return 0;
        }

        // Build map[stop_id] = [route_id, ...]
        let mut map: Vec<Vec<u16>> = vec![Vec::new(); 1_000_000];
        for (route_num, route) in routes.iter().enumerate() {
            if route.len() == 1 {
                continue;
            }
            for stop in route {
                map[stop.clone() as usize].push(route_num.clone() as u16);
            }
        }

        // Check map contains target and source route
        if map[source as usize].len() == 0 || map[target as usize].len() == 0 {
            return -1;
        }

        let mut visited_stop: Vec<bool> = vec![false; 1_000_000];
        let mut visited_route: Vec<bool> = vec![false; 500];
        let mut stack: VecDeque<u16> = VecDeque::from([]);
        let mut next_stack: VecDeque<u16> = VecDeque::from([]);
        for route in map[source as usize].iter() {
            next_stack.push_front(route.clone());
        }

        let mut curr_cost = 1;

        while next_stack.len() > 0 {
            mem::swap(&mut next_stack, &mut stack);

            while stack.len() > 0 {
                let route = stack.pop_back().unwrap();
                visited_route[route as usize] = true;

                for bus_stop in routes[route as usize].iter() {
                    // Check visited to target bus stop
                    let bus_stop_idx = *bus_stop as usize;
                    if visited_stop[bus_stop_idx] {
                        continue;
                    }
                    if bus_stop == &target {
                        return curr_cost;
                    }
                    visited_stop[bus_stop_idx] = true;

                    // Stack routes for next iteration
                    for next_route in map[bus_stop_idx].iter() {
                        if visited_route[*next_route as usize] {
                            continue;
                        }
                        next_stack.push_front(*next_route);
                    }
                }
            }

            curr_cost += 1;
        }

        -1
    }
}
