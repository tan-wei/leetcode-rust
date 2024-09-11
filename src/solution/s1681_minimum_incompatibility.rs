/**
 * [1681] Minimum Incompatibility
 *
 * You are given an integer array nums​​​ and an integer k. You are asked to distribute this array into k subsets of equal size such that there are no two equal elements in the same subset.
 * A subset's incompatibility is the difference between the maximum and minimum elements in that array.
 * Return the minimum possible sum of incompatibilities of the k subsets after distributing the array optimally, or return -1 if it is not possible.
 * A subset is a group integers that appear in the array with no particular order.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,1,4], k = 2
 * Output: 4
 * Explanation: The optimal distribution of subsets is [1,2] and [1,4].
 * The incompatibility is (2-1) + (4-1) = 4.
 * Note that [1,1] and [2,4] would result in a smaller sum, but the first subset contains 2 equal elements.
 * Example 2:
 *
 * Input: nums = [6,3,8,1,3,1,2,2], k = 4
 * Output: 6
 * Explanation: The optimal distribution of subsets is [1,2], [2,3], [6,8], and [1,3].
 * The incompatibility is (2-1) + (3-2) + (8-6) + (3-1) = 6.
 *
 * Example 3:
 *
 * Input: nums = [5,3,3,6,3,3], k = 3
 * Output: -1
 * Explanation: It is impossible to distribute nums into 3 subsets where no two elements are equal in the same subset.
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= nums.length <= 16
 * 	nums.length is divisible by k
 * 	1 <= nums[i] <= nums.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-incompatibility/
// discuss: https://leetcode.com/problems/minimum-incompatibility/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-incompatibility/solutions/1050356/rust-solution-4ms/
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let k = k as usize;
        let mut buckets =
            vec![std::collections::VecDeque::with_capacity(nums.len() / k as usize); k as usize];
        let mut result = std::i32::MAX;

        Self::dfs_helper(
            0,
            &nums,
            &mut buckets,
            0,
            nums.len() / k as usize,
            0,
            &mut result,
        );

        if result == std::i32::MAX {
            -1
        } else {
            result
        }
    }

    fn dfs_helper(
        index: usize,
        nums: &Vec<i32>,
        buckets: &mut Vec<std::collections::VecDeque<i32>>,
        k: usize,
        bucket_len: usize,
        total: i32,
        result: &mut i32,
    ) {
        if index == nums.len() {
            *result = total;
            return;
        }

        for i in 0..buckets.len().min(k + 1) {
            if buckets[i].len() < bucket_len
                && (buckets[i].is_empty() || *buckets[i].back().unwrap() < nums[index])
            {
                let cur = nums[index] - *buckets[i].back().unwrap_or(&nums[index]) + total;
                if cur < *result {
                    buckets[i].push_back(nums[index]);
                    Self::dfs_helper(
                        index + 1,
                        nums,
                        buckets,
                        k.max(i + 1),
                        bucket_len,
                        cur,
                        result,
                    );
                    buckets[i].pop_back();
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1681_example_1() {
        let nums = vec![1, 2, 1, 4];
        let k = 2;

        let result = 4;

        assert_eq!(Solution::minimum_incompatibility(nums, k), result);
    }

    #[test]
    fn test_1681_example_2() {
        let nums = vec![6, 3, 8, 1, 3, 1, 2, 2];
        let k = 4;

        let result = 6;

        assert_eq!(Solution::minimum_incompatibility(nums, k), result);
    }

    #[test]
    fn test_1681_example_3() {
        let nums = vec![5, 3, 3, 6, 3, 3];
        let k = 3;

        let result = -1;

        assert_eq!(Solution::minimum_incompatibility(nums, k), result);
    }
}
