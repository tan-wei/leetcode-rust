/**
 * [2461] Maximum Sum of Distinct Subarrays With Length K
 *
 * You are given an integer array nums and an integer k. Find the maximum subarray sum of all the subarrays of nums that meet the following conditions:
 *
 * 	The length of the subarray is k, and
 * 	All the elements of the subarray are distinct.
 *
 * Return the maximum subarray sum of all the subarrays that meet the conditions. If no subarray meets the conditions, return 0.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 *  
 * Example 1:
 *
 * Input: nums = [1,5,4,2,9,9,9], k = 3
 * Output: 15
 * Explanation: The subarrays of nums with length 3 are:
 * - [1,5,4] which meets the requirements and has a sum of 10.
 * - [5,4,2] which meets the requirements and has a sum of 11.
 * - [4,2,9] which meets the requirements and has a sum of 15.
 * - [2,9,9] which does not meet the requirements because the element 9 is repeated.
 * - [9,9,9] which does not meet the requirements because the element 9 is repeated.
 * We return 15 because it is the maximum subarray sum of all the subarrays that meet the conditions
 *
 * Example 2:
 *
 * Input: nums = [4,4,4], k = 3
 * Output: 0
 * Explanation: The subarrays of nums with length 3 are:
 * - [4,4,4] which does not meet the requirements because the element 4 is repeated.
 * We return 0 because no subarrays meet the conditions.
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/
// discuss: https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut hash: std::collections::HashMap<i64, usize> = std::collections::HashMap::new();
        let (mut result, mut sum) = (0, 0);
        for (idx, &num) in nums.iter().enumerate() {
            if idx as i32 >= k {
                if hash.len() == k as usize {
                    result = result.max(sum);
                }
                let value = nums[idx - k as usize] as i64;
                sum -= value;
                *hash.entry(value).or_insert(0) -= 1;
                if hash[&value] == 0 {
                    hash.remove(&value);
                }
            }
            *hash.entry(num as i64).or_insert(0) += 1;
            sum += num as i64;
        }
        if hash.len() as i32 == k {
            result = result.max(sum);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2461_example_1() {
        let nums = vec![1, 5, 4, 2, 9, 9, 9];
        let k = 3;

        let result = 15;

        assert_eq!(Solution::maximum_subarray_sum(nums, k), result);
    }

    #[test]
    fn test_2461_example_2() {
        let nums = vec![4, 4, 4];
        let k = 3;

        let result = 0;

        assert_eq!(Solution::maximum_subarray_sum(nums, k), result);
    }
}
