use crate::Solution;

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum: i32 = nums[0];
        let mut max_sum: i32 = sum;

        for i in 1..nums.len() {
            sum = nums[i] + sum.max(0);
            max_sum = max_sum.max(sum);
        }

        max_sum
    }
}
// @lc code=end
