use std::rc::Rc;
use std::cell::RefCell;
// #[derive(Debug, PartialEq, Eq)]
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
      right: None
    }
  }
}

pub struct Solution {

}

impl Solution {
    fn process(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<(i32, Option<i32>)> {
        if let Some(node) = root {
            let mut out: Option<i32> = None;

            let node = node.borrow();

            if let Some(right) = Self::process(&node.right) {
                let mut diff = i32::abs(right.0 - node.val);
                if let Some(min_right) = right.1 {
                    diff = i32::min(diff, min_right);
                }
                out = Some(diff);
            }

            if let Some(left) = Self::process(&node.left) {
                let mut diff = i32::abs(left.0 - node.val);
                if let Some(min_left) = left.1 {
                    diff = i32::min(diff, min_left);
                }
                if let Some(right_min) = out {
                    diff = i32::min(diff, right_min);
                }
                out = Some(diff);
            }

            return Some((node.val, out));
        }
        
        None
    }

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let result = Self::process(&root);
        result.unwrap().1.unwrap()
    }
}