/**
 * [2275] Largest Combination With Bitwise AND Greater Than Zero
 *
 * The bitwise AND of an array nums is the bitwise AND of all integers in nums.
 *
 * 	For example, for nums = [1, 5, 3], the bitwise AND is equal to 1 &amp; 5 &amp; 3 = 1.
 * 	Also, for nums = [7], the bitwise AND is 7.
 *
 * You are given an array of positive integers candidates. Compute the bitwise AND for all possible combinations of elements in the candidates array.
 * Return the size of the largest combination of candidates with a bitwise AND greater than 0.
 *  
 * Example 1:
 *
 * Input: candidates = [16,17,71,62,12,24,14]
 * Output: 4
 * Explanation: The combination [16,17,62,24] has a bitwise AND of 16 &amp; 17 &amp; 62 &amp; 24 = 16 > 0.
 * The size of the combination is 4.
 * It can be shown that no combination with a size greater than 4 has a bitwise AND greater than 0.
 * Note that more than one combination may have the largest size.
 * For example, the combination [62,12,24,14] has a bitwise AND of 62 &amp; 12 &amp; 24 &amp; 14 = 8 > 0.
 *
 * Example 2:
 *
 * Input: candidates = [8,8]
 * Output: 2
 * Explanation: The largest combination [8,8] has a bitwise AND of 8 &amp; 8 = 8 > 0.
 * The size of the combination is 2, so we return 2.
 *
 *  
 * Constraints:
 *
 * 	1 <= candidates.length <= 10^5
 * 	1 <= candidates[i] <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/
// discuss: https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2275_example_1() {
        let candidates = vec![16, 17, 71, 62, 12, 24, 14];

        let result = 4;

        assert_eq!(Solution::largest_combination(candidates), result);
    }

    #[test]
    #[ignore]
    fn test_2275_example_2() {
        let candidates = vec![8, 8];

        let result = 2;

        assert_eq!(Solution::largest_combination(candidates), result);
    }
}
