use crate::Solution;

// @lc code=start
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = nums;        
        ans.extend_from_within(0..);
        ans
    }
}
// @lc code=end
