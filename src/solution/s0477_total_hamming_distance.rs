/**
 * [0477] Total Hamming Distance
 *
 * The <a href="https://en.wikipedia.org/wiki/Hamming_distance" target="_blank">Hamming distance</a> between two integers is the number of positions at which the corresponding bits are different.
 * Given an integer array nums, return the sum of Hamming distances between all the pairs of the integers in nums.
 *  
 * Example 1:
 *
 * Input: nums = [4,14,2]
 * Output: 6
 * Explanation: In binary representation, the 4 is 0100, 14 is 1110, and 2 is 0010 (just
 * showing the four bits relevant in this case).
 * The answer will be:
 * HammingDistance(4, 14) + HammingDistance(4, 2) + HammingDistance(14, 2) = 2 + 2 + 2 = 6.
 *
 * Example 2:
 *
 * Input: nums = [4,14,4]
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	0 <= nums[i] <= 10^9
 * 	The answer for the given input will fit in a 32-bit integer.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/total-hamming-distance/
// discuss: https://leetcode.com/problems/total-hamming-distance/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for i in 0..30 {
            let mut zeros = 0;
            let mut ones = 0;
            for num in &nums {
                match (1 << i) & num {
                    0 => zeros += 1,
                    _ => ones += 1,
                }
            }
            result += zeros * ones;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0477_example_1() {
        let nums = vec![4, 14, 2];
        let result = 6;

        assert_eq!(Solution::total_hamming_distance(nums), result);
    }

    #[test]
    fn test_0477_example_2() {
        let nums = vec![4, 14, 4];
        let result = 4;

        assert_eq!(Solution::total_hamming_distance(nums), result);
    }
}
