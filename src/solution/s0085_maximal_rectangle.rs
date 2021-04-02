/**
 * [85] Maximal Rectangle
 *
 * Given a rows x cols binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/maximal.jpg" style="width: 402px; height: 322px;" />
 * Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
 * Output: 6
 * Explanation: The maximal rectangle is shown in the above picture.
 *
 * Example 2:
 *
 * Input: matrix = []
 * Output: 0
 *
 * Example 3:
 *
 * Input: matrix = [["0"]]
 * Output: 0
 *
 * Example 4:
 *
 * Input: matrix = [["1"]]
 * Output: 1
 *
 * Example 5:
 *
 * Input: matrix = [["0","0"]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	rows == matrix.length
 * 	cols == matrix[i].length
 * 	0 <= row, cols <= 200
 * 	matrix[i][j] is '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximal-rectangle/
// discuss: https://leetcode.com/problems/maximal-rectangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn merge(row1: &mut Vec<char>, row2: &Vec<char>, n: usize) -> i32 {
        let mut current = 0;
        let mut longest = 0;

        for i in 0..n {
            if row1[i] == '1' && row2[i] == '1' {
                current += 1;
                longest = longest.max(current);
            } else {
                row1[i] = '0';
                current = 0;
            }
        }

        longest
    }

    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        match matrix.len() {
            0 => 0,
            rows @ _ => {
                let cols = matrix[0].len();
                let mut largest_area = 0;
                let mut working_matrix = vec![vec!['1'; cols]; rows];
                for height in 0..rows {
                    for j in 0..(rows - height) {
                        let base_area =
                            Solution::merge(&mut working_matrix[j], &matrix[j + height], cols);
                        largest_area = largest_area.max(base_area * ((height as i32) + 1));
                    }
                }
                largest_area
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0085_example_1() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        let result = 6;

        assert_eq!(Solution::maximal_rectangle(matrix), result);
    }

    #[test]
    fn test_0085_example_2() {
        let matrix = vec![];
        let result = 0;

        assert_eq!(Solution::maximal_rectangle(matrix), result);
    }

    #[test]
    fn test_0085_example_3() {
        let matrix = vec![vec!['0']];
        let result = 0;

        assert_eq!(Solution::maximal_rectangle(matrix), result);
    }

    #[test]
    fn test_0085_example_4() {
        let matrix = vec![vec!['1']];
        let result = 1;

        assert_eq!(Solution::maximal_rectangle(matrix), result);
    }

    #[test]
    fn test_0085_example_5() {
        let matrix = vec![vec!['0', '0']];
        let result = 0;

        assert_eq!(Solution::maximal_rectangle(matrix), result);
    }
}
