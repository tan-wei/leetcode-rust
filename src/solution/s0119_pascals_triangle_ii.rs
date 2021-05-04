/**
 * [119] Pascal's Triangle II
 *
 * Given an integer rowIndex, return the rowIndex^th (0-indexed) row of the Pascal's triangle.
 * In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" style="height:240px; width:260px" />
 *  
 * Example 1:
 * Input: rowIndex = 3
 * Output: [1,3,3,1]
 * Example 2:
 * Input: rowIndex = 0
 * Output: [1]
 * Example 3:
 * Input: rowIndex = 1
 * Output: [1,1]
 *  
 * Constraints:
 *
 * 	0 <= rowIndex <= 33
 *
 *  
 * Follow up: Could you optimize your algorithm to use only O(rowIndex) extra space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pascals-triangle-ii/
// discuss: https://leetcode.com/problems/pascals-triangle-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result = vec![1];
        let mut new_row;
        for _ in 0..row_index as usize {
            new_row = vec![1];
            let acc = result
                .iter()
                .zip(result[1..].iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<i32>>();
            new_row.extend_from_slice(&acc);
            new_row.push(1);
            result = new_row;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0119_example_1() {
        let row_index = 3;
        let result = vec![1, 3, 3, 1];

        assert_eq!(Solution::get_row(row_index), result);
    }

    #[test]
    fn test_0119_example_2() {
        let row_index = 0;
        let result = vec![1];

        assert_eq!(Solution::get_row(row_index), result);
    }

    #[test]
    fn test_0119_example_3() {
        let row_index = 1;
        let result = vec![1, 1];

        assert_eq!(Solution::get_row(row_index), result);
    }
}
