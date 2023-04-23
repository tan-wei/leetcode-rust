/**
 * [1043] Partition Array for Maximum Sum
 *
 * Given an integer array arr, partition the array into (contiguous) subarrays of length at most k. After partitioning, each subarray has their values changed to become the maximum value of that subarray.
 * Return the largest sum of the given array after partitioning. Test cases are generated so that the answer fits in a 32-bit integer.
 *  
 * Example 1:
 *
 * Input: arr = [1,15,7,9,2,5,10], k = 3
 * Output: 84
 * Explanation: arr becomes [15,15,15,9,10,10,10]
 *
 * Example 2:
 *
 * Input: arr = [1,4,1,5,7,3,6,1,9,9,3], k = 4
 * Output: 83
 *
 * Example 3:
 *
 * Input: arr = [1], k = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 500
 * 	0 <= arr[i] <= 10^9
 * 	1 <= k <= arr.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-array-for-maximum-sum/
// discuss: https://leetcode.com/problems/partition-array-for-maximum-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = arr.len();
        let mut dp = vec![0; n];
        dp[0] = arr[0];
        let mut cur_max = arr[0];

        for i in 0..k {
            cur_max = std::cmp::max(cur_max, arr[i]);
            dp[i] = cur_max * (i as i32 + 1);
        }

        for i in k..n {
            cur_max = 0;
            for j in 1..k + 1 {
                cur_max = std::cmp::max(cur_max, arr[i + 1 - j]);
                dp[i] = std::cmp::max(dp[i], dp[i - j] + cur_max * (j as i32));
            }
        }
        dp[n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1043_example_1() {
        let arr = vec![1, 15, 7, 9, 2, 5, 10];
        let k = 3;
        let result = 84;

        assert_eq!(Solution::max_sum_after_partitioning(arr, k), result);
    }

    #[test]
    fn test_1043_example_2() {
        let arr = vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3];
        let k = 4;
        let result = 83;

        assert_eq!(Solution::max_sum_after_partitioning(arr, k), result);
    }

    #[test]
    fn test_1043_example_3() {
        let arr = vec![1];
        let k = 1;
        let result = 1;

        assert_eq!(Solution::max_sum_after_partitioning(arr, k), result);
    }
}
