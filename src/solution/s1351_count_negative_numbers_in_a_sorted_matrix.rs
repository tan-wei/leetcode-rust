/**
 * [1351] Count Negative Numbers in a Sorted Matrix
 *
 * Given a m x n matrix grid which is sorted in non-increasing order both row-wise and column-wise, return the number of negative numbers in grid.
 *  
 * Example 1:
 *
 * Input: grid = [[4,3,2,-1],[3,2,1,-1],[1,1,-1,-2],[-1,-1,-2,-3]]
 * Output: 8
 * Explanation: There are 8 negatives number in the matrix.
 *
 * Example 2:
 *
 * Input: grid = [[3,2],[1,0]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 100
 * 	-100 <= grid[i][j] <= 100
 *
 *  
 * Follow up: Could you find an O(n + m) solution?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/
// discuss: https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.into_iter()
            .map(|arr| (arr.len() - arr.partition_point(|&x| x >= 0)) as i32)
            .sum::<i32>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1351_example_1() {
        let grid = vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3],
        ];

        let result = 8;

        assert_eq!(Solution::count_negatives(grid), result);
    }

    #[test]
    fn test_1351_example_2() {
        let grid = vec![vec![3, 2], vec![1, 0]];

        let result = 0;

        assert_eq!(Solution::count_negatives(grid), result);
    }
}
