/**
 * [1696] Jump Game VI
 *
 * You are given a 0-indexed integer array nums and an integer k.
 * You are initially standing at index 0. In one move, you can jump at most k steps forward without going outside the boundaries of the array. That is, you can jump from index i to any index in the range [i + 1, min(n - 1, i + k)] inclusive.
 * You want to reach the last index of the array (index n - 1). Your score is the sum of all nums[j] for each index j you visited in the array.
 * Return the maximum score you can get.
 *  
 * Example 1:
 *
 * Input: nums = [<u>1</u>,<u>-1</u>,-2,<u>4</u>,-7,<u>3</u>], k = 2
 * Output: 7
 * Explanation: You can choose your jumps forming the subsequence [1,-1,4,3] (underlined above). The sum is 7.
 *
 * Example 2:
 *
 * Input: nums = [<u>10</u>,-5,-2,<u>4</u>,0,<u>3</u>], k = 3
 * Output: 17
 * Explanation: You can choose your jumps forming the subsequence [10,4,3] (underlined above). The sum is 17.
 *
 * Example 3:
 *
 * Input: nums = [1,-5,-20,4,-1,3,-6,-3], k = 2
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length, k <= 10^5
 * 	-10^4 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/jump-game-vi/
// discuss: https://leetcode.com/problems/jump-game-vi/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut vd = std::collections::VecDeque::new();

        for i in (0..nums.len()).rev() {
            dp[i] = nums[i] + vd.front().map_or(0, |&f| dp[f]);
            while !vd.is_empty() {
                if let Some(&b) = vd.back() {
                    if dp[i] > dp[b] {
                        vd.pop_back();
                    } else {
                        break;
                    }
                }
            }

            vd.push_back(i);

            if let Some(&f) = vd.front() {
                if f == i + k as usize {
                    vd.pop_front();
                }
            }
        }

        dp[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1696_example_1() {
        let nums = vec![1, -1, -2, 4, -7, 3];
        let k = 2;

        let result = 7;

        assert_eq!(Solution::max_result(nums, k), result);
    }

    #[test]
    fn test_1696_example_2() {
        let nums = vec![10, -5, -2, 4, 0, 3];
        let k = 3;

        let result = 17;

        assert_eq!(Solution::max_result(nums, k), result);
    }

    #[test]
    fn test_1696_example_3() {
        let nums = vec![1, -5, -20, 4, -1, 3, -6, -3];
        let k = 2;

        let result = 0;

        assert_eq!(Solution::max_result(nums, k), result);
    }
}
