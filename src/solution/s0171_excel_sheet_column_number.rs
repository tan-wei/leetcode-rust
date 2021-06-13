/**
 * [171] Excel Sheet Column Number
 *
 * Given a string columnTitle that represents the column title as appear in an Excel sheet, return its corresponding column number.
 * For example:
 *
 * A -> 1
 * B -> 2
 * C -> 3
 * ...
 * Z -> 26
 * AA -> 27
 * AB -> 28
 * ...
 *
 *  
 * Example 1:
 *
 * Input: columnTitle = "A"
 * Output: 1
 *
 * Example 2:
 *
 * Input: columnTitle = "AB"
 * Output: 28
 *
 * Example 3:
 *
 * Input: columnTitle = "ZY"
 * Output: 701
 *
 * Example 4:
 *
 * Input: columnTitle = "FXSHRXW"
 * Output: 2147483647
 *
 *  
 * Constraints:
 *
 * 	1 <= columnTitle.length <= 7
 * 	columnTitle consists only of uppercase English letters.
 * 	columnTitle is in the range ["A", "FXSHRXW"].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/excel-sheet-column-number/
// discuss: https://leetcode.com/problems/excel-sheet-column-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .into_bytes()
            .into_iter()
            .fold(0, |acc, ele| acc * 26 + (ele - 65 + 1) as i32)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0171_example_1() {
        let column_title = "A".to_string();
        let result = 1;

        assert_eq!(Solution::title_to_number(column_title), result);
    }

    #[test]
    fn test_0171_example_2() {
        let column_title = "AB".to_string();
        let result = 28;

        assert_eq!(Solution::title_to_number(column_title), result);
    }

    #[test]
    fn test_0171_example_3() {
        let column_title = "ZY".to_string();
        let result = 701;

        assert_eq!(Solution::title_to_number(column_title), result);
    }

    #[test]
    fn test_0171_example_4() {
        let column_title = "FXSHRXW".to_string();
        let result = 2147483647;

        assert_eq!(Solution::title_to_number(column_title), result);
    }
}
