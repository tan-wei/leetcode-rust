/**
 * [1234] Replace the Substring for Balanced String
 *
 * You are given a string s of length n containing only four kinds of characters: 'Q', 'W', 'E', and 'R'.
 * A string is said to be balanced if each of its characters appears n / 4 times where n is the length of the string.
 * Return the minimum length of the substring that can be replaced with any other string of the same length to make s balanced. If s is already balanced, return 0.
 *  
 * Example 1:
 *
 * Input: s = "QWER"
 * Output: 0
 * Explanation: s is already balanced.
 *
 * Example 2:
 *
 * Input: s = "QQWE"
 * Output: 1
 * Explanation: We need to replace a 'Q' to 'R', so that "RQWE" (or "QRWE") is balanced.
 *
 * Example 3:
 *
 * Input: s = "QQQW"
 * Output: 2
 * Explanation: We can replace the first "QQ" to "ER".
 *
 *  
 * Constraints:
 *
 * 	n == s.length
 * 	4 <= n <= 10^5
 * 	n is a multiple of 4.
 * 	s contains only 'Q', 'W', 'E', and 'R'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/replace-the-substring-for-balanced-string/
// discuss: https://leetcode.com/problems/replace-the-substring-for-balanced-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = s.len();
        let mut result = n as i32;
        let mut i = 0;
        let k = n / 4;
        let mut count = [0; 128];

        for j in 0..n {
            count[bytes[j] as usize] += 1;
        }

        for j in 0..n {
            count[bytes[j] as usize] -= 1;
            while i < n
                && count[b'Q' as usize] <= k
                && count[b'W' as usize] <= k
                && count[b'E' as usize] <= k
                && count[b'R' as usize] <= k
            {
                result = result.min(j as i32 - i as i32 + 1);
                count[bytes[i] as usize] += 1;
                i += 1;
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
    fn test_1234_example_1() {
        let s = "QWER".to_string();
        let result = 0;

        assert_eq!(Solution::balanced_string(s), result);
    }

    #[test]
    fn test_1234_example_2() {
        let s = "QQWE".to_string();
        let result = 1;

        assert_eq!(Solution::balanced_string(s), result);
    }

    #[test]
    fn test_1234_example_3() {
        let s = "QQQW".to_string();
        let result = 2;

        assert_eq!(Solution::balanced_string(s), result);
    }
}
