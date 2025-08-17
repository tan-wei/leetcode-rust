/**
 * [2124] Check if All A's Appears Before All B's
 *
 * Given a string s consisting of only the characters 'a' and 'b', return true if every 'a' appears before every 'b' in the string. Otherwise, return false.
 *  
 * Example 1:
 *
 * Input: s = "aaabbb"
 * Output: true
 * Explanation:
 * The 'a's are at indices 0, 1, and 2, while the 'b's are at indices 3, 4, and 5.
 * Hence, every 'a' appears before every 'b' and we return true.
 *
 * Example 2:
 *
 * Input: s = "abab"
 * Output: false
 * Explanation:
 * There is an 'a' at index 2 and a 'b' at index 1.
 * Hence, not every 'a' appears before every 'b' and we return false.
 *
 * Example 3:
 *
 * Input: s = "bbb"
 * Output: true
 * Explanation:
 * There are no 'a's, hence, every 'a' appears before every 'b' and we return true.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s[i] is either 'a' or 'b'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-all-as-appears-before-all-bs/
// discuss: https://leetcode.com/problems/check-if-all-as-appears-before-all-bs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_string(s: String) -> bool {
        !s.chars().skip_while(|&x| x == 'a').any(|x| x == 'a')
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2124_example_1() {
        let s = "aaabbb".to_string();

        let result = true;

        assert_eq!(Solution::check_string(s), result);
    }

    #[test]
    fn test_2124_example_2() {
        let s = "abab".to_string();

        let result = false;

        assert_eq!(Solution::check_string(s), result);
    }

    #[test]
    fn test_2124_example_3() {
        let s = "bbb".to_string();

        let result = true;

        assert_eq!(Solution::check_string(s), result);
    }
}
