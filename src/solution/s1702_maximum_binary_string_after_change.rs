/**
 * [1702] Maximum Binary String After Change
 *
 * You are given a binary string binary consisting of only 0's or 1's. You can apply each of the following operations any number of times:
 *
 * 	Operation 1: If the number contains the substring "00", you can replace it with "10".
 *
 * 		For example, "<u>00</u>010" -> "<u>10</u>010"
 *
 *
 * 	Operation 2: If the number contains the substring "10", you can replace it with "01".
 *
 * 		For example, "000<u>10</u>" -> "000<u>01</u>"
 *
 *
 *
 * Return the maximum binary string you can obtain after any number of operations. Binary string x is greater than binary string y if x's decimal representation is greater than y's decimal representation.
 *  
 * Example 1:
 *
 * Input: binary = "000110"
 * Output: "111011"
 * Explanation: A valid transformation sequence can be:
 * "0001<u>10</u>" -> "0001<u>01</u>"
 * "<u>00</u>0101" -> "<u>10</u>0101"
 * "1<u>00</u>101" -> "1<u>10</u>101"
 * "110<u>10</u>1" -> "110<u>01</u>1"
 * "11<u>00</u>11" -> "11<u>10</u>11"
 *
 * Example 2:
 *
 * Input: binary = "01"
 * Output: "01"
 * Explanation: "01" cannot be transformed any further.
 *
 *  
 * Constraints:
 *
 * 	1 <= binary.length <= 10^5
 * 	binary consist of '0' and '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-binary-string-after-change/
// discuss: https://leetcode.com/problems/maximum-binary-string-after-change/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let s = binary.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut stack = vec![];
        let mut temp = vec![];

        let mut i = 0;

        while i < n {
            let c = s[i];
            if temp.is_empty() {
                if c == '1' {
                    stack.push(c);
                } else {
                    temp.push(c);
                }
            } else {
                if c == '1' {
                    temp.push('1');
                } else {
                    stack.push('1');
                }
            }
            i += 1;
        }

        for c in temp {
            stack.push(c);
        }

        stack.into_iter().map(|v| v.to_string()).collect::<_>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1702_example_1() {
        let binary = "000110".to_string();

        let result = "111011".to_string();

        assert_eq!(Solution::maximum_binary_string(binary), result);
    }

    #[test]
    fn test_1702_example_2() {
        let binary = "01".to_string();

        let result = "01".to_string();

        assert_eq!(Solution::maximum_binary_string(binary), result);
    }
}
