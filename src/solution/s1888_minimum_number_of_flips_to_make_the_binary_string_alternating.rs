/**
 * [1888] Minimum Number of Flips to Make the Binary String Alternating
 *
 * You are given a binary string s. You are allowed to perform two types of operations on the string in any sequence:
 *
 * 	Type-1: Remove the character at the start of the string s and append it to the end of the string.
 * 	Type-2: Pick any character in s and flip its value, i.e., if its value is '0' it becomes '1' and vice-versa.
 *
 * Return the minimum number of type-2 operations you need to perform such that s becomes alternating.
 * The string is called alternating if no two adjacent characters are equal.
 *
 * 	For example, the strings "010" and "1010" are alternating, while the string "0100" is not.
 *
 *  
 * Example 1:
 *
 * Input: s = "111000"
 * Output: 2
 * Explanation: Use the first operation two times to make s = "100011".
 * Then, use the second operation on the third and sixth elements to make s = "10<u>1</u>01<u>0</u>".
 *
 * Example 2:
 *
 * Input: s = "010"
 * Output: 0
 * Explanation: The string is already alternating.
 *
 * Example 3:
 *
 * Input: s = "1110"
 * Output: 1
 * Explanation: Use the second operation on the second element to make s = "1<u>0</u>10".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is either '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-flips-to-make-the-binary-string-alternating/
// discuss: https://leetcode.com/problems/minimum-number-of-flips-to-make-the-binary-string-alternating/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1888_example_1() {
        let s = "111000".to_string();

        let result = 2;

        assert_eq!(Solution::min_flips(s), result);
    }

    #[test]
    #[ignore]
    fn test_1888_example_2() {
        let s = "010".to_string();

        let result = 0;

        assert_eq!(Solution::min_flips(s), result);
    }

    #[test]
    #[ignore]
    fn test_1888_example_3() {
        let s = "1110".to_string();

        let result = 1;

        assert_eq!(Solution::min_flips(s), result);
    }
}
