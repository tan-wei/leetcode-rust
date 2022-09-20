/**
 * [0828] Count Unique Characters of All Substrings of a Given String
 *
 * Let's define a function countUniqueChars(s) that returns the number of unique characters on s.
 *
 * 	For example, calling countUniqueChars(s) if s = "LEETCODE" then "L", "T", "C", "O", "D" are the unique characters since they appear only once in s, therefore countUniqueChars(s) = 5.
 *
 * Given a string s, return the sum of countUniqueChars(t) where t is a substring of s. The test cases are generated such that the answer fits in a 32-bit integer.
 * Notice that some substrings can be repeated so in this case you have to count the repeated ones too.
 *  
 * Example 1:
 *
 * Input: s = "ABC"
 * Output: 10
 * Explanation: All possible substrings are: "A","B","C","AB","BC" and "ABC".
 * Every substring is composed with only unique letters.
 * Sum of lengths of all substring is 1 + 1 + 1 + 2 + 2 + 3 = 10
 *
 * Example 2:
 *
 * Input: s = "ABA"
 * Output: 8
 * Explanation: The same as example 1, except countUniqueChars("ABA") = 1.
 *
 * Example 3:
 *
 * Input: s = "LEETCODE"
 * Output: 92
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of uppercase English letters only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-unique-characters-of-all-substrings-of-a-given-string/
// discuss: https://leetcode.com/problems/count-unique-characters-of-all-substrings-of-a-given-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/count-unique-characters-of-all-substrings-of-a-given-string/discuss/2221291/Rust-O(n)-time-O(1)-Space
    pub fn unique_letter_string(s: String) -> i32 {
        let len = s.len() as i32;
        let mut left = std::collections::HashMap::new();
        let mut pos = std::collections::HashMap::new();

        let mut result = 0;

        for (i, ch) in s.chars().enumerate() {
            if let Some(mid) = pos.insert(ch, i as i32) {
                let left_dis = mid - left.insert(ch, mid).unwrap_or(-1);
                let right_dis = i as i32 - mid;
                result += left_dis * right_dis;
            }
        }

        for ch in 'A'..='Z' {
            if let Some(&mid) = pos.get(&ch) {
                result += (len - mid) * (mid - *left.get(&ch).unwrap_or(&-1));
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
    fn test_0828_example_1() {
        let s = "ABC".to_string();
        let result = 10;

        assert_eq!(Solution::unique_letter_string(s), result);
    }

    #[test]
    fn test_0828_example_2() {
        let s = "ABA".to_string();
        let result = 8;

        assert_eq!(Solution::unique_letter_string(s), result);
    }

    #[test]
    fn test_0828_example_3() {
        let s = "LEETCODE".to_string();
        let result = 92;

        assert_eq!(Solution::unique_letter_string(s), result);
    }
}
