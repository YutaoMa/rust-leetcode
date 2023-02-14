use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// @lc code=start
impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut curr = &mut head;

        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry > 0 {
            if let Some(node1) = l1 {
                carry += node1.val;
                l1 = node1.next;
            }

            if let Some(node2) = l2 {
                carry += node2.val;
                l2 = node2.next;
            }

            curr.next = Some(Box::new(ListNode::new(carry % 10)));
            curr = curr.next.as_mut().unwrap();
            carry /= 10;
        }

        head.next
    }
}
// @lc code=end
