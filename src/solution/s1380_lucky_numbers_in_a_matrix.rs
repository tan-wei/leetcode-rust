/**
 * [1380] Lucky Numbers in a Matrix
 *
 * Given an m x n matrix of distinct numbers, return all lucky numbers in the matrix in any order.
 * A lucky number is an element of the matrix such that it is the minimum element in its row and maximum in its column.
 *  
 * Example 1:
 *
 * Input: matrix = [[3,7,8],[9,11,13],[15,16,17]]
 * Output: [15]
 * Explanation: 15 is the only lucky number since it is the minimum in its row and the maximum in its column.
 *
 * Example 2:
 *
 * Input: matrix = [[1,10,4,2],[9,3,8,7],[15,16,17,12]]
 * Output: [12]
 * Explanation: 12 is the only lucky number since it is the minimum in its row and the maximum in its column.
 *
 * Example 3:
 *
 * Input: matrix = [[7,8],[1,2]]
 * Output: [7]
 * Explanation: 7 is the only lucky number since it is the minimum in its row and the maximum in its column.
 *
 *  
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= n, m <= 50
 * 	1 <= matrix[i][j] <= 10^5.
 * 	All elements in the matrix are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/lucky-numbers-in-a-matrix/
// discuss: https://leetcode.com/problems/lucky-numbers-in-a-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                let min_val = matrix[i].iter().min();
                if Some(&matrix[i][j]) == min_val {
                    let mut t = vec![];
                    for r in 0..matrix.len() {
                        t.push(matrix[r][j])
                    }
                    let max_val = t.iter().max();
                    if Some(&matrix[i][j]) == max_val {
                        return vec![matrix[i][j]];
                    }
                }
            }
        }

        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1380_example_1() {
        let matrix = vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]];

        let result = vec![15];

        assert_eq!(Solution::lucky_numbers(matrix), result);
    }

    #[test]
    fn test_1380_example_2() {
        let matrix = vec![vec![1, 10, 4, 2], vec![9, 3, 8, 7], vec![15, 16, 17, 12]];

        let result = vec![12];

        assert_eq!(Solution::lucky_numbers(matrix), result);
    }

    #[test]
    fn test_1380_example_3() {
        let matrix = vec![vec![7, 8], vec![1, 2]];

        let result = vec![7];

        assert_eq!(Solution::lucky_numbers(matrix), result);
    }
}
