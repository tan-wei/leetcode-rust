/**
 * [2030] Smallest K-Length Subsequence With Occurrences of a Letter
 *
 * You are given a string s, an integer k, a letter letter, and an integer repetition.
 * Return the lexicographically smallest subsequence of s of length k that has the letter letter appear at least repetition times. The test cases are generated so that the letter appears in s at least repetition times.
 * A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
 * A string a is lexicographically smaller than a string b if in the first position where a and b differ, string a has a letter that appears earlier in the alphabet than the corresponding letter in b.
 *  
 * Example 1:
 *
 * Input: s = "leet", k = 3, letter = "e", repetition = 1
 * Output: "eet"
 * Explanation: There are four subsequences of length 3 that have the letter 'e' appear at least 1 time:
 * - "lee" (from "<u>lee</u>t")
 * - "let" (from "<u>le</u>e<u>t</u>")
 * - "let" (from "<u>l</u>e<u>et</u>")
 * - "eet" (from "l<u>eet</u>")
 * The lexicographically smallest subsequence among them is "eet".
 *
 * Example 2:
 * <img alt="example-2" src="https://assets.leetcode.com/uploads/2021/09/13/smallest-k-length-subsequence.png" style="width: 339px; height: 67px;" />
 * Input: s = "leetcode", k = 4, letter = "e", repetition = 2
 * Output: "ecde"
 * Explanation: "ecde" is the lexicographically smallest subsequence of length 4 that has the letter "e" appear at least 2 times.
 *
 * Example 3:
 *
 * Input: s = "bb", k = 2, letter = "b", repetition = 2
 * Output: "bb"
 * Explanation: "bb" is the only subsequence of length 2 that has the letter "b" appear at least 2 times.
 *
 *  
 * Constraints:
 *
 * 	1 <= repetition <= k <= s.length <= 5 * 10^4
 * 	s consists of lowercase English letters.
 * 	letter is a lowercase English letter, and appears in s at least repetition times.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-k-length-subsequence-with-occurrences-of-a-letter/
// discuss: https://leetcode.com/problems/smallest-k-length-subsequence-with-occurrences-of-a-letter/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2030_example_1() {
        let s = "leet".to_string();
        let k = 3;
        let letter = "e".to_string();
        let repetition = 1;

        let result = "eet".to_string();

        assert_eq!(
            Solution::smallest_subsequence(s, k, letter, repetition),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2030_example_2() {
        let s = "leetcode".to_string();
        let k = 4;
        let letter = "e".to_string();
        let repetition = 2;

        let result = "ecde".to_string();

        assert_eq!(
            Solution::smallest_subsequence(s, k, letter, repetition),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2030_example_3() {
        let s = "bb".to_string();
        let k = 2;
        let letter = "eb".to_string();
        let repetition = 2;

        let result = "bb".to_string();

        assert_eq!(
            Solution::smallest_subsequence(s, k, letter, repetition),
            result
        );
    }
}
