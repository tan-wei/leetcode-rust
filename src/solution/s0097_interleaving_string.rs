/**
 * [97] Interleaving String
 *
 * Given strings s1, s2, and s3, find whether s3 is formed by an interleaving of s1 and s2.
 * An interleaving of two strings s and t is a configuration where they are divided into non-empty substrings such that:
 *
 * 	s = s1 + s2 + ... + sn
 * 	t = t1 + t2 + ... + tm
 * 	|n - m| <= 1
 * 	The interleaving is s1 + t1 + s2 + t2 + s3 + t3 + ... or t1 + s1 + t2 + s2 + t3 + s3 + ...
 *
 * Note: a + b is the concatenation of strings a and b.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/02/interleave.jpg" style="width: 561px; height: 203px;" />
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
 * Output: true
 *
 * Example 2:
 *
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
 * Output: false
 *
 * Example 3:
 *
 * Input: s1 = "", s2 = "", s3 = ""
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	0 <= s1.length, s2.length <= 100
 * 	0 <= s3.length <= 200
 * 	s1, s2, and s3 consist of lowercase English letters.
 *
 *  
 * Follow up: Could you solve it using only O(s2.length) additional memory space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/interleaving-string/
// discuss: https://leetcode.com/problems/interleaving-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (l1, l2, l3) = (s1.len(), s2.len(), s3.len());

        if l1 + l2 != l3 {
            return false;
        }

        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut dp = vec![vec![false; l2 + 1]; l1 + 1];

        dp[0][0] = true;

        for i in 1..=l1 {
            if s1[i - 1] == s3[i - 1] {
                dp[i][0] = dp[i - 1][0];
            }
        }

        for j in 1..=l2 {
            if s2[j - 1] == s3[j - 1] {
                dp[0][j] = dp[0][j - 1];
            }
        }

        for i in 1..=l1 {
            for j in 1..=l2 {
                dp[i][j] = (dp[i - 1][j] && s3[i + j - 1] == s1[i - 1])
                    || (dp[i][j - 1] && s3[i + j - 1] == s2[j - 1]);
            }
        }

        dp[l1][l2]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0097_example_1() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbcbcac".to_string();
        let result = true;

        assert_eq!(Solution::is_interleave(s1, s2, s3), result);
    }

    #[test]
    fn test_0097_example_2() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbbaccc".to_string();
        let result = false;

        assert_eq!(Solution::is_interleave(s1, s2, s3), result);
    }

    #[test]
    fn test_0097_example_3() {
        let s1 = "".to_string();
        let s2 = "".to_string();
        let s3 = "".to_string();
        let result = true;

        assert_eq!(Solution::is_interleave(s1, s2, s3), result);
    }
}
