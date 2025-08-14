/**
 * [2120] Execution of All Suffix Instructions Staying in a Grid
 *
 * There is an n x n grid, with the top-left cell at (0, 0) and the bottom-right cell at (n - 1, n - 1). You are given the integer n and an integer array startPos where startPos = [startrow, startcol] indicates that a robot is initially at cell (startrow, startcol).
 * You are also given a 0-indexed string s of length m where s[i] is the i^th instruction for the robot: 'L' (move left), 'R' (move right), 'U' (move up), and 'D' (move down).
 * The robot can begin executing from any i^th instruction in s. It executes the instructions one by one towards the end of s but it stops if either of these conditions is met:
 *
 * 	The next instruction will move the robot off the grid.
 * 	There are no more instructions left to execute.
 *
 * Return an array answer of length m where answer[i] is the number of instructions the robot can execute if the robot begins executing from the i^th instruction in s.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/09/1.png" style="width: 145px; height: 142px;" />
 * Input: n = 3, startPos = [0,1], s = "RRDDLU"
 * Output: [1,5,4,3,1,0]
 * Explanation: Starting from startPos and beginning execution from the i^th instruction:
 * - 0^th: "<u>R</u>RDDLU". Only one instruction "R" can be executed before it moves off the grid.
 * - 1^st:  "<u>RDDLU</u>". All five instructions can be executed while it stays in the grid and ends at (1, 1).
 * - 2^nd:   "<u>DDLU</u>". All four instructions can be executed while it stays in the grid and ends at (1, 0).
 * - 3^rd:    "<u>DLU</u>". All three instructions can be executed while it stays in the grid and ends at (0, 0).
 * - 4^th:     "<u>L</u>U". Only one instruction "L" can be executed before it moves off the grid.
 * - 5^th:      "U". If moving up, it would move off the grid.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/09/2.png" style="width: 106px; height: 103px;" />
 * Input: n = 2, startPos = [1,1], s = "LURD"
 * Output: [4,1,0,0]
 * Explanation:
 * - 0^th: "<u>LURD</u>".
 * - 1^st:  "<u>U</u>RD".
 * - 2^nd:   "RD".
 * - 3^rd:    "D".
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/09/3.png" style="width: 67px; height: 64px;" />
 * Input: n = 1, startPos = [0,0], s = "LRUD"
 * Output: [0,0,0,0]
 * Explanation: No matter which instruction the robot begins execution from, it would move off the grid.
 *
 *  
 * Constraints:
 *
 * 	m == s.length
 * 	1 <= n, m <= 500
 * 	startPos.length == 2
 * 	0 <= startrow, startcol < n
 * 	s consists of 'L', 'R', 'U', and 'D'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/execution-of-all-suffix-instructions-staying-in-a-grid/
// discuss: https://leetcode.com/problems/execution-of-all-suffix-instructions-staying-in-a-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2120_example_1() {
        let n = 3;
        let start_pos = vec![0, 1];
        let s = "RRDDLU".to_string();

        let result = vec![1, 5, 4, 3, 1, 0];

        assert_eq!(Solution::execute_instructions(n, start_pos, s), result);
    }

    #[test]
    #[ignore]
    fn test_2120_example_2() {
        let n = 2;
        let start_pos = vec![1, 1];
        let s = "LURD".to_string();

        let result = vec![4, 1, 0, 0];

        assert_eq!(Solution::execute_instructions(n, start_pos, s), result);
    }

    #[test]
    #[ignore]
    fn test_2120_example_3() {
        let n = 1;
        let start_pos = vec![0, 0];
        let s = "LRUD".to_string();

        let result = vec![0, 0, 0, 0];

        assert_eq!(Solution::execute_instructions(n, start_pos, s), result);
    }
}
