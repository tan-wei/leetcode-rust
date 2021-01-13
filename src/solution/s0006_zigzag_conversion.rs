/**
 * [6] ZigZag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 * And then read line by line: "PAHNAPLSIIGYIR"
 * Write the code that will take a string and make this conversion given a number of rows:
 *
 * string convert(string s, int numRows);
 *
 *  
 * Example 1:
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 * Example 2:
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 * Example 3:
 *
 * Input: s = "A", numRows = 1
 * Output: "A"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consists of English letters (lower-case and upper-case), ',' and '.'.
 * 	1 <= numRows <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/zigzag-conversion/
// discuss: https://leetcode.com/problems/zigzag-conversion/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();

        if num_rows == 1 {
            return s;
        }

        for i in 0..num_rows {
            let step1 = (num_rows - i - 1) * 2;
            let step2 = i * 2;
            let mut pos: usize = i as usize;

            if pos < s.len() {
                result.push(chars[pos]);
            }

            loop {
                pos += step1 as usize;
                if pos >= s.len() {
                    break;
                }

                if step1 > 0 {
                    result.push(chars[pos]);
                }

                pos += step2 as usize;

                if pos >= s.len() {
                    break;
                }

                if step2 > 0 {
                    result.push(chars[pos]);
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
    fn test_0006_example_1() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;

        assert_eq!(Solution::convert(s, num_rows), "PAHNAPLSIIGYIR".to_string());
    }

    #[test]
    fn test_0006_example_2() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;

        assert_eq!(Solution::convert(s, num_rows), "PINALSIGYAHRPI".to_string());
    }

    #[test]
    fn test_0006_example_3() {
        let s = "A".to_string();
        let num_rows = 1;

        assert_eq!(Solution::convert(s, num_rows), "A".to_string());
    }
}
