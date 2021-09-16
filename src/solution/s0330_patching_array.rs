/**
 * [330] Patching Array
 *
 * Given a sorted integer array nums and an integer n, add/patch elements to the array such that any number in the range [1, n] inclusive can be formed by the sum of some elements in the array.
 * Return the minimum number of patches required.
 *  
 * Example 1:
 *
 * Input: nums = [1,3], n = 6
 * Output: 1
 * Explanation:
 * Combinations of nums are [1], [3], [1,3], which form possible sums of: 1, 3, 4.
 * Now if we add/patch 2 to nums, the combinations are: [1], [2], [3], [1,3], [2,3], [1,2,3].
 * Possible sums are 1, 2, 3, 4, 5, 6, which now covers the range [1, 6].
 * So we only need 1 patch.
 *
 * Example 2:
 *
 * Input: nums = [1,5,10], n = 20
 * Output: 2
 * Explanation: The two patches can be [2, 4].
 *
 * Example 3:
 *
 * Input: nums = [1,2,2], n = 5
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i] <= 10^4
 * 	nums is sorted in ascending order.
 * 	1 <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/patching-array/
// discuss: https://leetcode.com/problems/patching-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let n = n as u64;
        let mut miss = 1;
        let mut patches = 0;
        let mut i = 0;

        while miss <= n {
            if i < nums.len() && (nums[i] as u64) <= miss {
                miss += nums[i] as u64;
                i += 1;
            } else {
                miss += miss;
                patches += 1;
            }
        }

        patches
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0330_example_1() {
        let nums = vec![1, 3];
        let n = 6;
        let result = 1;

        assert_eq!(Solution::min_patches(nums, n), result);
    }

    #[test]
    fn test_0330_example_2() {
        let nums = vec![1, 5, 10];
        let n = 20;
        let result = 2;

        assert_eq!(Solution::min_patches(nums, n), result);
    }

    #[test]
    fn test_0330_example_3() {
        let nums = vec![1, 2, 2];
        let n = 5;
        let result = 0;

        assert_eq!(Solution::min_patches(nums, n), result);
    }
}
