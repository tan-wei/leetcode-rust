/**
 * [168] Excel Sheet Column Title
 *
 * Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.
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
 * Input: columnNumber = 1
 * Output: "A"
 *
 * Example 2:
 *
 * Input: columnNumber = 28
 * Output: "AB"
 *
 * Example 3:
 *
 * Input: columnNumber = 701
 * Output: "ZY"
 *
 * Example 4:
 *
 * Input: columnNumber = 2147483647
 * Output: "FXSHRXW"
 *
 *  
 * Constraints:
 *
 * 	1 <= columnNumber <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/excel-sheet-column-title/
// discuss: https://leetcode.com/problems/excel-sheet-column-title/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut column_number = column_number;
        std::iter::from_fn(move || match column_number {
            0 => None,
            _ => {
                column_number -= 1;
                let d = column_number % 26 + 'A' as i32;
                column_number /= 26;
                Some(d as u8 as char)
            }
        })
        .collect::<Vec<char>>()
        .into_iter()
        .rev()
        .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0168_example_1() {
        let column_number = 1;
        let result = "A".to_string();

        assert_eq!(Solution::convert_to_title(column_number), result);
    }

    #[test]
    fn test_0168_example_2() {
        let column_number = 28;
        let result = "AB".to_string();

        assert_eq!(Solution::convert_to_title(column_number), result);
    }

    #[test]
    fn test_0168_example_3() {
        let column_number = 701;
        let result = "ZY".to_string();

        assert_eq!(Solution::convert_to_title(column_number), result);
    }

    #[test]
    fn test_0168_example_4() {
        let column_number = 2147483647;
        let result = "FXSHRXW".to_string();

        assert_eq!(Solution::convert_to_title(column_number), result);
    }
}
