/**
 * [93] Restore IP Addresses
 *
 * Given a string s containing only digits, return all possible valid IP addresses that can be obtained from s. You can return them in any order.
 * A valid IP address consists of exactly four integers, each integer is between 0 and 255, separated by single dots and cannot have leading zeros. For example, "0.1.2.201" and "192.168.1.1" are valid IP addresses and "0.011.255.245", "192.168.1.312" and "192.168@1.1" are invalid IP addresses.
 *  
 * Example 1:
 * Input: s = "25525511135"
 * Output: ["255.255.11.135","255.255.111.35"]
 * Example 2:
 * Input: s = "0000"
 * Output: ["0.0.0.0"]
 * Example 3:
 * Input: s = "1111"
 * Output: ["1.1.1.1"]
 * Example 4:
 * Input: s = "010010"
 * Output: ["0.10.0.10","0.100.1.0"]
 * Example 5:
 * Input: s = "101023"
 * Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 3000
 * 	s consists of digits only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/restore-ip-addresses/
// discuss: https://leetcode.com/problems/restore-ip-addresses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/restore-ip-addresses/discuss/1119335/Rust-cheapest-and-best
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let digits = s
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        Self::helper(&digits, 4)
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|d| format!("{}", d))
                    .collect::<Vec<String>>()
                    .join(".")
            })
            .collect()
    }

    fn helper(digits: &[u32], k: usize) -> Vec<Vec<u32>> {
        match (k, digits.len()) {
            (_, 0) => vec![],
            (1, l) => {
                if l == 1 || (l < 4 && digits[0] != 0) {
                    let n = digits.iter().fold(0u32, |acc, cur| acc * 10 + cur);
                    if n <= 255 {
                        return vec![vec![n]];
                    }
                }
                vec![]
            }
            (_, l) => {
                let mut result = vec![];
                for i in 1..4.min(l) {
                    if let Some(seed) = Self::helper(&digits[0..i], 1).pop() {
                        for rest in Self::helper(&digits[i..], k - 1) {
                            let mut new = seed.clone();
                            new.extend(rest);
                            result.push(new);
                        }
                    }
                }
                result
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0093_example_1() {
        let s = "25525511135".to_string();
        let result = vec_string!["255.255.11.135", "255.255.111.35"];

        assert_eq_sorted!(Solution::restore_ip_addresses(s), result);
    }

    #[test]
    fn test_0093_example_2() {
        let s = "0000".to_string();
        let result = vec_string!["0.0.0.0"];

        assert_eq_sorted!(Solution::restore_ip_addresses(s), result);
    }

    #[test]
    fn test_0093_example_3() {
        let s = "1111".to_string();
        let result = vec_string!["1.1.1.1"];

        assert_eq_sorted!(Solution::restore_ip_addresses(s), result);
    }

    #[test]
    fn test_0093_example_4() {
        let s = "010010".to_string();
        let result = vec_string!["0.10.0.10", "0.100.1.0"];

        assert_eq_sorted!(Solution::restore_ip_addresses(s), result);
    }

    #[test]
    fn test_0093_example_5() {
        let s = "101023".to_string();
        let result = vec_string![
            "1.0.10.23",
            "1.0.102.3",
            "10.1.0.23",
            "10.10.2.3",
            "101.0.2.3"
        ];

        assert_eq_sorted!(Solution::restore_ip_addresses(s), result);
    }
}
