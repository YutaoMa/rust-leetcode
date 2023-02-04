use crate::Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = Vec::new();

        fn generate_step(str: String, l: i32, r: i32, n: i32, ans: &mut Vec<String>) {
            if l > n || r > n || r > l {
                return;
            }

            if str.len() == (n * 2) as usize && l == r {
                ans.push(str);
                return;
            }

            generate_step(str.clone() + "(", l + 1, r, n, ans);
            generate_step(str.clone() + ")", l, r + 1, n, ans);
        }

        generate_step("".to_string(), 0, 0, n, &mut ans);

        ans
    }
}
