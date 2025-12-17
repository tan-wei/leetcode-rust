/**
 * [2278] Percentage of Letter in String
 *
 * Given a string s and a character letter, return the percentage of characters in s that equal letter rounded down to the nearest whole percent.
 *  
 * Example 1:
 *
 * Input: s = "foobar", letter = "o"
 * Output: 33
 * Explanation:
 * The percentage of characters in s that equal the letter 'o' is 2 / 6 * 100% = 33% when rounded down, so we return 33.
 *
 * Example 2:
 *
 * Input: s = "jjjj", letter = "k"
 * Output: 0
 * Explanation:
 * The percentage of characters in s that equal the letter 'k' is 0%, so we return 0.
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s consists of lowercase English letters.
 * 	letter is a lowercase English letter.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/percentage-of-letter-in-string/
// discuss: https://leetcode.com/problems/percentage-of-letter-in-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        s.as_bytes()
            .iter()
            .fold(0, |acc, c| acc + if *c == letter as u8 { 1 } else { 0 })
            * 100
            / s.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2278_example_1() {
        let s = "foobar".to_string();
        let letter = 'o';

        let result = 33;

        assert_eq!(Solution::percentage_letter(s, letter), result);
    }

    #[test]
    fn test_2278_example_2() {
        let s = "jjjj".to_string();
        let letter = 'k';

        let result = 0;

        assert_eq!(Solution::percentage_letter(s, letter), result);
    }
}
