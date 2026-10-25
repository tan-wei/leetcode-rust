/**
 * [2663] Lexicographically Smallest Beautiful String
 *
 * A string is beautiful if:
 *
 * 	It consists of the first k letters of the English lowercase alphabet.
 * 	It does not contain any substring of length 2 or more which is a palindrome.
 *
 * You are given a beautiful string s of length n and a positive integer k.
 * Return the lexicographically smallest string of length n, which is larger than s and is beautiful. If there is no such string, return an empty string.
 * A string a is lexicographically larger than a string b (of the same length) if in the first position where a and b differ, a has a character strictly larger than the corresponding character in b.
 *
 * 	For example, "abcd" is lexicographically larger than "abcc" because the first position they differ is at the fourth character, and d is greater than c.
 *
 *  
 * Example 1:
 *
 * Input: s = "abcz", k = 26
 * Output: "abda"
 * Explanation: The string "abda" is beautiful and lexicographically larger than the string "abcz".
 * It can be proven that there is no string that is lexicographically larger than the string "abcz", beautiful, and lexicographically smaller than the string "abda".
 *
 * Example 2:
 *
 * Input: s = "dc", k = 4
 * Output: ""
 * Explanation: It can be proven that there is no string that is lexicographically larger than the string "dc" and is beautiful.
 *
 *  
 * Constraints:
 *
 * 	1 <= n == s.length <= 10^5
 * 	4 <= k <= 26
 * 	s is a beautiful string.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/lexicographically-smallest-beautiful-string/
// discuss: https://leetcode.com/problems/lexicographically-smallest-beautiful-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_beautiful_string(s: String, k: i32) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2663_example_1() {
        let s = "abcz".to_string();
        let k = 26;

        let result = "abda".to_string();

        assert_eq!(Solution::smallest_beautiful_string(s, k), result);
    }

    #[test]
    #[ignore]
    fn test_2663_example_2() {
        let s = "dc".to_string();
        let k = 4;

        let result = "".to_string();

        assert_eq!(Solution::smallest_beautiful_string(s, k), result);
    }
}
