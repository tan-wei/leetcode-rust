/**
 * [2643] Row With Maximum Ones
 *
 * Given a m x n binary matrix mat, find the 0-indexed position of the row that contains the maximum count of ones, and the number of ones in that row.
 * In case there are multiple rows that have the maximum count of ones, the row with the smallest row number should be selected.
 * Return an array containing the index of the row, and the number of ones in it.
 *  
 * Example 1:
 *
 * Input: mat = [[0,1],[1,0]]
 * Output: [0,1]
 * Explanation: Both rows have the same number of 1's. So we return the index of the smaller row, 0, and the maximum count of ones (1). So, the answer is [0,1].
 *
 * Example 2:
 *
 * Input: mat = [[0,0,0],[0,1,1]]
 * Output: [1,2]
 * Explanation: The row indexed 1 has the maximum count of ones (2). So we return its index, 1, and the count. So, the answer is [1,2].
 *
 * Example 3:
 *
 * Input: mat = [[0,0],[1,1],[0,0]]
 * Output: [1,2]
 * Explanation: The row indexed 1 has the maximum count of ones (2). So the answer is [1,2].
 *
 *  
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 100
 * 	mat[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/row-with-maximum-ones/
// discuss: https://leetcode.com/problems/row-with-maximum-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        mat.into_iter()
            .enumerate()
            .map(|(i, row)| vec![i as i32, row.into_iter().sum::<i32>()])
            .max_by_key(|ar| (ar[1], -ar[0]))
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2643_example_1() {
        let mat = vec![vec![0, 1], vec![1, 0]];

        let result = vec![0, 1];

        assert_eq!(Solution::row_and_maximum_ones(mat), result);
    }

    #[test]
    fn test_2643_example_2() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 1]];

        let result = vec![1, 2];

        assert_eq!(Solution::row_and_maximum_ones(mat), result);
    }

    #[test]
    fn test_2643_example_3() {
        let mat = vec![vec![0, 0], vec![1, 1], vec![0, 0]];

        let result = vec![1, 2];

        assert_eq!(Solution::row_and_maximum_ones(mat), result);
    }
}
