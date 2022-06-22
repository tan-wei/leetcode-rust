/**
 * [0724] Find Pivot Index
 *
 * Given an array of integers nums, calculate the pivot index of this array.
 * The pivot index is the index where the sum of all the numbers strictly to the left of the index is equal to the sum of all the numbers strictly to the index's right.
 * If the index is on the left edge of the array, then the left sum is 0 because there are no elements to the left. This also applies to the right edge of the array.
 * Return the leftmost pivot index. If no such index exists, return -1.
 *  
 * Example 1:
 *
 * Input: nums = [1,7,3,6,5,6]
 * Output: 3
 * Explanation:
 * The pivot index is 3.
 * Left sum = nums[0] + nums[1] + nums[2] = 1 + 7 + 3 = 11
 * Right sum = nums[4] + nums[5] = 5 + 6 = 11
 *
 * Example 2:
 *
 * Input: nums = [1,2,3]
 * Output: -1
 * Explanation:
 * There is no index that satisfies the conditions in the problem statement.
 * Example 3:
 *
 * Input: nums = [2,1,-1]
 * Output: 0
 * Explanation:
 * The pivot index is 0.
 * Left sum = 0 (no elements to the left of index 0)
 * Right sum = nums[1] + nums[2] = 1 + -1 = 0
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-1000 <= nums[i] <= 1000
 *
 *  
 * Note: This question is the same as 1991: <a href="https://leetcode.com/problems/find-the-middle-index-in-array/" target="_blank">https://leetcode.com/problems/find-the-middle-index-in-array/</a>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-pivot-index/
// discuss: https://leetcode.com/problems/find-pivot-index/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        let mut left_sum = 0;
        for (i, &val) in nums.iter().enumerate() {
            if total_sum - left_sum - val == left_sum {
                return i as _;
            }
            left_sum += val;
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0724_example_1() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        let result = 3;

        assert_eq!(Solution::pivot_index(nums), result);
    }

    #[test]
    fn test_0724_example_2() {
        let nums = vec![1, 2, 3];
        let result = -1;

        assert_eq!(Solution::pivot_index(nums), result);
    }

    #[test]
    fn test_0724_example_3() {
        let nums = vec![2, 1, -1];
        let result = 0;

        assert_eq!(Solution::pivot_index(nums), result);
    }
}
