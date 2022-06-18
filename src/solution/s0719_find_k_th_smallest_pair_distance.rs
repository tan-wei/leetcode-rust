/**
 * [0719] Find K-th Smallest Pair Distance
 *
 * The distance of a pair of integers a and b is defined as the absolute difference between a and b.
 * Given an integer array nums and an integer k, return the k^th smallest distance among all the pairs nums[i] and nums[j] where 0 <= i < j < nums.length.
 *  
 * Example 1:
 *
 * Input: nums = [1,3,1], k = 1
 * Output: 0
 * Explanation: Here are all the pairs:
 * (1,3) -> 2
 * (1,1) -> 0
 * (3,1) -> 2
 * Then the 1^st smallest distance pair is (1,1), and its distance is 0.
 *
 * Example 2:
 *
 * Input: nums = [1,1,1], k = 2
 * Output: 0
 *
 * Example 3:
 *
 * Input: nums = [1,6,1], k = 3
 * Output: 5
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	2 <= n <= 10^4
 * 	0 <= nums[i] <= 10^6
 * 	1 <= k <= n * (n - 1) / 2
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-k-th-smallest-pair-distance/
// discuss: https://leetcode.com/problems/find-k-th-smallest-pair-distance/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();

        let mut low = 0;
        let mut high = nums[n - 1] - nums[0];

        while low < high {
            let m = (low + high) / 2;
            let mut count = 0;

            let mut i: usize = 0;
            for j in 0..n {
                while nums[j] - nums[i] > m {
                    i += 1;
                }
                count += j - i;
            }

            if count < k as usize {
                low = m + 1;
            } else {
                high = m;
            }
        }

        low
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0719_example_1() {
        let nums = vec![1, 3, 1];
        let k = 1;
        let result = 0;

        assert_eq!(Solution::smallest_distance_pair(nums, k), result);
    }
}
