/**
 * [2595] Number of Even and Odd Bits
 *
 * You are given a positive integer n.
 * Let even denote the number of even indices in the binary representation of n with value 1.
 * Let odd denote the number of odd indices in the binary representation of n with value 1.
 * Note that bits are indexed from right to left in the binary representation of a number.
 * Return the array [even, odd].
 *  
 * Example 1:
 *
 * Input: n = 50
 * Output: [1,2]
 * Explanation:
 * The binary representation of 50 is 110010.
 * It contains 1 on indices 1, 4, and 5.
 *
 * Example 2:
 *
 * Input: n = 2
 * Output: [0,1]
 * Explanation:
 * The binary representation of 2 is 10.
 * It contains 1 only on index 1.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-even-and-odd-bits/
// discuss: https://leetcode.com/problems/number-of-even-and-odd-bits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        vec![
            (n & 0x5555).count_ones() as _,
            (n & 0xAAAA).count_ones() as _,
        ]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2595_example_1() {
        let n = 50;

        let result = vec![1, 2];

        assert_eq!(Solution::even_odd_bit(n), result);
    }

    #[test]
    fn test_2595_example_2() {
        let n = 2;

        let result = vec![0, 1];

        assert_eq!(Solution::even_odd_bit(n), result);
    }
}
