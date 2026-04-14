/**
 * [2429] Minimize XOR
 *
 * Given two positive integers num1 and num2, find the positive integer x such that:
 *
 * 	x has the same number of set bits as num2, and
 * 	The value x XOR num1 is minimal.
 *
 * Note that XOR is the bitwise XOR operation.
 * Return the integer x. The test cases are generated such that x is uniquely determined.
 * The number of set bits of an integer is the number of 1's in its binary representation.
 *  
 * Example 1:
 *
 * Input: num1 = 3, num2 = 5
 * Output: 3
 * Explanation:
 * The binary representations of num1 and num2 are 0011 and 0101, respectively.
 * The integer 3 has the same number of set bits as num2, and the value 3 XOR 3 = 0 is minimal.
 *
 * Example 2:
 *
 * Input: num1 = 1, num2 = 12
 * Output: 3
 * Explanation:
 * The binary representations of num1 and num2 are 0001 and 1100, respectively.
 * The integer 3 has the same number of set bits as num2, and the value 3 XOR 1 = 2 is minimal.
 *
 *  
 * Constraints:
 *
 * 	1 <= num1, num2 <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimize-xor/
// discuss: https://leetcode.com/problems/minimize-xor/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        (0..30)
            .rev()
            .map(|i| num1 & (1 << i))
            .chain((0..30).map(|i| !num1 & (1 << i)))
            .filter(|&x| x != 0)
            .take(num2.count_ones() as usize)
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2429_example_1() {
        let num1 = 3;
        let num2 = 5;

        let result = 3;

        assert_eq!(Solution::minimize_xor(num1, num2), result);
    }

    #[test]
    fn test_2429_example_2() {
        let num1 = 1;
        let num2 = 12;

        let result = 3;

        assert_eq!(Solution::minimize_xor(num1, num2), result);
    }
}
