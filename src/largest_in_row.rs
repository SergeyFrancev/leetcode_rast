#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

pub struct Solution {}
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut out: Vec<i32> = Vec::from([]);
        if root.is_none() {
            return out;
        }
        let mut curr_lvl: Vec<Rc<RefCell<TreeNode>>> = Vec::from([]);
        let mut next_lvl: Vec<Rc<RefCell<TreeNode>>> = Vec::from([root.unwrap()]);

        while next_lvl.len() > 0 {
            curr_lvl = next_lvl.to_owned();
            next_lvl = Vec::from([]);

            let mut max_val = i32::MIN;
            while curr_lvl.len() > 0 {
                let item: TreeNode = Rc::try_unwrap(curr_lvl.pop().unwrap()).unwrap().into_inner();
                max_val = max(max_val, item.val);
                if item.left.is_some() {
                    next_lvl.push(item.left.unwrap())
                }
                if item.right.is_some() {
                    next_lvl.push(item.right.unwrap())
                }
            }

            out.push(max_val);
        }
        // root.
        out
    }
}
