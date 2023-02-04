use crate::Solution;

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_str = x.to_string();
        x_str.chars().rev().eq(x_str.chars())
    }
}
// @lc code=end
