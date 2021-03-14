/**
 * [67] Add Binary
 *
 * Given two binary strings a and b, return their sum as a binary string.
 *  
 * Example 1:
 * Input: a = "11", b = "1"
 * Output: "100"
 * Example 2:
 * Input: a = "1010", b = "1011"
 * Output: "10101"
 *  
 * Constraints:
 *
 * 	1 <= a.length, b.length <= 10^4
 * 	a and b consist only of '0' or '1' characters.
 * 	Each string does not contain leading zeros except for the zero itself.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/add-binary/
// discuss: https://leetcode.com/problems/add-binary/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        format!(
            "{:b}",
            u128::from_str_radix(&a, 2).unwrap() + u128::from_str_radix(&b, 2).unwrap()
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0067_example_1() {
        let a = "11".to_string();
        let b = "1".to_string();
        let result = "100".to_string();

        assert_eq!(Solution::add_binary(a, b), result);
    }

    #[test]
    fn test_0067_example_2() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        let result = "10101".to_string();

        assert_eq!(Solution::add_binary(a, b), result);
    }
}
