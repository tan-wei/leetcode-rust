/**
 * [1752] Check if Array Is Sorted and Rotated
 *
 * Given an array nums, return true if the array was originally sorted in non-decreasing order, then rotated some number of positions (including zero). Otherwise, return false.
 * There may be duplicates in the original array.
 * Note: An array A rotated by x positions results in an array B of the same length such that A[i] == B[(i+x) % A.length], where % is the modulo operation.
 *  
 * Example 1:
 *
 * Input: nums = [3,4,5,1,2]
 * Output: true
 * Explanation: [1,2,3,4,5] is the original sorted array.
 * You can rotate the array by x = 3 positions to begin on the the element of value 3: [3,4,5,1,2].
 *
 * Example 2:
 *
 * Input: nums = [2,1,3,4]
 * Output: false
 * Explanation: There is no sorted array once rotated that can make nums.
 *
 * Example 3:
 *
 * Input: nums = [1,2,3]
 * Output: true
 * Explanation: [1,2,3] is the original sorted array.
 * You can rotate the array by x = 0 positions (i.e. no rotation) to make nums.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	1 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/
// discuss: https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut count = 0;

        for (i, v) in nums.iter().enumerate() {
            if v > &nums[(i + 1) % nums.len()] {
                count += 1;
            }
            if count > 1 {
                return false;
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1752_example_1() {
        let nums = vec![3, 4, 5, 1, 2];

        let result = true;

        assert_eq!(Solution::check(nums), result);
    }

    #[test]
    fn test_1752_example_2() {
        let nums = vec![2, 1, 3, 4];

        let result = false;

        assert_eq!(Solution::check(nums), result);
    }

    #[test]
    fn test_1752_example_3() {
        let nums = vec![1, 2, 3];

        let result = true;

        assert_eq!(Solution::check(nums), result);
    }
}
