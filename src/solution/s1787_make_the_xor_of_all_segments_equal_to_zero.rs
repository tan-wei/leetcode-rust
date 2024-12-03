/**
 * [1787] Make the XOR of All Segments Equal to Zero
 *
 * You are given an array nums​​​ and an integer k​​​​​. The <font face="monospace">XOR</font> of a segment [left, right] where left <= right is the XOR of all the elements with indices between left and right, inclusive: nums[left] XOR nums[left+1] XOR ... XOR nums[right].
 * Return the minimum number of elements to change in the array such that the XOR of all segments of size k​​​​​​ is equal to zero.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,0,3,0], k = 1
 * Output: 3
 * Explanation: Modify the array from [<u>1</u>,<u>2</u>,0,<u>3</u>,0] to from [<u>0</u>,<u>0</u>,0,<u>0</u>,0].
 *
 * Example 2:
 *
 * Input: nums = [3,4,5,2,1,7,3,4,7], k = 3
 * Output: 3
 * Explanation: Modify the array from [3,4,<u>5</u>,<u>2</u>,<u>1</u>,7,3,4,7] to [3,4,<u>7</u>,<u>3</u>,<u>4</u>,7,3,4,7].
 *
 * Example 3:
 *
 * Input: nums = [1,2,4,1,2,5,1,2,6], k = 3
 * Output: 3
 * Explanation: Modify the array from [1,2,<u>4,</u>1,2,<u>5</u>,1,2,<u>6</u>] to [1,2,<u>3</u>,1,2,<u>3</u>,1,2,<u>3</u>].
 *  
 * Constraints:
 *
 * 	1 <= k <= nums.length <= 2000
 * 	​​​​​​0 <= nums[i] < 2^10
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/make-the-xor-of-all-segments-equal-to-zero/
// discuss: https://leetcode.com/problems/make-the-xor-of-all-segments-equal-to-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1787_example_1() {
        let nums = vec![1, 2, 0, 3, 0];
        let k = 1;

        let result = 3;

        assert_eq!(Solution::min_changes(nums, k), result);
    }

    #[test]
    #[ignore]
    fn test_1787_example_2() {
        let nums = vec![3, 4, 5, 2, 1, 7, 3, 4, 7];
        let k = 3;

        let result = 3;

        assert_eq!(Solution::min_changes(nums, k), result);
    }

    #[test]
    #[ignore]
    fn test_1787_example_3() {
        let nums = vec![1, 2, 4, 1, 2, 5, 1, 2, 6];
        let k = 3;

        let result = 3;

        assert_eq!(Solution::min_changes(nums, k), result);
    }
}
