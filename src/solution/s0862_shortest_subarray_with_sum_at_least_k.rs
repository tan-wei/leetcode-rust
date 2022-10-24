/**
 * [0862] Shortest Subarray with Sum at Least K
 *
 * Given an integer array nums and an integer k, return the length of the shortest non-empty subarray of nums with a sum of at least k. If there is no such subarray, return -1.
 * A subarray is a contiguous part of an array.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [1], k = 1
 * Output: 1
 * <strong class="example">Example 2:
 * Input: nums = [1,2], k = 4
 * Output: -1
 * <strong class="example">Example 3:
 * Input: nums = [2,-1,2], k = 3
 * Output: 3
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-10^5 <= nums[i] <= 10^5
 * 	1 <= k <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/
// discuss: https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/solutions/768261/rust-translated-16ms/
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut result = n as i32 + 1;
        let mut acc = vec![0; n + 1];
        for i in 0..n {
            acc[i + 1] = acc[i] + nums[i]
        }
        let mut q = std::collections::VecDeque::<usize>::new();
        for i in 0..n + 1 {
            while !q.is_empty() && acc[i] - acc[*q.front().unwrap()] >= k {
                result = std::cmp::min(result, (i - q.pop_front().unwrap()) as i32)
            }
            while !q.is_empty() && acc[i] <= acc[*q.back().unwrap()] {
                q.pop_back();
            }
            q.push_back(i);
        }
        if result <= n as i32 {
            result
        } else {
            -1
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0862_example_1() {
        let nums = vec![1];
        let k = 1;
        let result = 1;

        assert_eq!(Solution::shortest_subarray(nums, k), result);
    }

    #[test]
    fn test_0862_example_2() {
        let nums = vec![1, 2];
        let k = 4;
        let result = -1;

        assert_eq!(Solution::shortest_subarray(nums, k), result);
    }

    #[test]
    fn test_0862_example_3() {
        let nums = vec![2, -1, 2];
        let k = 3;
        let result = 3;

        assert_eq!(Solution::shortest_subarray(nums, k), result);
    }
}
