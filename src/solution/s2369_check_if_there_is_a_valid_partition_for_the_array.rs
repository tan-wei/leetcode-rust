/**
 * [2369] Check if There is a Valid Partition For The Array
 *
 * You are given a 0-indexed integer array nums. You have to partition the array into one or more contiguous subarrays.
 * We call a partition of the array valid if each of the obtained subarrays satisfies one of the following conditions:
 * <ol>
 * 	The subarray consists of exactly 2, equal elements. For example, the subarray [2,2] is good.
 * 	The subarray consists of exactly 3, equal elements. For example, the subarray [4,4,4] is good.
 * 	The subarray consists of exactly 3 consecutive increasing elements, that is, the difference between adjacent elements is 1. For example, the subarray [3,4,5] is good, but the subarray [1,3,5] is not.
 * </ol>
 * Return true if the array has at least one valid partition. Otherwise, return false.
 *  
 * Example 1:
 *
 * Input: nums = [4,4,4,5,6]
 * Output: true
 * Explanation: The array can be partitioned into the subarrays [4,4] and [4,5,6].
 * This partition is valid, so we return true.
 *
 * Example 2:
 *
 * Input: nums = [1,1,1,2]
 * Output: false
 * Explanation: There is no valid partition for this array.
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array/
// discuss: https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2369_example_1() {
        let nums = vec![4, 4, 4, 5, 6];

        let result = true;

        assert_eq!(Solution::valid_partition(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2369_example_2() {
        let nums = vec![1, 1, 1, 2];

        let result = false;

        assert_eq!(Solution::valid_partition(nums), result);
    }
}
