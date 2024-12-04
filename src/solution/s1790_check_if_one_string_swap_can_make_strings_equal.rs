/**
 * [1790] Check if One String Swap Can Make Strings Equal
 *
 * You are given two strings s1 and s2 of equal length. A string swap is an operation where you choose two indices in a string (not necessarily different) and swap the characters at these indices.
 * Return true if it is possible to make both strings equal by performing at most one string swap on exactly one of the strings. Otherwise, return false.
 *  
 * Example 1:
 *
 * Input: s1 = "bank", s2 = "kanb"
 * Output: true
 * Explanation: For example, swap the first character with the last character of s2 to make "bank".
 *
 * Example 2:
 *
 * Input: s1 = "attack", s2 = "defend"
 * Output: false
 * Explanation: It is impossible to make them equal with one string swap.
 *
 * Example 3:
 *
 * Input: s1 = "kelb", s2 = "kelb"
 * Output: true
 * Explanation: The two strings are already equal, so no string swap operation is required.
 *
 *  
 * Constraints:
 *
 * 	1 <= s1.length, s2.length <= 100
 * 	s1.length == s2.length
 * 	s1 and s2 consist of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/
// discuss: https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let diff = s1
            .chars()
            .zip(s2.chars())
            .filter(|(c1, c2)| *c1 != *c2)
            .take(3)
            .collect::<Vec<_>>();
        diff.is_empty() || (diff.len() == 2 && diff[0].0 == diff[1].1 && diff[0].1 == diff[1].0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1790_example_1() {
        let s1 = "bank".to_string();
        let s2 = "kanb".to_string();

        let result = true;

        assert_eq!(Solution::are_almost_equal(s1, s2), result);
    }

    #[test]
    fn test_1790_example_2() {
        let s1 = "attack".to_string();
        let s2 = "defend".to_string();

        let result = false;

        assert_eq!(Solution::are_almost_equal(s1, s2), result);
    }

    #[test]
    fn test_1790_example_3() {
        let s1 = "kelb".to_string();
        let s2 = "kelb".to_string();

        let result = true;

        assert_eq!(Solution::are_almost_equal(s1, s2), result);
    }
}
