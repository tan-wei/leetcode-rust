/**
 * [363] Max Sum of Rectangle No Larger Than K
 *
 * Given an m x n matrix matrix and an integer k, return the max sum of a rectangle in the matrix such that its sum is no larger than k.
 * It is guaranteed that there will be a rectangle with a sum no larger than k.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/18/sum-grid.jpg" style="width: 255px; height: 176px;" />
 * Input: matrix = [[1,0,1],[0,-2,3]], k = 2
 * Output: 2
 * Explanation: Because the sum of the blue rectangle [[0, 1], [-2, 3]] is 2, and 2 is the max number no larger than k (k = 2).
 *
 * Example 2:
 *
 * Input: matrix = [[2,2,-1]], k = 3
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 100
 * 	-100 <= matrix[i][j] <= 100
 * 	-10^5 <= k <= 10^5
 *
 *  
 * Follow up: What if the number of rows is much larger than the number of columns?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/
// discuss: https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/discuss/1313574/Rust-translated-%22Prefix-Sum-on-1D-Array-using-Sorted-Container%22
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (row, col) = (matrix.len(), matrix[0].len());
        let mut result = std::i32::MIN;
        for i in 0..row {
            let mut rowsum = vec![0; col];
            for r in matrix.iter().skip(i) {
                rowsum.iter_mut().zip(r).for_each(|(sum, val)| *sum += val);
                let mut sum = 0;
                let mut bts = std::collections::BTreeSet::new();
                bts.insert(0);
                for val in &rowsum {
                    sum += val;
                    if let Some(x) = bts.range(sum - k..).next() {
                        result = result.max(sum - x);
                        if result == k {
                            return k;
                        }
                    };
                    bts.insert(sum);
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0363_example_1() {
        let matrix = vec![vec![1, 0, 1], vec![0, -2, 3]];
        let k = 2;
        let result = 2;

        assert_eq!(Solution::max_sum_submatrix(matrix, k), result);
    }

    #[test]
    fn test_0363_example_2() {
        let matrix = vec![vec![2, 2, -1]];
        let k = 3;
        let result = 3;

        assert_eq!(Solution::max_sum_submatrix(matrix, k), result);
    }

    #[test]
    fn test_0363_addtional_case_1() {
        let matrix = vec![vec![1, 0, 1], vec![0, -2, 3]];
        let k = 3;
        let result = 3;

        assert_eq!(Solution::max_sum_submatrix(matrix, k), result);
    }

    #[test]
    fn test_0363_addtional_case_2() {
        let matrix = vec![vec![1, 0, 1], vec![0, -2, 3]];
        let k = 4;
        let result = 4;

        assert_eq!(Solution::max_sum_submatrix(matrix, k), result);
    }

    #[test]
    fn test_0363_addtional_case_3() {
        let matrix = vec![vec![1, 0, 1], vec![0, -2, 3]];
        let k = 5;
        let result = 4;

        assert_eq!(Solution::max_sum_submatrix(matrix, k), result);
    }

    #[test]
    fn test_0363_addtional_case_4() {
        let matrix = vec![vec![1, 0, 1], vec![0, -2, 3]];
        let k = 0;
        let result = 0;

        assert_eq!(Solution::max_sum_submatrix(matrix, k), result);
    }
}
