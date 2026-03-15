/**
 * [2395] Find Subarrays With Equal Sum
 *
 * Given a 0-indexed integer array nums, determine whether there exist two subarrays of length 2 with equal sum. Note that the two subarrays must begin at different indices.
 * Return true if these subarrays exist, and false otherwise.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 *  
 * Example 1:
 *
 * Input: nums = [4,2,4]
 * Output: true
 * Explanation: The subarrays with elements [4,2] and [2,4] have the same sum of 6.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4,5]
 * Output: false
 * Explanation: No two subarrays of size 2 have the same sum.
 *
 * Example 3:
 *
 * Input: nums = [0,0,0]
 * Output: true
 * Explanation: The subarrays [nums[0],nums[1]] and [nums[1],nums[2]] have the same sum of 0.
 * Note that even though the subarrays have the same content, the two subarrays are considered different because they are in different positions in the original array.
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 1000
 * 	-10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-subarrays-with-equal-sum/
// discuss: https://leetcode.com/problems/find-subarrays-with-equal-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        nums.windows(2)
            .map(|win| win[0] + win[1])
            .scan(
                std::collections::HashSet::with_capacity(nums.len()),
                |set, sum| Some(!set.insert(sum)),
            )
            .any(|b| b)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2395_example_1() {
        let nums = vec![4, 2, 4];

        let result = true;

        assert_eq!(Solution::find_subarrays(nums), result);
    }

    #[test]
    fn test_2395_example_2() {
        let nums = vec![1, 2, 3, 4, 5];

        let result = false;

        assert_eq!(Solution::find_subarrays(nums), result);
    }

    #[test]
    fn test_2395_example_3() {
        let nums = vec![0, 0, 0];

        let result = true;

        assert_eq!(Solution::find_subarrays(nums), result);
    }
}
