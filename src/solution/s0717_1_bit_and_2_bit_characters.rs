/**
 * [0717] 1-bit and 2-bit Characters
 *
 * We have two special characters:
 *
 * 	The first character can be represented by one bit 0.
 * 	The second character can be represented by two bits (10 or 11).
 *
 * Given a binary array bits that ends with 0, return true if the last character must be a one-bit character.
 *  
 * Example 1:
 *
 * Input: bits = [1,0,0]
 * Output: true
 * Explanation: The only way to decode it is two-bit character and one-bit character.
 * So the last character is one-bit character.
 *
 * Example 2:
 *
 * Input: bits = [1,1,1,0]
 * Output: false
 * Explanation: The only way to decode it is two-bit character and two-bit character.
 * So the last character is not one-bit character.
 *
 *  
 * Constraints:
 *
 * 	1 <= bits.length <= 1000
 * 	bits[i] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/1-bit-and-2-bit-characters/
// discuss: https://leetcode.com/problems/1-bit-and-2-bit-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let ones = bits.iter().rev().skip(1).take_while(|&&b| b == 1).count();

        ones % 2 == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0717_example_1() {
        let bits = vec![1, 0, 0];
        let result = true;

        assert_eq!(Solution::is_one_bit_character(bits), result);
    }

    #[test]
    fn test_0717_example_2() {
        let bits = vec![1, 1, 1, 0];
        let result = false;

        assert_eq!(Solution::is_one_bit_character(bits), result);
    }
}
