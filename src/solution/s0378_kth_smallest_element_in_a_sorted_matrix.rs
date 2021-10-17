/**
 * [378] Kth Smallest Element in a Sorted Matrix
 *
 * Given an n x n matrix where each of the rows and columns are sorted in ascending order, return the k^th smallest element in the matrix.
 * Note that it is the k^th smallest element in the sorted order, not the k^th distinct element.
 *  
 * Example 1:
 *
 * Input: matrix = [[1,5,9],[10,11,13],[12,13,15]], k = 8
 * Output: 13
 * Explanation: The elements in the matrix are [1,5,9,10,11,12,13,<u>13</u>,15], and the 8^th smallest number is 13
 *
 * Example 2:
 *
 * Input: matrix = [[-5]], k = 1
 * Output: -5
 *
 *  
 * Constraints:
 *
 * 	n == matrix.length
 * 	n == matrix[i].length
 * 	1 <= n <= 300
 * 	-10^9 <= matrix[i][j] <= 10^9
 * 	All the rows and columns of matrix are guaranteed to be sorted in non-decreasing order.
 * 	1 <= k <= n^2
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/
// discuss: https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/discuss/1322870/Rust-translated-binary-search-solution
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let (mut lo, mut hi) = (matrix[0][0], matrix[n - 1][n - 1]);
        while lo < hi {
            let mid = (lo + hi) / 2;
            let mut count = 0;
            let (mut i, mut j) = (n - 1, 0);
            while j < n {
                if matrix[i][j] > mid {
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                } else {
                    count += i as i32 + 1;
                    j += 1;
                }
            }
            if count < k {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0378_example_1() {
        let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
        let k = 8;
        let result = 13;

        assert_eq!(Solution::kth_smallest(matrix, k), result);
    }

    #[test]
    fn test_0378_example_2() {
        let matrix = vec![vec![-5]];
        let k = 1;
        let result = -5;

        assert_eq!(Solution::kth_smallest(matrix, k), result);
    }
}
