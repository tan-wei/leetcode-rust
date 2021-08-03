/**
 * [240] Search a 2D Matrix II
 *
 * Write an efficient algorithm that searches for a target value in an m x n integer matrix. The matrix has the following properties:
 *
 * 	Integers in each row are sorted in ascending from left to right.
 * 	Integers in each column are sorted in ascending from top to bottom.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/24/searchgrid2.jpg" style="width: 300px; height: 300px;" />
 * Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 5
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/24/searchgrid.jpg" style="width: 300px; height: 300px;" />
 * Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 20
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= n, m <= 300
 * 	-10^9 <= matix[i][j] <= 10^9
 * 	All the integers in each row are sorted in ascending order.
 * 	All the integers in each column are sorted in ascending order.
 * 	-10^9 <= target <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/search-a-2d-matrix-ii/
// discuss: https://leetcode.com/problems/search-a-2d-matrix-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (mut low, mut high) = (0, matrix[0].len() - 1);

        while low < matrix.len() {
            match matrix[low][high].cmp(&target) {
                std::cmp::Ordering::Less => low += 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater if high > 0 => high -= 1,
                std::cmp::Ordering::Greater => break,
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0240_example_1() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        let target = 5;
        let result = true;

        assert_eq!(Solution::search_matrix(matrix, target), result);
    }

    #[test]
    fn test_0240_example_2() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        let target = 20;
        let result = false;

        assert_eq!(Solution::search_matrix(matrix, target), result);
    }
}
