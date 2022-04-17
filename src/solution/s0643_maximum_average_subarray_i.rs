/**
 * [0643] Maximum Average Subarray I
 *
 * You are given an integer array nums consisting of n elements, and an integer k.
 * Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value. Any answer with a calculation error less than 10^-5 will be accepted.
 *  
 * Example 1:
 *
 * Input: nums = [1,12,-5,-6,50,3], k = 4
 * Output: 12.75000
 * Explanation: Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
 *
 * Example 2:
 *
 * Input: nums = [5], k = 1
 * Output: 5.00000
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= k <= n <= 10^5
 * 	-10^4 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-average-subarray-i/
// discuss: https://leetcode.com/problems/maximum-average-subarray-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let average = nums
            .into_iter()
            .map(|x| x as f64 / k as f64)
            .collect::<Vec<f64>>();
        let mut max = average.clone()[0..k as usize]
            .into_iter()
            .fold(0., |a, &b| a + b);
        let mut sum = max;
        for i in k as usize..average.len() {
            sum = sum - average[i - k as usize] + average[i];
            if sum > max {
                max = sum;
            }
        }
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0643_example_1() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;
        let result = 12.75000;

        assert_eq!(Solution::find_max_average(nums, k), result);
    }

    #[test]
    fn test_0643_example_2() {
        let nums = vec![5];
        let k = 1;
        let result = 5.00000;

        assert_eq!(Solution::find_max_average(nums, k), result);
    }
}
