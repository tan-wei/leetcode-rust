/**
 * [0468] Validate IP Address
 *
 * Given a string queryIP, return "IPv4" if IP is a valid IPv4 address, "IPv6" if IP is a valid IPv6 address or "Neither" if IP is not a correct IP of any type.
 * A valid IPv4 address is an IP in the form "x1.x2.x3.x4" where 0 <= xi <= 255 and xi cannot contain leading zeros. For example, "192.168.1.1" and "192.168.1.0" are valid IPv4 addresses but "192.168.01.1", while "192.168.1.00" and "192.168@1.1" are invalid IPv4 addresses.
 * A valid IPv6 address is an IP in the form "x1:x2:x3:x4:x5:x6:x7:x8" where:
 *
 * 	1 <= xi.length <= 4
 * 	xi is a hexadecimal string which may contain digits, lower-case English letter ('a' to 'f') and upper-case English letters ('A' to 'F').
 * 	Leading zeros are allowed in xi.
 *
 * For example, "2001:0db8:85a3:0000:0000:8a2e:0370:7334" and "2001:db8:85a3:0:0:8A2E:0370:7334" are valid IPv6 addresses, while "2001:0db8:85a3::8A2E:037j:7334" and "02001:0db8:85a3:0000:0000:8a2e:0370:7334" are invalid IPv6 addresses.
 *  
 * Example 1:
 *
 * Input: queryIP = "172.16.254.1"
 * Output: "IPv4"
 * Explanation: This is a valid IPv4 address, return "IPv4".
 *
 * Example 2:
 *
 * Input: queryIP = "2001:0db8:85a3:0:0:8A2E:0370:7334"
 * Output: "IPv6"
 * Explanation: This is a valid IPv6 address, return "IPv6".
 *
 * Example 3:
 *
 * Input: queryIP = "256.256.256.256"
 * Output: "Neither"
 * Explanation: This is neither a IPv4 address nor a IPv6 address.
 *
 *  
 * Constraints:
 *
 * 	queryIP consists only of English letters, digits and the characters '.' and ':'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/validate-ip-address/
// discuss: https://leetcode.com/problems/validate-ip-address/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/validate-ip-address/discuss/1533935/Rust-fail-fast
enum IPAddressType {
    IPv4,
    IPv6,
    Neither,
}

impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        match Self::_check_ip_address(query_ip) {
            IPAddressType::IPv4 => String::from("IPv4"),
            IPAddressType::IPv6 => String::from("IPv6"),
            IPAddressType::Neither => String::from("Neither"),
        }
    }

    fn _check_ip_address(ip: String) -> IPAddressType {
        if ip.len() < 7 || ip.len() > 39 {
            IPAddressType::Neither
        } else if ip.chars().filter(|&x| x == '.').count() == 3 {
            if ip.chars().any(|x| !(x.is_digit(10) || x == '.')) || ip.len() > 15 {
                IPAddressType::Neither
            } else {
                for segment in ip.split('.') {
                    if segment.len() == 0
                        || segment.len() > 3
                        || (segment.len() >= 2 && segment.chars().nth(0).unwrap() == '0')
                    {
                        return IPAddressType::Neither;
                    }
                    if segment.parse::<u8>().is_err() {
                        return IPAddressType::Neither;
                    }
                }
                IPAddressType::IPv4
            }
        } else if ip.chars().filter(|&x| x == ':').count() == 7 {
            if ip.chars().any(|x| !(x.is_digit(16) || x == ':')) || ip.len() < 15 {
                IPAddressType::Neither
            } else {
                for segment in ip.split(':') {
                    if segment.len() == 0 || segment.len() > 4 {
                        return IPAddressType::Neither;
                    }
                    if u16::from_str_radix(segment, 16).is_err() {
                        return IPAddressType::Neither;
                    }
                }
                IPAddressType::IPv6
            }
        } else {
            IPAddressType::Neither
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0468_example_1() {
        let query_ip = "172.16.254.1".to_string();
        let result = "IPv4".to_string();

        assert_eq!(Solution::valid_ip_address(query_ip), result);
    }

    #[test]
    fn test_0468_example_2() {
        let query_ip = "2001:0db8:85a3:0:0:8A2E:0370:7334".to_string();
        let result = "IPv6".to_string();

        assert_eq!(Solution::valid_ip_address(query_ip), result);
    }

    #[test]
    fn test_0468_example_3() {
        let query_ip = "256.256.256.256".to_string();
        let result = "Neither".to_string();

        assert_eq!(Solution::valid_ip_address(query_ip), result);
    }
}
