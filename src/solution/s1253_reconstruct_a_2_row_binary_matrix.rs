/**
 * [1253] Reconstruct a 2-Row Binary Matrix
 *
 * Given the following details of a matrix with n columns and 2 rows :
 *
 * 	The matrix is a binary matrix, which means each element in the matrix can be 0 or 1.
 * 	The sum of elements of the 0-th(upper) row is given as upper.
 * 	The sum of elements of the 1-st(lower) row is given as lower.
 * 	The sum of elements in the i-th column(0-indexed) is colsum[i], where colsum is given as an integer array with length n.
 *
 * Your task is to reconstruct the matrix with upper, lower and colsum.
 * Return it as a 2-D integer array.
 * If there are more than one valid solution, any of them will be accepted.
 * If no valid solution exists, return an empty 2-D array.
 *  
 * Example 1:
 *
 * Input: upper = 2, lower = 1, colsum = [1,1,1]
 * Output: [[1,1,0],[0,0,1]]
 * Explanation: [[1,0,1],[0,1,0]], and [[0,1,1],[1,0,0]] are also correct answers.
 *
 * Example 2:
 *
 * Input: upper = 2, lower = 3, colsum = [2,2,1,1]
 * Output: []
 *
 * Example 3:
 *
 * Input: upper = 5, lower = 5, colsum = [2,1,2,0,1,0,1,2,0,1]
 * Output: [[1,1,1,0,1,0,0,1,0,0],[1,0,1,0,0,0,1,1,0,1]]
 *
 *  
 * Constraints:
 *
 * 	1 <= colsum.length <= 10^5
 * 	0 <= upper, lower <= colsum.length
 * 	0 <= colsum[i] <= 2
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reconstruct-a-2-row-binary-matrix/
// discuss: https://leetcode.com/problems/reconstruct-a-2-row-binary-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut upper = upper;
        let mut lower = lower;
        let mut colsum = colsum;
        let n = colsum.len();
        let mut ups = vec![0; n];
        let mut downs = vec![0; n];

        for i in 0..n {
            if colsum[i] == 2 {
                upper -= 1;
                lower -= 1;
                colsum[i] = 0;
                ups[i] = 1;
                downs[i] = 1;
            }
            if upper < 0 || lower < 0 {
                return vec![];
            }
        }

        for i in 0..n {
            if colsum[i] == 1 {
                if 0 < upper {
                    upper -= 1;
                    ups[i] = 1;
                } else if 0 < lower {
                    lower -= 1;
                    downs[i] = 1;
                } else {
                    return vec![];
                }
            }
        }

        if 0 < lower || upper < 0 {
            vec![]
        } else {
            vec![ups, downs]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1253_example_1() {
        let upper = 2;
        let lower = 1;
        let colsum = vec![1, 1, 1];
        let result = vec![vec![1, 1, 0], vec![0, 0, 1]];

        assert_eq!(Solution::reconstruct_matrix(upper, lower, colsum), result);
    }

    #[test]
    fn test_1253_example_2() {
        let upper = 2;
        let lower = 3;
        let colsum = vec![2, 2, 1, 1];
        let result: Vec<Vec<i32>> = vec![];

        assert_eq!(Solution::reconstruct_matrix(upper, lower, colsum), result);
    }

    #[test]
    fn test_1253_example_3() {
        let upper = 5;
        let lower = 5;
        let colsum = vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1];
        let result = vec![
            vec![1, 1, 1, 0, 1, 0, 0, 1, 0, 0],
            vec![1, 0, 1, 0, 0, 0, 1, 1, 0, 1],
        ];

        assert_eq!(Solution::reconstruct_matrix(upper, lower, colsum), result);
    }
}
