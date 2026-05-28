/**
 * [2484] Count Palindromic Subsequences
 *
 * Given a string of digits s, return the number of palindromic subsequences of s having length 5. Since the answer may be very large, return it modulo 10^9 + 7.
 * Note:
 *
 * 	A string is palindromic if it reads the same forward and backward.
 * 	A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
 *
 *  
 * Example 1:
 *
 * Input: s = "103301"
 * Output: 2
 * Explanation:
 * There are 6 possible subsequences of length 5: "10330","10331","10301","10301","13301","03301".
 * Two of them (both equal to "10301") are palindromic.
 *
 * Example 2:
 *
 * Input: s = "0000000"
 * Output: 21
 * Explanation: All 21 subsequences are "00000", which is palindromic.
 *
 * Example 3:
 *
 * Input: s = "9999900000"
 * Output: 2
 * Explanation: The only two palindromic subsequences are "99999" and "00000".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^4
 * 	s consists of digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-palindromic-subsequences/
// discuss: https://leetcode.com/problems/count-palindromic-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_palindromes(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2484_example_1() {
        let s = "103301".to_string();

        let result = 2;

        assert_eq!(Solution::count_palindromes(s), result);
    }

    #[test]
    #[ignore]
    fn test_2484_example_2() {
        let s = "0000000".to_string();

        let result = 21;

        assert_eq!(Solution::count_palindromes(s), result);
    }

    #[test]
    #[ignore]
    fn test_2484_example_3() {
        let s = "9999900000".to_string();
        let result = 2;

        assert_eq!(Solution::count_palindromes(s), result);
    }
}
