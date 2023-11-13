use std::collections::{HashMap, VecDeque};

pub struct Graph {
    nodes: usize,
    // ways: HashMap<i32, Vec<Vec<i32>>>,
    ways: Vec<Vec<(usize, i32)>>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut inst = Self {
            ways: vec![Vec::new(); n as usize],
            nodes: n as usize,
        };
        inst.process_ways(edges);
        inst
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        // assert!(
        //     edge[0] < self.nodes && edge[1] < self.nodes,
        //     "Node IDX out of range"
        // );
        let mut ways = self.ways[edge[0] as usize].push((edge[1] as usize, edge[2]));
        // let exist_way = ways.iter().find(|way| way[0] == edge[1]);
        // if exist_way.is_none() {
        //     ways.push(Self::create_way(edge))
        // } else {
        //     panic!("way is exist")
        // }
    }

    fn shortest_path(&self, from_node: i32, to_node: i32) -> i32 {
        let from = from_node as usize;
        let to = to_node as usize;
        if from >= self.nodes || to >= self.nodes {
            return -1;
        }
        if from == to {
            return 0;
        }
        let mut costs = vec![-1; self.nodes];

        let mut queue: VecDeque<usize> = VecDeque::from([from as usize]);
        costs.insert(from as usize, 0);

        // let no_ways: Vec<Vec<i32>> = Vec::from([]);
        let mut exist_target_cost = -1;
        while queue.len() > 0 {
            let target = queue.pop_back().unwrap();
            let cur_cost = costs[target];

            for way in self.ways[target].iter() {
                let target = way.0;
                let new_cost_to = cur_cost + way.1;

                // Save result cost if now in target node
                if target == to && (exist_target_cost == -1 || new_cost_to < exist_target_cost) {
                    exist_target_cost = new_cost_to;
                }

                // No repeat if exist path with min cost or cost more then exist
                // let target_cost = path.get(&target).unwrap_or(&i32::MAX).clone();
                let target_cost = costs[target];
                if target_cost != -1 && target_cost <= new_cost_to {
                    continue;
                }
                // (exist_target_cost != -1 && exist_target_cost <= new_cost_to)
                if exist_target_cost == -1 || exist_target_cost >= new_cost_to {
                    costs[target] = new_cost_to;
                    queue.push_front(target);
                }
            }
        }
        exist_target_cost
    }

    fn process_ways(&mut self, edges: Vec<Vec<i32>>) {
        for edg in edges {
            self.add_way(edg);
        }
    }

    fn add_way(&mut self, way: Vec<i32>) {
        self.ways[way[0] as usize].push((way[1] as usize, way[2]));
            // .entry(way[0])
            // .or_insert(Vec::from([]))
            // .push(Self::create_way(way));
    }

    // fn create_way(edge: Vec<i32>) -> Vec<i32> {
    //     let mut way: Vec<i32> = Vec::from([edge[1], edge[2]]);
    //     way.shrink_to_fit();
    //     way
    // }
}
