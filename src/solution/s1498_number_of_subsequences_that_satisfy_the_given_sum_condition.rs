/**
 * [1498] Number of Subsequences That Satisfy the Given Sum Condition
 *
 * You are given an array of integers nums and an integer target.
 * Return the number of non-empty subsequences of nums such that the sum of the minimum and maximum element on it is less or equal to target. Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: nums = [3,5,6,7], target = 9
 * Output: 4
 * Explanation: There are 4 subsequences that satisfy the condition.
 * [3] -> Min value + max value <= target (3 + 3 <= 9)
 * [3,5] -> (3 + 5 <= 9)
 * [3,5,6] -> (3 + 6 <= 9)
 * [3,6] -> (3 + 6 <= 9)
 *
 * Example 2:
 *
 * Input: nums = [3,3,6,8], target = 10
 * Output: 6
 * Explanation: There are 6 subsequences that satisfy the condition. (nums can have repeated numbers).
 * [3] , [3] , [3,3], [3,6] , [3,6] , [3,3,6]
 *
 * Example 3:
 *
 * Input: nums = [2,3,3,4,6,7], target = 12
 * Output: 61
 * Explanation: There are 63 non-empty subsequences, two of them do not satisfy the condition ([6,7], [7]).
 * Number of valid subsequences (63 - 2 = 61).
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^6
 * 	1 <= target <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/
// discuss: https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let m = 10_i32.pow(9) + 7;
        let powers: Vec<_> = std::iter::successors(Some(1), |prev| Some((prev * 2) % m))
            .take(nums.len() + 1)
            .collect();

        nums.iter()
            .enumerate()
            .scan(nums.len(), |i_end, (i, &n)| {
                while *i_end > i && n + nums[*i_end - 1] > target {
                    *i_end -= 1;
                }
                (*i_end > i).then(|| powers[*i_end - i - 1])
            })
            .reduce(|sum, s| (sum + s) % m)
            .unwrap_or(0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1498_example_1() {
        let nums = vec![3, 5, 6, 7];
        let target = 9;

        let result = 4;

        assert_eq!(Solution::num_subseq(nums, target), result);
    }

    #[test]
    fn test_1498_example_2() {
        let nums = vec![3, 3, 6, 8];
        let target = 10;

        let result = 6;

        assert_eq!(Solution::num_subseq(nums, target), result);
    }

    #[test]
    fn test_1498_example_3() {
        let nums = vec![2, 3, 3, 4, 6, 7];
        let target = 12;

        let result = 61;

        assert_eq!(Solution::num_subseq(nums, target), result);
    }
}
