/**
 * [0482] License Key Formatting
 *
 * You are given a license key represented as a string s that consists of only alphanumeric characters and dashes. The string is separated into n + 1 groups by n dashes. You are also given an integer k.
 * We want to reformat the string s such that each group contains exactly k characters, except for the first group, which could be shorter than k but still must contain at least one character. Furthermore, there must be a dash inserted between two groups, and you should convert all lowercase letters to uppercase.
 * Return the reformatted license key.
 *  
 * Example 1:
 *
 * Input: s = "5F3Z-2e-9-w", k = 4
 * Output: "5F3Z-2E9W"
 * Explanation: The string s has been split into two parts, each part has 4 characters.
 * Note that the two extra dashes are not needed and can be removed.
 *
 * Example 2:
 *
 * Input: s = "2-5g-3-J", k = 2
 * Output: "2-5G-3J"
 * Explanation: The string s has been split into three parts, each part has 2 characters except the first part as it could be shorter as mentioned above.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of English letters, digits, and dashes '-'.
 * 	1 <= k <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/license-key-formatting/
// discuss: https://leetcode.com/problems/license-key-formatting/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        s.to_ascii_uppercase()
            .replace("-", "")
            .as_bytes()
            .rchunks(k as usize)
            .rev()
            .map(|x| String::from_utf8_lossy(x))
            .collect::<Vec<_>>()
            .join("-")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0482_example_1() {
        let s = "5F3Z-2e-9-w".to_string();
        let k = 4;
        let result = "5F3Z-2E9W".to_string();

        assert_eq!(Solution::license_key_formatting(s, k), result);
    }

    #[test]
    fn test_0482_example_2() {
        let s = "2-5g-3-J".to_string();
        let k = 2;
        let result = "2-5G-3J".to_string();

        assert_eq!(Solution::license_key_formatting(s, k), result);
    }
}
