/**
 * [1755] Closest Subsequence Sum
 *
 * You are given an integer array nums and an integer goal.
 * You want to choose a subsequence of nums such that the sum of its elements is the closest possible to goal. That is, if the sum of the subsequence's elements is sum, then you want to minimize the absolute difference abs(sum - goal).
 * Return the minimum possible value of abs(sum - goal).
 * Note that a subsequence of an array is an array formed by removing some elements (possibly all or none) of the original array.
 *  
 * Example 1:
 *
 * Input: nums = [5,-7,3,5], goal = 6
 * Output: 0
 * Explanation: Choose the whole array as a subsequence, with a sum of 6.
 * This is equal to the goal, so the absolute difference is 0.
 *
 * Example 2:
 *
 * Input: nums = [7,-9,15,-2], goal = -5
 * Output: 1
 * Explanation: Choose the subsequence [7,-9,-2], with a sum of -4.
 * The absolute difference is abs(-4 - (-5)) = abs(1) = 1, which is the minimum.
 *
 * Example 3:
 *
 * Input: nums = [1,2,3], goal = -7
 * Output: 7
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 40
 * 	-10^7 <= nums[i] <= 10^7
 * 	-10^9 <= goal <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/closest-subsequence-sum/
// discuss: https://leetcode.com/problems/closest-subsequence-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/closest-subsequence-sum/solutions/5557655/rust-filter-out-out-of-bound-values/
    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        let mut nums = nums;

        let mut pos_sum = nums.iter().filter(|&&x| x > 0).sum::<i32>();
        let mut neg_sum = nums.iter().filter(|&&x| x < 0).sum::<i32>();

        nums.sort_unstable_by_key(|&x| -x.abs());

        let mut v = vec![-goal];
        let mut v2 = Vec::<i32>::new();
        let mut result = goal.abs();

        for &x in nums.iter() {
            if x > 0 {
                pos_sum -= x;
            } else {
                neg_sum -= x;
            }
            for y in v.drain(..).flat_map(|val| [val, val + x]) {
                result = y.abs().min(result);
                if y >= -neg_sum {
                    result = result.min(y + neg_sum);
                } else if -y >= pos_sum {
                    result = result.min(-y - pos_sum);
                } else {
                    v2.push(y);
                }
                if result == 0 {
                    return 0;
                }
            }
            std::mem::swap(&mut v, &mut v2);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1755_example_1() {
        let nums = vec![5, -7, 3, 5];
        let goal = 6;

        let result = 0;

        assert_eq!(Solution::min_abs_difference(nums, goal), result);
    }

    #[test]
    fn test_1755_example_2() {
        let nums = vec![7, -9, 15, -2];
        let goal = -5;

        let result = 1;

        assert_eq!(Solution::min_abs_difference(nums, goal), result);
    }

    #[test]
    fn test_1755_example_3() {
        let nums = vec![1, 2, 3];
        let goal = -7;

        let result = 7;

        assert_eq!(Solution::min_abs_difference(nums, goal), result);
    }
}
