/**
 * [190] Reverse Bits
 *
 * Reverse bits of a given 32 bits unsigned integer.
 * Note:
 *
 * 	Note that in some languages such as Java, there is no unsigned integer type. In this case, both input and output will be given as a signed integer type. They should not affect your implementation, as the integer's internal binary representation is the same, whether it is signed or unsigned.
 * 	In Java, the compiler represents the signed integers using <a href="https://en.wikipedia.org/wiki/Two%27s_complement" target="_blank">2's complement notation</a>. Therefore, in Example 2 above, the input represents the signed integer -3 and the output represents the signed integer -1073741825.
 *
 * Follow up:
 * If this function is called many times, how would you optimize it?
 *  
 * Example 1:
 *
 * Input: n = 00000010100101000001111010011100
 * Output:    964176192 (00111001011110000010100101000000)
 * Explanation: The input binary string 00000010100101000001111010011100 represents the unsigned integer 43261596, so return 964176192 which its binary representation is 00111001011110000010100101000000.
 *
 * Example 2:
 *
 * Input: n = 11111111111111111111111111111101
 * Output:   3221225471 (10111111111111111111111111111111)
 * Explanation: The input binary string 11111111111111111111111111111101 represents the unsigned integer 4294967293, so return 3221225471 which its binary representation is 10111111111111111111111111111111.
 *
 *  
 * Constraints:
 *
 * 	The input must be a binary string of length 32
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-bits/
// discuss: https://leetcode.com/problems/reverse-bits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        x.reverse_bits()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0190_example_1() {
        let x = 43261596;
        let result = 964176192;

        assert_eq!(Solution::reverse_bits(x), result);
    }

    #[test]
    fn test_0190_example_2() {
        let x = 4294967293;
        let result = 3221225471;

        assert_eq!(Solution::reverse_bits(x), result);
    }
}
