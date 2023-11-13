// Definition for a binary tree node.
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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

struct Solution {

}
impl Solution {
    fn to_deep(root: Option<Rc<RefCell<TreeNode>>>, avg_count: &mut i32) -> (i32, i32) {
        if let Some(node) = root {
            let cell = node.borrow();
            let left = Self::to_deep(cell.left, avg_count);
            let right = Self::to_deep(cell.right, avg_count);

            let sum = left.0 + right.0 + cell.val;
            let count = left.1 + right.1 + 1;

            if (((sum/count) as f64).ceil() as i32) == cell.val {
                *avg_count = *avg_count + 1;
            }

            return (sum, count)
        }

        (0, 0)
    }

    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut out: i32 = 0;
        let _  = Self::to_deep(root, &mut out);
        out
    }
}