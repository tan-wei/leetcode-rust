/**
 * [1970] Last Day Where You Can Still Cross
 *
 * There is a 1-based binary matrix where 0 represents land and 1 represents water. You are given integers row and col representing the number of rows and columns in the matrix, respectively.
 * Initially on day 0, the entire matrix is land. However, each day a new cell becomes flooded with water. You are given a 1-based 2D array cells, where cells[i] = [ri, ci] represents that on the i^th day, the cell on the ri^th row and ci^th column (1-based coordinates) will be covered with water (i.e., changed to 1).
 * You want to find the last day that it is possible to walk from the top to the bottom by only walking on land cells. You can start from any cell in the top row and end at any cell in the bottom row. You can only travel in the four cardinal directions (left, right, up, and down).
 * Return the last day where it is possible to walk from the top to the bottom by only walking on land cells.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/27/1.png" style="width: 624px; height: 162px;" />
 * Input: row = 2, col = 2, cells = [[1,1],[2,1],[1,2],[2,2]]
 * Output: 2
 * Explanation: The above image depicts how the matrix changes each day starting from day 0.
 * The last day where it is possible to cross from top to bottom is on day 2.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/27/2.png" style="width: 504px; height: 178px;" />
 * Input: row = 2, col = 2, cells = [[1,1],[1,2],[2,1],[2,2]]
 * Output: 1
 * Explanation: The above image depicts how the matrix changes each day starting from day 0.
 * The last day where it is possible to cross from top to bottom is on day 1.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/27/3.png" style="width: 666px; height: 167px;" />
 * Input: row = 3, col = 3, cells = [[1,2],[2,1],[3,3],[2,2],[1,1],[1,3],[2,3],[3,2],[3,1]]
 * Output: 3
 * Explanation: The above image depicts how the matrix changes each day starting from day 0.
 * The last day where it is possible to cross from top to bottom is on day 3.
 *
 *  
 * Constraints:
 *
 * 	2 <= row, col <= 2 * 10^4
 * 	4 <= row * col <= 2 * 10^4
 * 	cells.length == row * col
 * 	1 <= ri <= row
 * 	1 <= ci <= col
 * 	All the values of cells are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/last-day-where-you-can-still-cross/
// discuss: https://leetcode.com/problems/last-day-where-you-can-still-cross/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1970_example_1() {
        let row = 3;
        let col = 2;
        let cells = vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2]];

        let result = 2;

        assert_eq!(Solution::latest_day_to_cross(row, col, cells), result);
    }

    #[test]
    #[ignore]
    fn test_1970_example_2() {
        let row = 2;
        let col = 2;
        let cells = vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]];

        let result = 1;

        assert_eq!(Solution::latest_day_to_cross(row, col, cells), result);
    }

    #[test]
    #[ignore]
    fn test_1970_example_3() {
        let row = 3;
        let col = 3;
        let cells = vec![
            vec![1, 2],
            vec![2, 1],
            vec![3, 3],
            vec![2, 2],
            vec![1, 1],
            vec![1, 3],
            vec![2, 3],
            vec![3, 2],
            vec![3, 1],
        ];

        let result = 3;

        assert_eq!(Solution::latest_day_to_cross(row, col, cells), result);
    }
}
