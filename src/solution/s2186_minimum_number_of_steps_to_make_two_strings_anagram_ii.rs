/**
 * [2186] Minimum Number of Steps to Make Two Strings Anagram II
 *
 * You are given two strings s and t. In one step, you can append any character to either s or t.
 * Return the minimum number of steps to make s and t anagrams of each other.
 * An anagram of a string is a string that contains the same characters with a different (or the same) ordering.
 *  
 * Example 1:
 *
 * Input: s = "<u>lee</u>tco<u>de</u>", t = "co<u>a</u>t<u>s</u>"
 * Output: 7
 * Explanation:
 * - In 2 steps, we can append the letters in "as" onto s = "leetcode", forming s = "leetcode<u>as</u>".
 * - In 5 steps, we can append the letters in "leede" onto t = "coats", forming t = "coats<u>leede</u>".
 * "leetcodeas" and "coatsleede" are now anagrams of each other.
 * We used a total of 2 + 5 = 7 steps.
 * It can be shown that there is no way to make them anagrams of each other with less than 7 steps.
 *
 * Example 2:
 *
 * Input: s = "night", t = "thing"
 * Output: 0
 * Explanation: The given strings are already anagrams of each other. Thus, we do not need any further steps.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length, t.length <= 2 * 10^5
 * 	s and t consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram-ii/
// discuss: https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2186_example_1() {
        let s = "leetcode".to_string();
        let t = "coats".to_string();

        let result = 7;

        assert_eq!(Solution::min_steps(s, t), result);
    }

    #[test]
    #[ignore]
    fn test_2186_example_2() {
        let s = "night".to_string();
        let t = "thing".to_string();

        let result = 0;

        assert_eq!(Solution::min_steps(s, t), result);
    }
}
