/**
 * [1659] Maximize Grid Happiness
 *
 * You are given four integers, m, n, introvertsCount, and extrovertsCount. You have an m x n grid, and there are two types of people: introverts and extroverts. There are introvertsCount introverts and extrovertsCount extroverts.
 * You should decide how many people you want to live in the grid and assign each of them one grid cell. Note that you do not have to have all the people living in the grid.
 * The happiness of each person is calculated as follows:
 *
 * 	Introverts start with 120 happiness and lose 30 happiness for each neighbor (introvert or extrovert).
 * 	Extroverts start with 40 happiness and gain 20 happiness for each neighbor (introvert or extrovert).
 *
 * Neighbors live in the directly adjacent cells north, east, south, and west of a person's cell.
 * The grid happiness is the sum of each person's happiness. Return the maximum possible grid happiness.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/grid_happiness.png" style="width: 261px; height: 121px;" />
 * Input: m = 2, n = 3, introvertsCount = 1, extrovertsCount = 2
 * Output: 240
 * Explanation: Assume the grid is 1-indexed with coordinates (row, column).
 * We can put the introvert in cell (1,1) and put the extroverts in cells (1,3) and (2,3).
 * - Introvert at (1,1) happiness: 120 (starting happiness) - (0 * 30) (0 neighbors) = 120
 * - Extrovert at (1,3) happiness: 40 (starting happiness) + (1 * 20) (1 neighbor) = 60
 * - Extrovert at (2,3) happiness: 40 (starting happiness) + (1 * 20) (1 neighbor) = 60
 * The grid happiness is 120 + 60 + 60 = 240.
 * The above figure shows the grid in this example with each person's happiness. The introvert stays in the light green cell while the extroverts live on the light purple cells.
 *
 * Example 2:
 *
 * Input: m = 3, n = 1, introvertsCount = 2, extrovertsCount = 1
 * Output: 260
 * Explanation: Place the two introverts in (1,1) and (3,1) and the extrovert at (2,1).
 * - Introvert at (1,1) happiness: 120 (starting happiness) - (1 * 30) (1 neighbor) = 90
 * - Extrovert at (2,1) happiness: 40 (starting happiness) + (2 * 20) (2 neighbors) = 80
 * - Introvert at (3,1) happiness: 120 (starting happiness) - (1 * 30) (1 neighbor) = 90
 * The grid happiness is 90 + 80 + 90 = 260.
 *
 * Example 3:
 *
 * Input: m = 2, n = 2, introvertsCount = 4, extrovertsCount = 0
 * Output: 240
 *
 *  
 * Constraints:
 *
 * 	1 <= m, n <= 5
 * 	0 <= introvertsCount, extrovertsCount <= min(m * n, 6)
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-grid-happiness/
// discuss: https://leetcode.com/problems/maximize-grid-happiness/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximize-grid-happiness/solutions/5164887/rust-memoization/
    pub fn get_max_grid_happiness(
        m: i32,
        n: i32,
        introverts_count: i32,
        extroverts_count: i32,
    ) -> i32 {
        let mut dp = std::collections::HashMap::<[i32; 5], i32>::new();
        let state = [0, 0, 0, introverts_count, extroverts_count];
        Self::dfs_helper(state, m, n, &mut dp)
    }

    fn dfs_helper(
        state: [i32; 5],
        m: i32,
        n: i32,
        dp: &mut std::collections::HashMap<[i32; 5], i32>,
    ) -> i32 {
        if state[0] == m {
            return 0;
        }

        if let Some(&result) = dp.get(&state) {
            return result;
        }

        let i = state[0];
        let j = state[1];
        let prev = state[2];
        let intro = state[3];
        let extro = state[4];

        let above = (prev >> (j * 2)) & 3;
        let left = if j == 0 { 0 } else { (prev >> (j * 2 - 2)) & 3 };
        let prev2 = prev - (above << (j * 2));

        let i2 = i + if j == n - 1 { 1 } else { 0 };
        let j2 = if j == n - 1 { 0 } else { j + 1 };
        let state2 = [i2, j2, prev2, intro, extro];
        let mut result = Self::dfs_helper(state2, m, n, dp);

        let count = (above + 1) / 2 + (left + 1) / 2;
        let modify = [0, -30, 20];
        let extra = modify[above as usize] + modify[left as usize];

        if intro > 0 {
            let mut ans2 = 120 + modify[1] * count + extra;
            let state2 = [i2, j2, prev2 + (1 << (j * 2)), intro - 1, extro];
            ans2 += Self::dfs_helper(state2, m, n, dp);
            result = result.max(ans2);
        }
        if extro > 0 {
            let mut ans2 = 40 + modify[2] * count + extra;
            let state2 = [i2, j2, prev2 + (2 << (j * 2)), intro, extro - 1];
            ans2 += Self::dfs_helper(state2, m, n, dp);
            result = result.max(ans2);
        }
        dp.insert(state, result);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1659_example_1() {
        let m = 2;
        let n = 3;
        let introverts_count = 1;
        let extroverts_count = 2;

        let result = 240;

        assert_eq!(
            Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count),
            result
        );
    }

    #[test]
    fn test_1659_example_2() {
        let m = 3;
        let n = 1;
        let introverts_count = 2;
        let extroverts_count = 1;

        let result = 260;

        assert_eq!(
            Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count),
            result
        );
    }

    #[test]
    fn test_1659_example_3() {
        let m = 2;
        let n = 2;
        let introverts_count = 4;
        let extroverts_count = 0;

        let result = 240;

        assert_eq!(
            Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count),
            result
        );
    }
}
