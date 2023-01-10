/**
 * [0940] Distinct Subsequences II
 *
 * Given a string s, return the number of distinct non-empty subsequences of s. Since the answer may be very large, return it modulo 10^9 + 7.
 * A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "<u>a</u>b<u>c</u>d<u>e</u>" while "aec" is not.
 *  
 * Example 1:
 *
 * Input: s = "abc"
 * Output: 7
 * Explanation: The 7 distinct subsequences are "a", "b", "c", "ab", "ac", "bc", and "abc".
 *
 * Example 2:
 *
 * Input: s = "aba"
 * Output: 6
 * Explanation: The 6 distinct subsequences are "a", "b", "ab", "aa", "ba", and "aba".
 *
 * Example 3:
 *
 * Input: s = "aaa"
 * Output: 3
 * Explanation: The 3 distinct subsequences are "a", "aa" and "aaa".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 2000
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/distinct-subsequences-ii/
// discuss: https://leetcode.com/problems/distinct-subsequences-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let mut dp = vec![0; 26];
        let module = 10u32.pow(9) as i32 + 7;
        let mut result = 0;
        let s = s.chars().collect::<Vec<char>>();
        for i in 0..=s.len() {
            result = 0;
            for j in 0..26 {
                result = (result + dp[j]) % module;
            }
            if i < s.len() {
                let index = s[i] as usize - 'a' as usize;
                dp[index] = (1 + result) % module;
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
    fn test_0940_example_1() {
        let s = "abc".to_string();
        let result = 7;

        assert_eq!(Solution::distinct_subseq_ii(s), result);
    }

    #[test]
    fn test_0940_example_2() {
        let s = "aba".to_string();
        let result = 6;

        assert_eq!(Solution::distinct_subseq_ii(s), result);
    }

    #[test]
    fn test_0940_example_3() {
        let s = "aaa".to_string();
        let result = 3;

        assert_eq!(Solution::distinct_subseq_ii(s), result);
    }
}
