/**
 * [1108] Defanging an IP Address
 *
 * Given a valid (IPv4) IP address, return a defanged version of that IP address.
 *
 * A defanged IP address replaces every period "." with "[.]".
 *
 *  
 * Example 1:
 * Input: address = "1.1.1.1"
 * Output: "1[.]1[.]1[.]1"
 * Example 2:
 * Input: address = "255.100.50.0"
 * Output: "255[.]100[.]50[.]0"
 *
 *  
 * Constraints:
 *
 *
 * 	The given address is a valid IPv4 address.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/defanging-an-ip-address/
// discuss: https://leetcode.com/problems/defanging-an-ip-address/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace('.', "[.]")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1108_example_1() {
        let address = "1.1.1.1".to_string();
        let result = "1[.]1[.]1[.]1".to_string();

        assert_eq!(Solution::defang_i_paddr(address), result);
    }

    #[test]
    fn test_1108_example_2() {
        let address = "255.100.50.0".to_string();
        let result = "255[.]100[.]50[.]0".to_string();

        assert_eq!(Solution::defang_i_paddr(address), result);
    }
}
