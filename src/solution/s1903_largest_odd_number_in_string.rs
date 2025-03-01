/**
 * [1903] Largest Odd Number in String
 *
 * You are given a string num, representing a large integer. Return the largest-valued odd integer (as a string) that is a non-empty substring of num, or an empty string "" if no odd integer exists.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * Example 1:
 *
 * Input: num = "52"
 * Output: "5"
 * Explanation: The only non-empty substrings are "5", "2", and "52". "5" is the only odd number.
 *
 * Example 2:
 *
 * Input: num = "4206"
 * Output: ""
 * Explanation: There are no odd numbers in "4206".
 *
 * Example 3:
 *
 * Input: num = "35427"
 * Output: "35427"
 * Explanation: "35427" is already an odd number.
 *
 *  
 * Constraints:
 *
 * 	1 <= num.length <= 10^5
 * 	num only consists of digits and does not contain any leading zeros.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-odd-number-in-string/
// discuss: https://leetcode.com/problems/largest-odd-number-in-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        num.as_bytes()
            .iter()
            .rposition(|&b| b & 1 == 1)
            .map_or(String::new(), |p| num[..=p].to_string())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1903_example_1() {
        let num = "52".to_string();

        let result = "5".to_string();

        assert_eq!(Solution::largest_odd_number(num), result);
    }

    #[test]
    fn test_1903_example_2() {
        let num = "4206".to_string();

        let result = "".to_string();

        assert_eq!(Solution::largest_odd_number(num), result);
    }

    #[test]
    fn test_1903_example_3() {
        let num = "35427".to_string();

        let result = "35427".to_string();

        assert_eq!(Solution::largest_odd_number(num), result);
    }
}
