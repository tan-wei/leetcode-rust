/**
 * [1221] Split a String in Balanced Strings
 *
 * Balanced strings are those that have an equal quantity of 'L' and 'R' characters.
 * Given a balanced string s, split it into some number of substrings such that:
 *
 * 	Each substring is balanced.
 *
 * Return the maximum number of balanced strings you can obtain.
 *  
 * Example 1:
 *
 * Input: s = "RLRRLLRLRL"
 * Output: 4
 * Explanation: s can be split into "RL", "RRLL", "RL", "RL", each substring contains same number of 'L' and 'R'.
 *
 * Example 2:
 *
 * Input: s = "RLRRRLLRLL"
 * Output: 2
 * Explanation: s can be split into "RL", "RRRLLRLL", each substring contains same number of 'L' and 'R'.
 * Note that s cannot be split into "RL", "RR", "RL", "LR", "LL", because the 2^nd and 5^th substrings are not balanced.
 * Example 3:
 *
 * Input: s = "LLLLRRRR"
 * Output: 1
 * Explanation: s can be split into "LLLLRRRR".
 *
 *  
 * Constraints:
 *
 * 	2 <= s.length <= 1000
 * 	s[i] is either 'L' or 'R'.
 * 	s is a balanced string.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/split-a-string-in-balanced-strings/
// discuss: https://leetcode.com/problems/split-a-string-in-balanced-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut result = 0;
        let mut count = 0;
        for c in s.chars() {
            count += if c == 'L' { 1 } else { -1 };
            if count == 0 {
                result += 1;
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
    fn test_1221_example_1() {
        let s = "RLRRLLRLRL".to_string();
        let result = 4;

        assert_eq!(Solution::balanced_string_split(s), result);
    }

    #[test]
    fn test_1221_example_2() {
        let s = "RLRRRLLRLL".to_string();
        let result = 2;

        assert_eq!(Solution::balanced_string_split(s), result);
    }

    #[test]
    fn test_1221_example_3() {
        let s = "LLLLRRRR".to_string();
        let result = 1;

        assert_eq!(Solution::balanced_string_split(s), result);
    }
}
