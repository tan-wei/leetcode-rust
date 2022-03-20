/**
 * [0583] Delete Operation for Two Strings
 *
 * Given two strings word1 and word2, return the minimum number of steps required to make word1 and word2 the same.
 * In one step, you can delete exactly one character in either string.
 *  
 * Example 1:
 *
 * Input: word1 = "sea", word2 = "eat"
 * Output: 2
 * Explanation: You need one step to make "sea" to "ea" and another step to make "eat" to "ea".
 *
 * Example 2:
 *
 * Input: word1 = "leetcode", word2 = "etco"
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= word1.length, word2.length <= 500
 * 	word1 and word2 consist of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/delete-operation-for-two-strings/
// discuss: https://leetcode.com/problems/delete-operation-for-two-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let s1 = word1.as_bytes();
        let s2 = word2.as_bytes();

        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

        for (i, b1) in s1.iter().enumerate() {
            for (j, b2) in s2.iter().enumerate() {
                dp[i + 1][j + 1] = if b1 == b2 {
                    dp[i][j] + 1
                } else {
                    dp[i + 1][j].max(dp[i][j + 1])
                }
            }
        }

        (s1.len() + s2.len() - 2 * dp[s1.len()][s2.len()]) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0583_example_1() {
        let word1 = "sea".to_string();
        let word2 = "eat".to_string();
        let result = 2;

        assert_eq!(Solution::min_distance(word1, word2), result);
    }

    #[test]
    fn test_0583_example_2() {
        let word1 = "leetcode".to_string();
        let word2 = "etco".to_string();
        let result = 4;

        assert_eq!(Solution::min_distance(word1, word2), result);
    }
}
