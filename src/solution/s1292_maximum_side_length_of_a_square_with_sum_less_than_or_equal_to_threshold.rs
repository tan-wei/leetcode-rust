/**
 * [1292] Maximum Side Length of a Square with Sum Less than or Equal to Threshold
 *
 * Given a m x n matrix mat and an integer threshold, return the maximum side-length of a square with a sum less than or equal to threshold or return 0 if there is no such square.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/12/05/e1.png" style="width: 335px; height: 186px;" />
 * Input: mat = [[1,1,3,2,4,3,2],[1,1,3,2,4,3,2],[1,1,3,2,4,3,2]], threshold = 4
 * Output: 2
 * Explanation: The maximum side length of square with sum less than 4 is 2 as shown.
 *
 * Example 2:
 *
 * Input: mat = [[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2]], threshold = 1
 * Output: 0
 *
 *
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 300
 * 	0 <= mat[i][j] <= 10^4
 * 	0 <= threshold <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/
// discuss: https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let (n, m) = (mat.len(), mat[0].len());
        let mut dp = vec![vec![0; m + 1]; n + 1];
        let mut result = 0;

        for (i, row) in mat.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                dp[i + 1][j + 1] = dp[i + 1][j] + dp[i][j + 1] - dp[i][j] + val;

                while i >= result && j >= result {
                    let square_sum =
                        dp[i + 1][j + 1] - dp[i - result][j + 1] - dp[i + 1][j - result]
                            + dp[i - result][j - result];

                    if square_sum <= threshold {
                        result += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1292_example_1() {
        let mat = vec![
            vec![1, 1, 3, 2, 4, 3, 2],
            vec![1, 1, 3, 2, 4, 3, 2],
            vec![1, 1, 3, 2, 4, 3, 2],
        ];
        let threshold = 4;
        let result = 2;

        assert_eq!(Solution::max_side_length(mat, threshold), result);
    }

    #[test]
    fn test_1292_example_2() {
        let mat = vec![
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2],
        ];
        let threshold = 1;
        let result = 0;

        assert_eq!(Solution::max_side_length(mat, threshold), result);
    }
}
