/**
 * [59] Spiral Matrix II
 *
 * Given a positive integer n, generate an n x n matrix filled with elements from 1 to n^2 in spiral order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiraln.jpg" style="width: 242px; height: 242px;" />
 * Input: n = 3
 * Output: [[1,2,3],[8,9,4],[7,6,5]]
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: [[1]]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 20
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/spiral-matrix-ii/
// discuss: https://leetcode.com/problems/spiral-matrix-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix: Vec<Vec<i32>> = Vec::new();

        for i in 0..n {
            matrix.push(vec![0; n as usize]);
        }

        let mut counter: i32 = 1;
        let mut offset: usize = 0;

        while counter <= n.pow(2) {
            if counter == n.pow(2) {
                matrix[offset][offset] = counter;
                break;
            }

            for i in offset..n as usize - 1 - offset {
                matrix[offset][i] = counter;
                counter += 1;
            }

            for i in offset..n as usize - 1 - offset {
                matrix[i][n as usize - 1 - offset] = counter;
                counter += 1;
            }
            for i in (offset + 1..n as usize - offset).rev() {
                matrix[n as usize - 1 - offset][i] = counter;
                counter += 1;
            }

            for i in (offset + 1..n as usize - offset).rev() {
                matrix[i][offset] = counter;
                counter += 1;
            }

            offset += 1;
        }

        matrix
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0059_example_1() {
        let n = 3;
        let result = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];

        assert_eq!(Solution::generate_matrix(n), result);
    }

    #[test]
    fn test_0059_example_2() {
        let n = 1;
        let result = vec![vec![1]];

        assert_eq!(Solution::generate_matrix(n), result);
    }
}
