/**
 * [1400] Construct K Palindrome Strings
 *
 * Given a string s and an integer k, return true if you can use all the characters in s to construct k palindrome strings or false otherwise.
 *  
 * Example 1:
 *
 * Input: s = "annabelle", k = 2
 * Output: true
 * Explanation: You can construct two palindromes using all characters in s.
 * Some possible constructions "anna" + "elble", "anbna" + "elle", "anellena" + "b"
 *
 * Example 2:
 *
 * Input: s = "leetcode", k = 3
 * Output: false
 * Explanation: It is impossible to construct 3 palindromes using all the characters of s.
 *
 * Example 3:
 *
 * Input: s = "true", k = 4
 * Output: true
 * Explanation: The only possible solution is to put each character in a separate string.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of lowercase English letters.
 * 	1 <= k <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/construct-k-palindrome-strings/
// discuss: https://leetcode.com/problems/construct-k-palindrome-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if (s.len() as i32) < k {
            return false;
        }

        let mut memo = vec![0; 26];
        for c in s.chars() {
            memo[(c as u8 - 'a' as u8) as usize] += 1;
        }

        let mut odds = 0;
        for v in memo {
            if v % 2 == 1 {
                odds += 1;
            }
        }

        k >= odds
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1400_example_1() {
        let s = "annabelle".to_string();
        let k = 2;

        let result = true;

        assert_eq!(Solution::can_construct(s, k), result);
    }

    #[test]
    fn test_1400_example_2() {
        let s = "leetcode".to_string();
        let k = 3;

        let result = false;

        assert_eq!(Solution::can_construct(s, k), result);
    }

    #[test]
    fn test_1400_example_3() {
        let s = "true".to_string();
        let k = 4;

        let result = true;

        assert_eq!(Solution::can_construct(s, k), result);
    }
}
