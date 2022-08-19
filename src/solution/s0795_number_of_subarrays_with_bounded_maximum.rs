/**
 * [0795] Number of Subarrays with Bounded Maximum
 *
 * Given an integer array nums and two integers left and right, return the number of contiguous non-empty subarrays such that the value of the maximum array element in that subarray is in the range [left, right].
 * The test cases are generated so that the answer will fit in a 32-bit integer.
 *  
 * Example 1:
 *
 * Input: nums = [2,1,4,3], left = 2, right = 3
 * Output: 3
 * Explanation: There are three subarrays that meet the requirements: [2], [2, 1], [3].
 *
 * Example 2:
 *
 * Input: nums = [2,9,2,5,6], left = 2, right = 8
 * Output: 7
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^9
 * 	0 <= left <= right <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-subarrays-with-bounded-maximum/
// discuss: https://leetcode.com/problems/number-of-subarrays-with-bounded-maximum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        (0..nums.len())
            .fold((0, -1, -1), |(count, a, b), i| {
                if nums[i] >= left && nums[i] <= right {
                    (count + i as i32 - b, i as i32, b)
                } else if nums[i] < left {
                    (count + (a - b).max(0), a, b)
                } else {
                    (count, a, i as i32)
                }
            })
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0795_example_1() {
        let nums = vec![2, 1, 4, 3];
        let left = 2;
        let right = 3;
        let result = 3;

        assert_eq!(
            Solution::num_subarray_bounded_max(nums, left, right),
            result
        );
    }

    #[test]
    fn test_0795_example_2() {
        let nums = vec![2, 9, 2, 5, 6];
        let left = 2;
        let right = 8;
        let result = 7;

        assert_eq!(
            Solution::num_subarray_bounded_max(nums, left, right),
            result
        );
    }
}
