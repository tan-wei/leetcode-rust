/**
 * [1312] Minimum Insertion Steps to Make a String Palindrome
 *
 * Given a string s. In one step you can insert any character at any index of the string.
 * Return the minimum number of steps to make s palindrome.
 * A Palindrome String is one that reads the same backward as well as forward.
 *
 * Example 1:
 *
 * Input: s = "zzazz"
 * Output: 0
 * Explanation: The string "zzazz" is already palindrome we do not need any insertions.
 *
 * Example 2:
 *
 * Input: s = "mbadm"
 * Output: 2
 * Explanation: String can be "mbdadbm" or "mdbabdm".
 *
 * Example 3:
 *
 * Input: s = "leetcode"
 * Output: 5
 * Explanation: Inserting 5 characters the string becomes "leetcodocteel".
 *
 *
 * Constraints:
 *
 * 	1 <= s.length <= 500
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/
// discuss: https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/solutions/3443346/rust-dp-short-and-concise-solution/
    pub fn min_insertions(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![0; chars.len()]; chars.len() + 1];
        for i in 2..chars.len() + 1 {
            for j in 0..chars.len() - i + 1 {
                if chars[j] == chars[j + i - 1] {
                    dp[i][j] = dp[i - 2][j + 1];
                } else {
                    dp[i][j] = std::cmp::min(dp[i - 1][j], dp[i - 1][j + 1]) + 1;
                }
            }
        }

        dp[chars.len()][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1312_example_1() {
        let s = "zzazz".to_string();
        let result = 0;

        assert_eq!(Solution::min_insertions(s), result);
    }

    #[test]
    fn test_1312_example_2() {
        let s = "mbadm".to_string();
        let result = 2;

        assert_eq!(Solution::min_insertions(s), result);
    }

    #[test]
    fn test_1312_example_3() {
        let s = "leetcode".to_string();
        let result = 5;

        assert_eq!(Solution::min_insertions(s), result);
    }
}
