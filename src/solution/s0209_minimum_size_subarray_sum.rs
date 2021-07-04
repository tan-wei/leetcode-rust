/**
 * [209] Minimum Size Subarray Sum
 *
 * Given an array of positive integers nums and a positive integer target, return the minimal length of a contiguous subarray [numsl, numsl+1, ..., numsr-1, numsr] of which the sum is greater than or equal to target. If there is no such subarray, return 0 instead.
 *  
 * Example 1:
 *
 * Input: target = 7, nums = [2,3,1,2,4,3]
 * Output: 2
 * Explanation: The subarray [4,3] has the minimal length under the problem constraint.
 *
 * Example 2:
 *
 * Input: target = 4, nums = [1,4,4]
 * Output: 1
 *
 * Example 3:
 *
 * Input: target = 11, nums = [1,1,1,1,1,1,1,1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= target <= 10^9
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^5
 *
 *  
 * Follow up: If you have figured out the O(n) solution, try coding another solution of which the time complexity is O(n log(n)).
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-size-subarray-sum/
// discuss: https://leetcode.com/problems/minimum-size-subarray-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            _ => {
                let (mut t, mut m, mut i, mut j) = (0, i32::MAX, 0, 0);
                while j < nums.len() + 1 {
                    if t < target {
                        t += nums.get(j).unwrap_or(&0);
                        j += 1;
                    } else {
                        m = ((j - i) as i32).min(m);
                        t -= nums[i];
                        i += 1;
                    }
                }
                if m == i32::MAX {
                    0
                } else {
                    m
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
    fn test_0209_example_1() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];
        let result = 2;

        assert_eq!(Solution::min_sub_array_len(target, nums), result);
    }

    #[test]
    fn test_0209_example_2() {
        let target = 4;
        let nums = vec![1, 4, 4];
        let result = 1;

        assert_eq!(Solution::min_sub_array_len(target, nums), result);
    }

    #[test]
    fn test_0209_example_3() {
        let target = 11;
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let result = 0;

        assert_eq!(Solution::min_sub_array_len(target, nums), result);
    }
}
