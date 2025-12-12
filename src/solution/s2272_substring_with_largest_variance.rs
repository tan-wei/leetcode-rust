/**
 * [2272] Substring With Largest Variance
 *
 * The variance of a string is defined as the largest difference between the number of occurrences of any 2 characters present in the string. Note the two characters may or may not be the same.
 * Given a string s consisting of lowercase English letters only, return the largest variance possible among all substrings of s.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * Example 1:
 *
 * Input: s = "aababbb"
 * Output: 3
 * Explanation:
 * All possible variances along with their respective substrings are listed below:
 * - Variance 0 for substrings "a", "aa", "ab", "abab", "aababb", "ba", "b", "bb", and "bbb".
 * - Variance 1 for substrings "aab", "aba", "abb", "aabab", "ababb", "aababbb", and "bab".
 * - Variance 2 for substrings "aaba", "ababbb", "abbb", and "babb".
 * - Variance 3 for substring "babbb".
 * Since the largest possible variance is 3, we return it.
 *
 * Example 2:
 *
 * Input: s = "abcde"
 * Output: 0
 * Explanation:
 * No letter occurs more than once in s, so the variance of every substring is 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^4
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/substring-with-largest-variance/
// discuss: https://leetcode.com/problems/substring-with-largest-variance/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2272_example_1() {
        let s = "aababbb".to_string();

        let result = 3;

        assert_eq!(Solution::largest_variance(s), result);
    }

    #[test]
    #[ignore]
    fn test_2272_example_2() {
        let s = "abcde".to_string();

        let result = 0;

        assert_eq!(Solution::largest_variance(s), result);
    }
}
