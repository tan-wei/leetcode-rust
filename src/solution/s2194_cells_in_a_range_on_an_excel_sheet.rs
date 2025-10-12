/**
 * [2194] Cells in a Range on an Excel Sheet
 *
 * A cell (r, c) of an excel sheet is represented as a string "<col><row>" where:
 *
 * 	<col> denotes the column number c of the cell. It is represented by alphabetical letters.
 *
 * 		For example, the 1^st column is denoted by 'A', the 2^nd by 'B', the 3^rd by 'C', and so on.
 *
 *
 * 	<row> is the row number r of the cell. The r^th row is represented by the integer r.
 *
 * You are given a string s in the format "<col1><row1>:<col2><row2>", where <col1> represents the column c1, <row1> represents the row r1, <col2> represents the column c2, and <row2> represents the row r2, such that r1 <= r2 and c1 <= c2.
 * Return the list of cells (x, y) such that r1 <= x <= r2 and c1 <= y <= c2. The cells should be represented as strings in the format mentioned above and be sorted in non-decreasing order first by columns and then by rows.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/02/08/ex1drawio.png" style="width: 250px; height: 160px;" />
 * Input: s = "K1:L2"
 * Output: ["K1","K2","L1","L2"]
 * Explanation:
 * The above diagram shows the cells which should be present in the list.
 * The red arrows denote the order in which the cells should be presented.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/02/09/exam2drawio.png" style="width: 500px; height: 50px;" />
 * Input: s = "A1:F1"
 * Output: ["A1","B1","C1","D1","E1","F1"]
 * Explanation:
 * The above diagram shows the cells which should be present in the list.
 * The red arrow denotes the order in which the cells should be presented.
 *
 *  
 * Constraints:
 *
 * 	s.length == 5
 * 	'A' <= s[0] <= s[3] <= 'Z'
 * 	'1' <= s[1] <= s[4] <= '9'
 * 	s consists of uppercase English letters, digits and ':'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/cells-in-a-range-on-an-excel-sheet/
// discuss: https://leetcode.com/problems/cells-in-a-range-on-an-excel-sheet/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let s = s.as_bytes();
        (s[0]..=s[3])
            .flat_map(|col| (s[1]..=s[4]).map(move |row| format!("{}{}", col as char, row as char)))
            .collect::<Vec<_>>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2194_example_1() {
        let s = "K1:L2".to_string();

        let result = vec_string!["K1", "K2", "L1", "L2"];

        assert_eq!(Solution::cells_in_range(s), result);
    }

    #[test]
    fn test_2194_example_2() {
        let s = "A1:F1".to_string();

        let result = vec_string!["A1", "B1", "C1", "D1", "E1", "F1"];

        assert_eq!(Solution::cells_in_range(s), result);
    }
}
