/**
 * [0896] Monotonic Array
 *
 * An array is monotonic if it is either monotone increasing or monotone decreasing.
 * An array nums is monotone increasing if for all i <= j, nums[i] <= nums[j]. An array nums is monotone decreasing if for all i <= j, nums[i] >= nums[j].
 * Given an integer array nums, return true if the given array is monotonic, or false otherwise.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,2,3]
 * Output: true
 *
 * Example 2:
 *
 * Input: nums = [6,5,4,4]
 * Output: true
 *
 * Example 3:
 *
 * Input: nums = [1,3,2]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-10^5 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/monotonic-array/
// discuss: https://leetcode.com/problems/monotonic-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        nums.iter()
            .zip(nums.iter().skip(1))
            .filter(|&(&a, &b)| a >= b)
            .map(|(_, _)| 1)
            .sum::<usize>()
            == nums.len() - 1
            || nums
                .iter()
                .zip(nums.iter().skip(1))
                .filter(|&(&a, &b)| a <= b)
                .map(|(_, _)| 1)
                .sum::<usize>()
                == nums.len() - 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0896_example_1() {
        let nums = vec![1, 2, 2, 3];
        let result = true;

        assert_eq!(Solution::is_monotonic(nums), result);
    }

    #[test]
    fn test_0896_example_2() {
        let nums = vec![6, 5, 4, 4];
        let result = true;

        assert_eq!(Solution::is_monotonic(nums), result);
    }

    #[test]
    fn test_0896_example_3() {
        let nums = vec![1, 3, 2];
        let result = false;

        assert_eq!(Solution::is_monotonic(nums), result);
    }
}
