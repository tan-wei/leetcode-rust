/**
 * [0405] Convert a Number to Hexadecimal
 *
 * Given an integer num, return a string representing its hexadecimal representation. For negative integers, <a href="https://en.wikipedia.org/wiki/Two%27s_complement" target="_blank">two&rsquo;s complement</a> method is used.
 * All the letters in the answer string should be lowercase characters, and there should not be any leading zeros in the answer except for the zero itself.
 * Note: You are not allowed to use any built-in library method to directly solve this problem.
 *  
 * Example 1:
 * Input: num = 26
 * Output: "1a"
 * Example 2:
 * Input: num = -1
 * Output: "ffffffff"
 *  
 * Constraints:
 *
 * 	-2^31 <= num <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/convert-a-number-to-hexadecimal/
// discuss: https://leetcode.com/problems/convert-a-number-to-hexadecimal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn to_hex(num: i32) -> String {
        format!("{:x}", num)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0405_example_1() {
        let num = 26;
        let result = "1a".to_string();

        assert_eq!(Solution::to_hex(num), result);
    }

    #[test]
    fn test_0405_example_2() {
        let num = -1;
        let result = "ffffffff".to_string();

        assert_eq!(Solution::to_hex(num), result);
    }
}
