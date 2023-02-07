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
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();

        let root = match root {
            Some(node) => node,
            None => return ans
        };

        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let len = queue.len();

            for i in 0..len {
                let node = queue.pop_front().unwrap();

                if i == len - 1 {
                    ans.push(node.borrow().val.clone());
                }

                if let Some(ref left) = node.clone().borrow().left {
                    queue.push_back(left.clone());
                }

                if let Some(ref right) = node.clone().borrow().right {
                    queue.push_back(right.clone());
                }
            }
        }

        ans
    }
}
// @lc code=end
