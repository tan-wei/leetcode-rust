/**
 * [62] Unique Paths
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 * How many possible unique paths are there?
 *  
 * Example 1:
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png" style="width: 400px; height: 183px;" />
 * Input: m = 3, n = 7
 * Output: 28
 *
 * Example 2:
 *
 * Input: m = 3, n = 2
 * Output: 3
 * Explanation:
 * From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
 * 1. Right -> Down -> Down
 * 2. Down -> Down -> Right
 * 3. Down -> Right -> Down
 *
 * Example 3:
 *
 * Input: m = 7, n = 3
 * Output: 28
 *
 * Example 4:
 *
 * Input: m = 3, n = 3
 * Output: 6
 *
 *  
 * Constraints:
 *
 * 	1 <= m, n <= 100
 * 	It's guaranteed that the answer will be less than or equal to 2 * 10^9.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-paths/
// discuss: https://leetcode.com/problems/unique-paths/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m <= 0 || n <= 0 {
            return 0;
        }

        let mut result = 1u64;

        for i in n..m + n - 1 {
            result = result * i as u64 / (i - n + 1) as u64;
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0062_example_1() {
        let m = 3;
        let n = 7;
        let result = 28;

        assert_eq!(Solution::unique_paths(m, n), result);
    }

    #[test]
    fn test_0062_example_2() {
        let m = 3;
        let n = 2;
        let result = 3;

        assert_eq!(Solution::unique_paths(m, n), result);
    }

    #[test]
    fn test_0062_example_3() {
        let m = 7;
        let n = 3;
        let result = 28;

        assert_eq!(Solution::unique_paths(m, n), result);
    }

    #[test]
    fn test_0062_example_4() {
        let m = 3;
        let n = 3;
        let result = 6;

        assert_eq!(Solution::unique_paths(m, n), result);
    }
}
