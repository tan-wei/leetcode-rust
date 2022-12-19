/**
 * [0918] Maximum Sum Circular Subarray
 *
 * Given a circular integer array nums of length n, return the maximum possible sum of a non-empty subarray of nums.
 * A circular array means the end of the array connects to the beginning of the array. Formally, the next element of nums[i] is nums[(i + 1) % n] and the previous element of nums[i] is nums[(i - 1 + n) % n].
 * A subarray may only include each element of the fixed buffer nums at most once. Formally, for a subarray nums[i], nums[i + 1], ..., nums[j], there does not exist i <= k1, k2 <= j with k1 % n == k2 % n.
 *  
 * Example 1:
 *
 * Input: nums = [1,-2,3,-2]
 * Output: 3
 * Explanation: Subarray [3] has maximum sum 3.
 *
 * Example 2:
 *
 * Input: nums = [5,-3,5]
 * Output: 10
 * Explanation: Subarray [5,5] has maximum sum 5 + 5 = 10.
 *
 * Example 3:
 *
 * Input: nums = [-3,-2,-3]
 * Output: -2
 * Explanation: Subarray [-2] has maximum sum -2.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 3 * 10^4
 * 	-3 * 10^4 <= nums[i] <= 3 * 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-sum-circular-subarray/
// discuss: https://leetcode.com/problems/maximum-sum-circular-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut min_curr, mut max_curr, mut min, mut max, mut sum) = (0, 0, i32::MAX, i32::MIN, 0);
        for &num in nums.iter() {
            min_curr = num.min(min_curr + num);
            min = min.min(min_curr);
            max_curr = num.max(max_curr + num);
            max = max.max(max_curr);
            sum += num;
        }
        if max > 0 {
            max.max(sum - min)
        } else {
            max
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0918_example_1() {
        let nums = vec![1, -2, 3, -2];
        let result = 3;

        assert_eq!(Solution::max_subarray_sum_circular(nums), result);
    }

    #[test]
    fn test_0918_example_2() {
        let nums = vec![5, -3, 5];
        let result = 10;

        assert_eq!(Solution::max_subarray_sum_circular(nums), result);
    }

    #[test]
    fn test_0918_example_3() {
        let nums = vec![-3, -2, -3];
        let result = -2;

        assert_eq!(Solution::max_subarray_sum_circular(nums), result);
    }
}
