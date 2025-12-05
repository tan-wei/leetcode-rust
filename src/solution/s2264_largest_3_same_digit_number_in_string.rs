/**
 * [2264] Largest 3-Same-Digit Number in String
 *
 * You are given a string num representing a large integer. An integer is good if it meets the following conditions:
 *
 * 	It is a substring of num with length 3.
 * 	It consists of only one unique digit.
 *
 * Return the maximum good integer as a string or an empty string "" if no such integer exists.
 * Note:
 *
 * 	A substring is a contiguous sequence of characters within a string.
 * 	There may be leading zeroes in num or a good integer.
 *
 *  
 * Example 1:
 *
 * Input: num = "6<u>777</u>133339"
 * Output: "777"
 * Explanation: There are two distinct good integers: "777" and "333".
 * "777" is the largest, so we return "777".
 *
 * Example 2:
 *
 * Input: num = "23<u>000</u>19"
 * Output: "000"
 * Explanation: "000" is the only good integer.
 *
 * Example 3:
 *
 * Input: num = "42352338"
 * Output: ""
 * Explanation: No substring of length 3 consists of only one unique digit. Therefore, there are no good integers.
 *
 *  
 * Constraints:
 *
 * 	3 <= num.length <= 1000
 * 	num only consists of digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-3-same-digit-number-in-string/
// discuss: https://leetcode.com/problems/largest-3-same-digit-number-in-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        num.as_bytes()
            .windows(3)
            .filter(|bytes| bytes.iter().all(|&b| b == bytes[0])) //[[55,55,55]], [57,57,57]]
            .map(|b| b[0])
            .max()
            .map(|b| (b as char).to_string().repeat(3))
            .unwrap_or_default() //"999"
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2264_example_1() {
        let num = "6777133339".to_string();

        let result = "777".to_string();

        assert_eq!(Solution::largest_good_integer(num), result);
    }

    #[test]
    fn test_2264_example_2() {
        let num = "2300019".to_string();

        let result = "000".to_string();

        assert_eq!(Solution::largest_good_integer(num), result);
    }

    #[test]
    fn test_2264_example_3() {
        let num = "42352338".to_string();

        let result = "".to_string();

        assert_eq!(Solution::largest_good_integer(num), result);
    }
}
