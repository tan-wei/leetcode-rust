/**
 * [2116] Check if a Parentheses String Can Be Valid
 *
 * A parentheses string is a non-empty string consisting only of '(' and ')'. It is valid if any of the following conditions is true:
 *
 * 	It is ().
 * 	It can be written as AB (A concatenated with B), where A and B are valid parentheses strings.
 * 	It can be written as (A), where A is a valid parentheses string.
 *
 * You are given a parentheses string s and a string locked, both of length n. locked is a binary string consisting only of '0's and '1's. For each index i of locked,
 *
 * 	If locked[i] is '1', you cannot change s[i].
 * 	But if locked[i] is '0', you can change s[i] to either '(' or ')'.
 *
 * Return true if you can make s a valid parentheses string. Otherwise, return false.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/11/06/eg1.png" style="width: 311px; height: 101px;" />
 * Input: s = "))()))", locked = "010100"
 * Output: true
 * Explanation: locked[1] == '1' and locked[3] == '1', so we cannot change s[1] or s[3].
 * We change s[0] and s[4] to '(' while leaving s[2] and s[5] unchanged to make s valid.
 * Example 2:
 *
 * Input: s = "()()", locked = "0000"
 * Output: true
 * Explanation: We do not need to make any changes because s is already valid.
 *
 * Example 3:
 *
 * Input: s = ")", locked = "0"
 * Output: false
 * Explanation: locked permits us to change s[0].
 * Changing s[0] to either '(' or ')' will not make s valid.
 *
 * Example 4:
 *
 * Input: s = "(((())(((())", locked = "111111010111"
 * Output: true
 * Explanation: locked permits us to change s[6] and s[8].
 * We change s[6] and s[8] to ')' to make s valid.
 *
 *  
 * Constraints:
 *
 * 	n == s.length == locked.length
 * 	1 <= n <= 10^5
 * 	s[i] is either '(' or ')'.
 * 	locked[i] is either '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-a-parentheses-string-can-be-valid/
// discuss: https://leetcode.com/problems/check-if-a-parentheses-string-can-be-valid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        s.len() & 1 == 0
            && s.chars()
                .zip(locked.chars())
                .fold((0, 0), |(mut min, mut max), (s, l)| {
                    min += if s != '(' || l == '0' { -1 } else { 1 };
                    max += if s == '(' || l == '0' { 1 } else { -1 };
                    if max < 0 {
                        (i32::MIN, i32::MIN)
                    } else {
                        (min.max(0), max)
                    }
                })
                .0
                == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2116_example_1() {
        let s = "))()))".to_string();
        let locked = "010100".to_string();

        let result = true;

        assert_eq!(Solution::can_be_valid(s, locked), result);
    }

    #[test]
    fn test_2116_example_2() {
        let s = "()()".to_string();
        let locked = "0000".to_string();

        let result = true;

        assert_eq!(Solution::can_be_valid(s, locked), result);
    }

    #[test]
    fn test_2116_example_3() {
        let s = ")".to_string();
        let locked = "0".to_string();

        let result = false;

        assert_eq!(Solution::can_be_valid(s, locked), result);
    }

    #[test]
    fn test_2116_example_4() {
        let s = "(((())(((())".to_string();
        let locked = "111111010111".to_string();

        let result = true;

        assert_eq!(Solution::can_be_valid(s, locked), result);
    }
}
