/**
 * [1508] Range Sum of Sorted Subarray Sums
 *
 * You are given the array nums consisting of n positive integers. You computed the sum of all non-empty continuous subarrays from the array and then sorted them in non-decreasing order, creating a new array of n * (n + 1) / 2 numbers.
 * Return the sum of the numbers from index left to index right (indexed from 1), inclusive, in the new array. Since the answer can be a huge number return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4], n = 4, left = 1, right = 5
 * Output: 13
 * Explanation: All subarray sums are 1, 3, 6, 10, 2, 5, 9, 3, 7, 4. After sorting them in non-decreasing order we have the new array [1, 2, 3, 3, 4, 5, 6, 7, 9, 10]. The sum of the numbers from index le = 1 to ri = 5 is 1 + 2 + 3 + 3 + 4 = 13.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4], n = 4, left = 3, right = 4
 * Output: 6
 * Explanation: The given array is the same as example 1. We have the new array [1, 2, 3, 3, 4, 5, 6, 7, 9, 10]. The sum of the numbers from index le = 3 to ri = 4 is 3 + 3 = 6.
 *
 * Example 3:
 *
 * Input: nums = [1,2,3,4], n = 4, left = 1, right = 10
 * Output: 50
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i] <= 100
 * 	1 <= left <= right <= n * (n + 1) / 2
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/range-sum-of-sorted-subarray-sums/
// discuss: https://leetcode.com/problems/range-sum-of-sorted-subarray-sums/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let n = n as usize;
        let left = left as usize;
        let right = right as usize;
        let mut sums = Vec::with_capacity(n * (n + 1) / 2);
        for i in 0..n {
            let mut sum = 0;
            for &num in nums.iter().take(n).skip(i) {
                sum += num;
                sums.push(sum);
            }
        }

        sums.sort_unstable();

        let mut result = 0;
        for &sum in sums.iter().take(right).skip(left - 1) {
            result += sum;
            result %= 1_000_000_007;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1508_example_1() {
        let nums = vec![1, 2, 3, 4];
        let n = 4;
        let left = 1;
        let right = 5;

        let result = 13;

        assert_eq!(Solution::range_sum(nums, n, left, right), result);
    }

    #[test]
    fn test_1508_example_2() {
        let nums = vec![1, 2, 3, 4];
        let n = 4;
        let left = 3;
        let right = 4;

        let result = 6;

        assert_eq!(Solution::range_sum(nums, n, left, right), result);
    }

    #[test]
    fn test_1508_example_3() {
        let nums = vec![1, 2, 3, 4];
        let n = 4;
        let left = 1;
        let right = 10;

        let result = 50;

        assert_eq!(Solution::range_sum(nums, n, left, right), result);
    }
}
