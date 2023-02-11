use crate::Solution;

// @lc code=start
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        // Can form substring 0..i using word_dict
        let mut dp = vec![false; n + 1];
        // Can always form empty string
        dp[0] = true;

        for i in 1..(n + 1) {
            for word in word_dict.iter() {
                if word.len() > i || !dp[i - word.len()] {
                    continue;
                }

                let substring = &s[(i - word.len())..i];
                if substring == word {
                    dp[i] = true;
                }
            }
        }

        dp[n]
    }
}
// @lc code=end
