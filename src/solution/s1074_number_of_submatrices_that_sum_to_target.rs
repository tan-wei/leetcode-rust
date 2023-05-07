/**
 * [1074] Number of Submatrices That Sum to Target
 *
 * Given a matrix and a target, return the number of non-empty submatrices that sum to <font face="monospace">target</font>.
 * A submatrix x1, y1, x2, y2 is the set of all cells matrix[x][y] with x1 <= x <= x2 and y1 <= y <= y2.
 * Two submatrices (x1, y1, x2, y2) and (x1', y1', x2', y2') are different if they have some coordinate that is different: for example, if x1 != x1'.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/02/mate1.jpg" style="width: 242px; height: 242px;" />
 * Input: matrix = [[0,1,0],[1,1,1],[0,1,0]], target = 0
 * Output: 4
 * Explanation: The four 1x1 submatrices that only contain 0.
 *
 * Example 2:
 *
 * Input: matrix = [[1,-1],[-1,1]], target = 0
 * Output: 5
 * Explanation: The two 1x2 submatrices, plus the two 2x1 submatrices, plus the 2x2 submatrix.
 *
 * Example 3:
 *
 * Input: matrix = [[904]], target = 0
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= matrix.length <= 100
 * 	1 <= matrix[0].length <= 100
 * 	-1000 <= matrix[i] <= 1000
 * 	-10^8 <= target <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/
// discuss: https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/solutions/1162957/rust-solution/
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let sums = matrix
            .iter()
            .map(|row| {
                row.iter()
                    .scan(0, |sum, &x| {
                        *sum += x;
                        Some(*sum)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let len = matrix[0].len();
        let mut result = 0;
        let mut hm = std::collections::HashMap::new();
        for i in 0..len {
            for j in i..len {
                hm.clear();
                hm.insert(0, 1);
                let mut sum = 0;
                for row in &sums {
                    sum += row[j] - if i > 0 { row[i - 1] } else { 0 };
                    if let Some(&count) = hm.get(&(sum - target)) {
                        result += count;
                    }
                    *hm.entry(sum).or_default() += 1;
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
    fn test_1074_example_1() {
        let matrix = vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]];
        let target = 0;
        let result = 4;

        assert_eq!(Solution::num_submatrix_sum_target(matrix, target), result);
    }

    #[test]
    fn test_1074_example_2() {
        let matrix = vec![vec![1, -1], vec![-1, 1]];
        let target = 0;
        let result = 5;

        assert_eq!(Solution::num_submatrix_sum_target(matrix, target), result);
    }

    #[test]
    fn test_1074_example_3() {
        let matrix = vec![vec![904]];
        let target = 0;
        let result = 0;

        assert_eq!(Solution::num_submatrix_sum_target(matrix, target), result);
    }
}
