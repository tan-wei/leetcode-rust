/**
 * [137] Single Number II
 *
 * Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it.
 * You must implement a solution with a linear runtime complexity and use only constant extra space.
 *  
 * Example 1:
 * Input: nums = [2,2,3,2]
 * Output: 3
 * Example 2:
 * Input: nums = [0,1,0,1,0,1,99]
 * Output: 99
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 3 * 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	Each element in nums appears exactly three times except for one element which appears once.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/single-number-ii/
// discuss: https://leetcode.com/problems/single-number-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        (0..std::mem::size_of::<i32>() * 8)
            .map(|i| {
                if nums.iter().filter(|n| **n & (1 << i) != 0).count() % 3 as usize > 0 {
                    1 << i
                } else {
                    0
                }
            })
            .sum()
    }

    // Credit: https://leetcode.com/problems/single-number-ii/discuss/43294/Challenge-me-thx
    pub fn single_number_v2(nums: Vec<i32>) -> i32 {
        let mut ones = 0;
        let mut twos = 0;
        for i in 0..nums.len() {
            ones = (ones ^ nums[i]) & !twos;
            twos = (twos ^ nums[i]) & !ones;
        }
        ones
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0137_example_1() {
        let nums = vec![2, 2, 3, 2];
        let result = 3;

        assert_eq!(Solution::single_number(nums), result);

        let nums = vec![2, 2, 3, 2];
        let result = 3;

        assert_eq!(Solution::single_number_v2(nums), result);
    }

    #[test]
    fn test_0137_example_2() {
        let nums = vec![0, 1, 0, 1, 0, 1, 99];
        let result = 99;

        assert_eq!(Solution::single_number(nums), result);

        let nums = vec![0, 1, 0, 1, 0, 1, 99];
        let result = 99;

        assert_eq!(Solution::single_number_v2(nums), result);
    }
}
