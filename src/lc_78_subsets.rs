use crate::Solution;

// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();

        Solution::subsets_backtrack(&nums, 0, &mut Vec::new(), &mut ans);

        ans
    }

    fn subsets_backtrack(nums: &Vec<i32>, index: usize, curr: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        ans.push(curr.clone());

        for i in index..nums.len() {
            curr.push(nums[i]);
            Solution::subsets_backtrack(nums, i + 1, curr, ans);
            curr.pop();
        }
    }
}
// @lc code=end
