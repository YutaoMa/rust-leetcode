use crate::Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut product = nums[0];
        let mut max = product;
        let mut min = product;

        for i in 1..nums.len() {
            if nums[i] < 0 {
                std::mem::swap(&mut max, &mut min);
            }

            max = std::cmp::max(nums[i], max * nums[i]);
            min = std::cmp::min(nums[i], min * nums[i]);

            product = std::cmp::max(product, max);
        }

        product
    }
}
