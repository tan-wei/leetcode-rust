/**
 * [0476] Number Complement
 *
 * The complement of an integer is the integer you get when you flip all the 0's to 1's and all the 1's to 0's in its binary representation.
 *
 * 	For example, The integer 5 is "101" in binary and its complement is "010" which is the integer 2.
 *
 * Given an integer num, return its complement.
 *  
 * Example 1:
 *
 * Input: num = 5
 * Output: 2
 * Explanation: The binary representation of 5 is 101 (no leading zero bits), and its complement is 010. So you need to output 2.
 *
 * Example 2:
 *
 * Input: num = 1
 * Output: 0
 * Explanation: The binary representation of 1 is 1 (no leading zero bits), and its complement is 0. So you need to output 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= num < 2^31
 *
 *  
 * Note: This question is the same as 1009: <a href="https://leetcode.com/problems/complement-of-base-10-integer/" target="_blank">https://leetcode.com/problems/complement-of-base-10-integer/</a>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-complement/
// discuss: https://leetcode.com/problems/number-complement/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut mask = !0;
        while num & mask != 0 {
            mask <<= 1;
        }

        !mask & !num
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0476_example_1() {
        let num = 5;
        let result = 2;

        assert_eq!(Solution::find_complement(num), result);
    }

    #[test]
    fn test_0476_example_2() {
        let num = 1;
        let result = 0;

        assert_eq!(Solution::find_complement(num), result);
    }
}
