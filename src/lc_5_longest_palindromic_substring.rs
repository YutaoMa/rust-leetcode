use crate::Solution;

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();
        let [mut max_left, mut max_right] = [0; 2];

        for i in 0..s.len() {
            let [new_max_left, new_max_right] = Solution::longest_palindrome_dp(&chars, i, i, max_left, max_right);

            if new_max_left - new_max_right > max_left - max_right {
                max_left = new_max_left;
                max_right = new_max_right;
            }

            let [new_max_left, new_max_right] = Solution::longest_palindrome_dp(&chars, i, i + 1, max_left, max_right);

            if new_max_left - new_max_right > max_left - max_right {
                max_left = new_max_left;
                max_right = new_max_right;
            }
        }

        chars[max_left..max_right+1].iter().collect()
    }

    fn longest_palindrome_dp(chars: &Vec<char>, mut left: usize, mut right: usize, max_left: usize, max_right: usize) -> [usize; 2] {
        while right < chars.len() && chars[left] == chars[right] {
            if left == 0 {
                break;
            }

            left -= 1;
            right += 1;
        }

        match left - right > max_left - max_right {
            true => return [left, right],
            false => return [max_left, max_right]
        }
    }
}
// @lc code=end
