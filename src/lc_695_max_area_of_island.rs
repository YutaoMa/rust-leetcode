use crate::Solution;

// @lc code=start
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut max_area = 0;
        
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    max_area = max_area.max(Solution::max_area_of_island_dfs(&mut grid, i, j));
                }
            }
        }
        max_area
    }

    fn max_area_of_island_dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        if grid[i][j] != 1 {
            return 0;
        }

        grid[i][j] = 0;

        let mut max_area = 1;

        max_area += if i < grid.len() - 1 { Solution::max_area_of_island_dfs(grid, i + 1, j) } else { 0 };
        max_area += if i > 0 { Solution::max_area_of_island_dfs(grid, i - 1, j) } else { 0 };
        max_area += if j < grid[0].len() - 1 { Solution::max_area_of_island_dfs(grid, i, j + 1) } else { 0 };
        max_area += if j > 0 { Solution::max_area_of_island_dfs(grid, i, j - 1) } else { 0 };

        max_area
    }
}
// @lc code=end
