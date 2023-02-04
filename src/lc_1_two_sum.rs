use crate::Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let other_num = target - *num;
            match seen.get(&other_num) {
                Some(&other_i) => return vec![other_i, i as i32],
                None => seen.insert(*num, i as i32)
            };
        }

        vec![]
    }
}
// @lc code=end
