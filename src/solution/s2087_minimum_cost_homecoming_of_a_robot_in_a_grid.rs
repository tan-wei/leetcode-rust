/**
 * [2087] Minimum Cost Homecoming of a Robot in a Grid
 *
 * There is an m x n grid, where (0, 0) is the top-left cell and (m - 1, n - 1) is the bottom-right cell. You are given an integer array startPos where startPos = [startrow, startcol] indicates that initially, a robot is at the cell (startrow, startcol). You are also given an integer array homePos where homePos = [homerow, homecol] indicates that its home is at the cell (homerow, homecol).
 * The robot needs to go to its home. It can move one cell in four directions: left, right, up, or down, and it can not move outside the boundary. Every move incurs some cost. You are further given two 0-indexed integer arrays: rowCosts of length m and colCosts of length n.
 *
 * 	If the robot moves up or down into a cell whose row is r, then this move costs rowCosts[r].
 * 	If the robot moves left or right into a cell whose column is c, then this move costs colCosts[c].
 *
 * Return the minimum total cost for this robot to return home.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/10/11/eg-1.png" style="width: 282px; height: 217px;" />
 * Input: startPos = [1, 0], homePos = [2, 3], rowCosts = [5, 4, 3], colCosts = [8, 2, 6, 7]
 * Output: 18
 * Explanation: One optimal path is that:
 * Starting from (1, 0)
 * -> It goes down to (<u>2</u>, 0). This move costs rowCosts[2] = 3.
 * -> It goes right to (2, <u>1</u>). This move costs colCosts[1] = 2.
 * -> It goes right to (2, <u>2</u>). This move costs colCosts[2] = 6.
 * -> It goes right to (2, <u>3</u>). This move costs colCosts[3] = 7.
 * The total cost is 3 + 2 + 6 + 7 = 18
 * Example 2:
 *
 * Input: startPos = [0, 0], homePos = [0, 0], rowCosts = [5], colCosts = [26]
 * Output: 0
 * Explanation: The robot is already at its home. Since no moves occur, the total cost is 0.
 *
 *  
 * Constraints:
 *
 * 	m == rowCosts.length
 * 	n == colCosts.length
 * 	1 <= m, n <= 10^5
 * 	0 <= rowCosts[r], colCosts[c] <= 10^4
 * 	startPos.length == 2
 * 	homePos.length == 2
 * 	0 <= startrow, homerow < m
 * 	0 <= startcol, homecol < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-homecoming-of-a-robot-in-a-grid/
// discuss: https://leetcode.com/problems/minimum-cost-homecoming-of-a-robot-in-a-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_cost(
        start_pos: Vec<i32>,
        home_pos: Vec<i32>,
        row_costs: Vec<i32>,
        col_costs: Vec<i32>,
    ) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2087_example_1() {
        let start_pos = vec![0, 0];
        let home_pos = vec![2, 3];
        let row_costs = vec![5, 4, 3];
        let col_costs = vec![8, 2, 6, 7];

        let result = 2;

        assert_eq!(
            Solution::min_cost(start_pos, home_pos, row_costs, col_costs),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2087_example_2() {
        let start_pos = vec![1, 0];
        let home_pos = vec![0, 0];
        let row_costs = vec![5];
        let col_costs = vec![26];

        let result = 18;

        assert_eq!(
            Solution::min_cost(start_pos, home_pos, row_costs, col_costs),
            result
        );
    }
}
