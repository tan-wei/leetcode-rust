/**
 * [2389] Longest Subsequence With Limited Sum
 *
 * You are given an integer array nums of length n, and an integer array queries of length m.
 * Return an array answer of length m where answer[i] is the maximum size of a subsequence that you can take from nums such that the sum of its elements is less than or equal to queries[i].
 * A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
 *  
 * Example 1:
 *
 * Input: nums = [4,5,2,1], queries = [3,10,21]
 * Output: [2,3,4]
 * Explanation: We answer the queries as follows:
 * - The subsequence [2,1] has a sum less than or equal to 3. It can be proven that 2 is the maximum size of such a subsequence, so answer[0] = 2.
 * - The subsequence [4,5,1] has a sum less than or equal to 10. It can be proven that 3 is the maximum size of such a subsequence, so answer[1] = 3.
 * - The subsequence [4,5,2,1] has a sum less than or equal to 21. It can be proven that 4 is the maximum size of such a subsequence, so answer[2] = 4.
 *
 * Example 2:
 *
 * Input: nums = [2,3,4,5], queries = [1]
 * Output: [0]
 * Explanation: The empty subsequence is the only subsequence that has a sum less than or equal to 1, so answer[0] = 0.
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	m == queries.length
 * 	1 <= n, m <= 1000
 * 	1 <= nums[i], queries[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-subsequence-with-limited-sum/
// discuss: https://leetcode.com/problems/longest-subsequence-with-limited-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        for i in 0..nums.len() - 1 {
            nums[i + 1] += nums[i];
        }
        queries
            .into_iter()
            .map(|x| nums.partition_point(|&y| y <= x) as i32)
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2389_example_1() {
        let nums = vec![4, 5, 2, 1];
        let queries = vec![3, 10, 21];

        let result = vec![2, 3, 4];

        assert_eq!(Solution::answer_queries(nums, queries), result)
    }

    #[test]
    fn test_2389_example_2() {
        let nums = vec![2, 3, 4, 5];
        let queries = vec![1];

        let result = vec![0];

        assert_eq!(Solution::answer_queries(nums, queries), result)
    }
}
