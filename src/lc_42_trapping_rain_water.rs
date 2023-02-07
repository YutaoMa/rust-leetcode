use crate::Solution;

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut max_height = vec![i32::MAX; height.len()];
        let mut left_max = 0;
        for (i, &h) in height.iter().enumerate() {
            left_max = left_max.max(h);
            max_height[i] = max_height[i].min(left_max);
        }
        let mut right_max = 0;
        for (i, &h) in height.iter().enumerate().rev() {
            right_max = right_max.max(h);
            max_height[i] = max_height[i].min(right_max);
        }

        max_height.iter().zip(&height).map(|(max_h, h)| max_h - h).sum()
    }
}
// @lc code=end
