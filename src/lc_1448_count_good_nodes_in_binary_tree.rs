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
// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::good_nodes_dfs(&root, i32::MIN)
    }

    fn good_nodes_dfs(node: &Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
        let node = match node {
            Some(x) => x,
            None => return 0
        };

        let max = std::cmp::max(max, node.borrow().val);
        let left = Solution::good_nodes_dfs(&node.borrow().left, max);
        let right = Solution::good_nodes_dfs(&node.borrow().right, max);

        left + right + if node.borrow().val >= max { 1 } else { 0 }
    }
}
// @lc code=end
