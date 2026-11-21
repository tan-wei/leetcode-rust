/**
 * [2718] Sum of Matrix After Queries
 *
 * You are given an integer n and a 0-indexed 2D array queries where queries[i] = [typei, indexi, vali].
 * Initially, there is a 0-indexed n x n matrix filled with 0's. For each query, you must apply one of the following changes:
 *
 * 	if typei == 0, set the values in the row with indexi to vali, overwriting any previous values.
 * 	if typei == 1, set the values in the column with indexi to vali, overwriting any previous values.
 *
 * Return the sum of integers in the matrix after all queries are applied.
 *  
 * Example 1:
 *
 * Input: n = 3, queries = [[0,0,1],[1,2,2],[0,2,3],[1,0,4]]
 * Output: 23
 * Explanation: The image above describes the matrix after each query. The sum of the matrix after all queries are applied is 23.
 *
 * Example 2:
 *
 * Input: n = 3, queries = [[0,0,4],[0,1,2],[1,0,1],[0,2,3],[1,2,1]]
 * Output: 17
 * Explanation: The image above describes the matrix after each query. The sum of the matrix after all queries are applied is 17.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^4
 * 	1 <= queries.length <= 5 * 10^4
 * 	queries[i].length == 3
 * 	0 <= typei <= 1
 * 	0 <= indexi < n
 * 	0 <= vali <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-matrix-after-queries/
// discuss: https://leetcode.com/problems/sum-of-matrix-after-queries/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
        0i64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2718_example_1() {
        let n = 3;
        let queries = vec![vec![0, 0, 1], vec![1, 2, 2], vec![0, 2, 3], vec![1, 0, 4]];

        let result = 23;

        assert_eq!(Solution::matrix_sum_queries(n, queries), result);
    }

    #[test]
    #[ignore]
    fn test_2718_example_2() {
        let n = 3;
        let queries = vec![
            vec![0, 0, 4],
            vec![0, 1, 2],
            vec![1, 0, 1],
            vec![0, 2, 3],
            vec![1, 2, 1],
        ];

        let result = 17;

        assert_eq!(Solution::matrix_sum_queries(n, queries), result);
    }
}
