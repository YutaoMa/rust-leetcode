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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::is_valid_bst_dfs(&root, None, None)
    }

    fn is_valid_bst_dfs(node: &Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
        let node = match node {
            Some(x) => x,
            None => return true
        };

        let val = node.borrow().val;
        let greater_than_min = match min {
            None => true,
            Some(x) => val > x
        };
        let less_than_max = match max {
            None => true,
            Some(x) => val < x
        };
        if !greater_than_min || !less_than_max {
            return false
        }

        Solution::is_valid_bst_dfs(&node.borrow().left, min, Some(val)) && Solution::is_valid_bst_dfs(&node.borrow().right, Some(val), max)
    }
}
// @lc code=end
