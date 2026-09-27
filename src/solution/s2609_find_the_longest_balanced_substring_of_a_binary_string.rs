/**
 * [2609] Find the Longest Balanced Substring of a Binary String
 *
 * You are given a binary string s consisting only of zeroes and ones.
 * A substring of s is considered balanced if all zeroes are before ones and the number of zeroes is equal to the number of ones inside the substring. Notice that the empty substring is considered a balanced substring.
 * Return the length of the longest balanced substring of s.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * Example 1:
 *
 * Input: s = "01000111"
 * Output: 6
 * Explanation: The longest balanced substring is "000111", which has length 6.
 *
 * Example 2:
 *
 * Input: s = "00111"
 * Output: 4
 * Explanation: The longest balanced substring is "0011", which has length 4.
 *
 * Example 3:
 *
 * Input: s = "111"
 * Output: 0
 * Explanation: There is no balanced substring except the empty substring, so the answer is 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 50
 * 	'0' <= s[i] <= '1'
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-longest-balanced-substring-of-a-binary-string/
// discuss: https://leetcode.com/problems/find-the-longest-balanced-substring-of-a-binary-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut result = 0;
        let mut temp = "01".to_string();

        while temp.len() <= s.len() {
            if s.contains(&temp) {
                result = temp.len() as i32;
            }
            temp = format!("0{}1", temp);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2609_example_1() {
        let s = "01000111".to_string();

        let result = 6;

        assert_eq!(Solution::find_the_longest_balanced_substring(s), result);
    }

    #[test]
    fn test_2609_example_2() {
        let s = "00111".to_string();

        let result = 4;

        assert_eq!(Solution::find_the_longest_balanced_substring(s), result);
    }

    #[test]
    fn test_2609_example_3() {
        let s = "111".to_string();

        let result = 0;

        assert_eq!(Solution::find_the_longest_balanced_substring(s), result);
    }
}
