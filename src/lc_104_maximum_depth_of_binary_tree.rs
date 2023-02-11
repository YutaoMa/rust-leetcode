use crate::Solution;

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

// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let node = match root {
            None => return 0,
            Some(node) => node
        };

        let left_max_depth = Solution::max_depth(node.borrow().left.clone());
        let right_max_depth = Solution::max_depth(node.borrow().right.clone());
        1 + std::cmp::max(left_max_depth, right_max_depth)
    }
}
// @lc code=end
