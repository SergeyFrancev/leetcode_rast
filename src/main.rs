mod garbage_collection;
use garbage_collection::Solution;

// use std::collections::{VecDeque, HashMap};

fn create_deep_vec(d: [[i32; 2]; 9]) -> Vec<Vec<i32>> {
    let d = [
        [-3, -9],
        [-5, 3],
        [2, -9],
        [6, -3],
        [6, 1],
        [5, 3],
        [8, 5],
        [-5, 1],
        [7, 2],
    ];

    let mut data = Vec::from([]);
    for item in d {
        data.push(Vec::from(item));
    }
    data
}

fn get_vec(d: [&str; 4]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for i in d {
        out.push(i.to_string())
    }
    out
}

fn main() {
    let data = get_vec(["G","P","GP","GG"]);
    let t = Vec::from([2,4,3]);
    let out = Solution::garbage_collection(data, t);
    println!("Out: {out}");
    // let data = Vec::from([7,9,10,8,6,4,1,5,2,3]);
    // let out = Solution::reduction_operations(data);
    // println!("{:?}", out);
    // let out = Solution::max_frequency(data, 2636);
    // println!("Out: {}", out);
    // let s = 10 >> 1;
    // println!("{}", s)
    // Solution::find_different_binary_string(Vec::from([String::from("111")]));
    // let binary_string = "1111111111111111";

    // match usize::from_str_radix(binary_string, 2) {
    //     Ok(binary_value) => {
    //         println!("Binary String: {}", binary_string);
    //         println!("Equivalent Binary Value: {}", binary_value);
    //     }
    //     Err(err) => {
    //         eprintln!("Error: {}", err);
    //     }
    // }
    // let out = Solution::count_palindromic_subsequence(String::from("aabca"));
    // println!("{}", out);

    // let data = String::from("aabca");
    // for b in data.bytes().map(|byte| usize::from(byte - b'a')) {
    //     println!("{:?}", b)
    // }
    // let mut path: HashMap<i32, i32> = HashMap::new();
    // path.insert(1, 2);
    // path.insert(3, 4);
    // let target_cost = path.get_mut(&1);
    // *target_cost.unwrap() = 5;

    // println!("{:?}", path);
    // let mut data: Vec<Vec<char>> = Vec::from([
    //     Vec::from(['5', '3', '.', '.', '7', '.', '.', '.', '.']),
    //     Vec::from(['6', '.', '.', '1', '9', '5', '.', '.', '.']),
    //     Vec::from(['.', '9', '8', '.', '.', '.', '.', '6', '.']),
    //     Vec::from(['8', '.', '.', '.', '6', '.', '.', '.', '3']),
    //     Vec::from(['4', '.', '.', '8', '.', '3', '.', '.', '1']),
    //     Vec::from(['7', '.', '.', '.', '2', '.', '.', '.', '6']),
    //     Vec::from(['.', '6', '.', '.', '.', '.', '2', '8', '.']),
    //     Vec::from(['.', '.', '.', '4', '1', '9', '.', '.', '5']),
    //     Vec::from(['.', '.', '.', '.', '8', '.', '.', '7', '9']),
    // ]);
    // let mut data: Vec<Vec<char>> = Vec::from([
    //     Vec::from(['.', '.', '9', '7', '4', '8', '.', '.', '.']),
    //     Vec::from(['7', '.', '.', '6', '.', '2', '.', '.', '.']),
    //     Vec::from(['.', '2', '.', '1', '.', '9', '.', '.', '.']),
    //     Vec::from(['.', '.', '7', '9', '8', '6', '2', '4', '1']),
    //     Vec::from(['2', '6', '4', '3', '1', '7', '5', '9', '8']),
    //     Vec::from(['1', '9', '8', '5', '2', '4', '3', '6', '7']),
    //     Vec::from(['.', '.', '.', '8', '6', '3', '.', '2', '.']),
    //     Vec::from(['.', '.', '.', '4', '9', '1', '.', '.', '6']),
    //     Vec::from(['.', '.', '.', '2', '7', '5', '9', '.', '.']),
    // ]);
    // Solution::solve_sudoku(&mut data);
    // let mut root = TreeNode::new(0);
    // root.left = Some(Rc::new(RefCell::new(TreeNode {left: None, right: None, val: 0})));
    // let r = Some(Rc::new(RefCell::new(TreeNode {left: None, right: None, val: 2})));
    // root.right = Some(Rc::new(RefCell::new(TreeNode{left: None, right: r, val: 2})));

    // let res = Solution::find_mode(Some(Rc::new(RefCell::new(root))));

    // let res = Solution::sort_by_bits(Vec::from([5]));
    // let data = Vec::from([2,1,3,5,4,6,7]);
    // let data = 3;
    // let data = String::from('Let's take LeetCode contest');
    // let res = Solution::get_winner(data, 2);
    // println!('-> res: {:?}', res);
}
