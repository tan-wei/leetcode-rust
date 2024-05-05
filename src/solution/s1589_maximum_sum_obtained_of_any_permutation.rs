/**
 * [1589] Maximum Sum Obtained of Any Permutation
 *
 * We have an array of integers, nums, and an array of requests where requests[i] = [starti, endi]. The i^th request asks for the sum of nums[starti] + nums[starti + 1] + ... + nums[endi - 1] + nums[endi]. Both starti and endi are 0-indexed.
 * Return the maximum total sum of all requests among all permutations of nums.
 * Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4,5], requests = [[1,3],[0,1]]
 * Output: 19
 * Explanation: One permutation of nums is [2,1,3,4,5] with the following result:
 * requests[0] -> nums[1] + nums[2] + nums[3] = 1 + 3 + 4 = 8
 * requests[1] -> nums[0] + nums[1] = 2 + 1 = 3
 * Total sum: 8 + 3 = 11.
 * A permutation with a higher total sum is [3,5,4,2,1] with the following result:
 * requests[0] -> nums[1] + nums[2] + nums[3] = 5 + 4 + 2 = 11
 * requests[1] -> nums[0] + nums[1] = 3 + 5  = 8
 * Total sum: 11 + 8 = 19, which is the best that you can do.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4,5,6], requests = [[0,1]]
 * Output: 11
 * Explanation: A permutation with the max total sum is [6,5,4,3,2,1] with request sums [11].
 * Example 3:
 *
 * Input: nums = [1,2,3,4,5,10], requests = [[0,2],[1,3],[1,1]]
 * Output: 47
 * Explanation: A permutation with the max total sum is [4,10,5,3,2,1] with request sums [19,18,10].
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 10^5
 * 	0 <= nums[i] <= 10^5
 * 	1 <= requests.length <= 10^5
 * 	requests[i].length == 2
 * 	0 <= starti <= endi < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-sum-obtained-of-any-permutation/
// discuss: https://leetcode.com/problems/maximum-sum-obtained-of-any-permutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: usize = 1_000_000_007;

impl Solution {
    pub fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut memo = vec![(0, 0); n];

        for arr in requests {
            memo[arr[0] as usize].0 += 1;
            memo[arr[1] as usize].1 += 1;
        }

        let mut counts = vec![0; n];
        let mut temp = 0;
        for i in 0..n {
            temp += memo[i].0;
            counts[i] = temp;
            temp -= memo[i].1;
        }

        counts.sort_unstable();
        let mut nums = nums;
        nums.sort_unstable();

        let mut result = 0;

        for i in 0..n {
            result += counts[i] as usize * nums[i] as usize % MOD;
            result %= MOD;
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1589_example_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let requests = vec![vec![1, 3], vec![0, 1]];

        let result = 19;

        assert_eq!(Solution::max_sum_range_query(nums, requests), result);
    }

    #[test]
    fn test_1589_example_2() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let requests = vec![vec![0, 1]];

        let result = 11;

        assert_eq!(Solution::max_sum_range_query(nums, requests), result);
    }

    #[test]
    fn test_1589_example_3() {
        let nums = vec![1, 2, 3, 4, 5, 10];
        let requests = vec![vec![0, 2], vec![1, 3], vec![1, 1]];

        let result = 47;

        assert_eq!(Solution::max_sum_range_query(nums, requests), result);
    }
}
