/**
 * [1981] Minimize the Difference Between Target and Chosen Elements
 *
 * You are given an m x n integer matrix mat and an integer target.
 * Choose one integer from each row in the matrix such that the absolute difference between target and the sum of the chosen elements is minimized.
 * Return the minimum absolute difference.
 * The absolute difference between two numbers a and b is the absolute value of a - b.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/03/matrix1.png" style="width: 181px; height: 181px;" />
 * Input: mat = [[1,2,3],[4,5,6],[7,8,9]], target = 13
 * Output: 0
 * Explanation: One possible choice is to:
 * - Choose 1 from the first row.
 * - Choose 5 from the second row.
 * - Choose 7 from the third row.
 * The sum of the chosen elements is 13, which equals the target, so the absolute difference is 0.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/03/matrix1-1.png" style="width: 61px; height: 181px;" />
 * Input: mat = [[1],[2],[3]], target = 100
 * Output: 94
 * Explanation: The best possible choice is to:
 * - Choose 1 from the first row.
 * - Choose 2 from the second row.
 * - Choose 3 from the third row.
 * The sum of the chosen elements is 6, and the absolute difference is 94.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/03/matrix1-3.png" style="width: 301px; height: 61px;" />
 * Input: mat = [[1,2,9,8,7]], target = 6
 * Output: 1
 * Explanation: The best choice is to choose 7 from the first row.
 * The absolute difference is 1.
 *
 *  
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 70
 * 	1 <= mat[i][j] <= 70
 * 	1 <= target <= 800
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimize-the-difference-between-target-and-chosen-elements/
// discuss: https://leetcode.com/problems/minimize-the-difference-between-target-and-chosen-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1981_example_1() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let target = 13;

        let result = 0;

        assert_eq!(Solution::minimize_the_difference(mat, target), result);
    }

    #[test]
    #[ignore]
    fn test_1981_example_2() {
        let mat = vec![vec![1], vec![2], vec![3]];
        let target = 100;

        let result = 94;

        assert_eq!(Solution::minimize_the_difference(mat, target), result);
    }

    #[test]
    #[ignore]
    fn test_1981_example_3() {
        let mat = vec![vec![1, 2, 9, 8, 7]];
        let target = 6;

        let result = 1;

        assert_eq!(Solution::minimize_the_difference(mat, target), result);
    }
}
