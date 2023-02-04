use crate::Solution;

impl Solution {
    pub fn hamming_weight (mut n: u32) -> i32 {
        // Solution 1
        // n.count_ones() as i32       

        // Solution 2
        let mut ans = 0;

        while n != 0 {
            n &= n - 1;
            ans += 1;
        }

        ans
    }
}
