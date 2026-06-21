/**
 * [2514] Count Anagrams
 *
 * You are given a string s containing one or more words. Every consecutive pair of words is separated by a single space ' '.
 * A string t is an anagram of string s if the i^th word of t is a permutation of the i^th word of s.
 *
 * 	For example, "acb dfe" is an anagram of "abc def", but "def cab" and "adc bef" are not.
 *
 * Return the number of distinct anagrams of s. Since the answer may be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: s = "too hot"
 * Output: 18
 * Explanation: Some of the anagrams of the given string are "too hot", "oot hot", "oto toh", "too toh", and "too oht".
 *
 * Example 2:
 *
 * Input: s = "aa"
 * Output: 1
 * Explanation: There is only one anagram possible for the given string.
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of lowercase English letters and spaces ' '.
 * 	There is single space between consecutive words.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-anagrams/
// discuss: https://leetcode.com/problems/count-anagrams/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_anagrams(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2514_example_1() {
        let s = "too hot".to_string();

        let result = 18;

        assert_eq!(Solution::count_anagrams(s), result);
    }

    #[test]
    #[ignore]
    fn test_2514_example_2() {
        let s = "aa".to_string();

        let result = 1;

        assert_eq!(Solution::count_anagrams(s), result);
    }
}
