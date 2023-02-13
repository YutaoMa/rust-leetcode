use crate::Solution;

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

// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn kth_smallest(mut root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        // Find smallest val
        while let Some(node) = root {
            stack.push(node.clone());
            root = node.borrow().left.clone();
        }

        while k != 0 {
            // While k-- > 0, find next smallest val
            // Next smallest val = stack.pop -> all the way left -> repeat
            if let Some(node) = stack.pop() {
                k -= 1;
                if k == 0 {
                    return node.borrow().val;
                }
                let mut next_node = node.borrow().right.clone();
                while let Some(node) = next_node {
                    stack.push(node.clone());
                    next_node = node.borrow().left.clone();
                }
            }
        }

        unreachable!()
    }
}
// @lc code=end
