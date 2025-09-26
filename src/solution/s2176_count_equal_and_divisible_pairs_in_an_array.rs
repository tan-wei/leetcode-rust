/**
 * [2176] Count Equal and Divisible Pairs in an Array
 *
 * Given a 0-indexed integer array nums of length n and an integer k, return the number of pairs (i, j) where 0 <= i < j < n, such that nums[i] == nums[j] and (i * j) is divisible by k.
 *  
 * Example 1:
 *
 * Input: nums = [3,1,2,2,2,1,3], k = 2
 * Output: 4
 * Explanation:
 * There are 4 pairs that meet all the requirements:
 * - nums[0] == nums[6], and 0 * 6 == 0, which is divisible by 2.
 * - nums[2] == nums[3], and 2 * 3 == 6, which is divisible by 2.
 * - nums[2] == nums[4], and 2 * 4 == 8, which is divisible by 2.
 * - nums[3] == nums[4], and 3 * 4 == 12, which is divisible by 2.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4], k = 1
 * Output: 0
 * Explanation: Since no value in nums is repeated, there are no pairs (i,j) that meet all the requirements.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	1 <= nums[i], k <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array/
// discuss: https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        (0..nums.len())
            .flat_map(|i| (i + 1..nums.len()).map(move |j| (i, j)))
            .filter(|&(i, j)| nums[i] == nums[j] && (i * j) % k as usize == 0)
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2176_example_1() {
        let nums = vec![3, 1, 2, 2, 2, 1, 3];
        let k = 2;

        let result = 4;

        assert_eq!(Solution::count_pairs(nums, k), result);
    }

    #[test]
    fn test_2176_example_2() {
        let nums = vec![1, 2, 3, 4];
        let k = 1;

        let result = 0;

        assert_eq!(Solution::count_pairs(nums, k), result);
    }
}
