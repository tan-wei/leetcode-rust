/**
 * [0806] Number of Lines To Write String
 *
 * You are given a string s of lowercase English letters and an array widths denoting how many pixels wide each lowercase English letter is. Specifically, widths[0] is the width of 'a', widths[1] is the width of 'b', and so on.
 * You are trying to write s across several lines, where each line is no longer than 100 pixels. Starting at the beginning of s, write as many letters on the first line such that the total width does not exceed 100 pixels. Then, from where you stopped in s, continue writing as many letters as you can on the second line. Continue this process until you have written all of s.
 * Return an array result of length 2 where:
 *
 * 	result[0] is the total number of lines.
 * 	result[1] is the width of the last line in pixels.
 *
 *  
 * Example 1:
 *
 * Input: widths = [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10], s = "abcdefghijklmnopqrstuvwxyz"
 * Output: [3,60]
 * Explanation: You can write s as follows:
 * abcdefghij  // 100 pixels wide
 * klmnopqrst  // 100 pixels wide
 * uvwxyz      // 60 pixels wide
 * There are a total of 3 lines, and the last line is 60 pixels wide.
 * Example 2:
 *
 * Input: widths = [4,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10], s = "bbbcccdddaaa"
 * Output: [2,4]
 * Explanation: You can write s as follows:
 * bbbcccdddaa  // 98 pixels wide
 * a            // 4 pixels wide
 * There are a total of 2 lines, and the last line is 4 pixels wide.
 *  
 * Constraints:
 *
 * 	widths.length == 26
 * 	2 <= widths[i] <= 10
 * 	1 <= s.length <= 1000
 * 	s contains only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-lines-to-write-string/
// discuss: https://leetcode.com/problems/number-of-lines-to-write-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let s = s.bytes().map(|x| (x - 97) as usize).collect::<Vec<usize>>();
        let (mut lines, mut width) = (1, 0);

        for index in s {
            let w = widths[index];
            width += w;
            if width > 100 {
                lines += 1;
                width = w;
            }
        }

        vec![lines, width as i32]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0806_example_1() {
        let widths = vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10,
        ];
        let s = "abcdefghijklmnopqrstuvwxyz".to_string();
        let result = vec![3, 60];

        assert_eq!(Solution::number_of_lines(widths, s), result);
    }

    #[test]
    fn test_0806_example_2() {
        let widths = vec![
            4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10,
        ];
        let s = "bbbcccdddaaa".to_string();
        let result = vec![2, 4];

        assert_eq!(Solution::number_of_lines(widths, s), result);
    }
}
