/**
 * [189] Rotate Array
 *
 * Given an array, rotate the array to the right by k steps, where k is non-negative.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4,5,6,7], k = 3
 * Output: [5,6,7,1,2,3,4]
 * Explanation:
 * rotate 1 steps to the right: [7,1,2,3,4,5,6]
 * rotate 2 steps to the right: [6,7,1,2,3,4,5]
 * rotate 3 steps to the right: [5,6,7,1,2,3,4]
 *
 * Example 2:
 *
 * Input: nums = [-1,-100,3,99], k = 2
 * Output: [3,99,-1,-100]
 * Explanation:
 * rotate 1 steps to the right: [99,-1,-100,3]
 * rotate 2 steps to the right: [3,99,-1,-100]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	0 <= k <= 10^5
 *
 *  
 * Follow up:
 *
 * 	Try to come up with as many solutions as you can. There are at least three different ways to solve this problem.
 * 	Could you do it in-place with O(1) extra space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rotate-array/
// discuss: https://leetcode.com/problems/rotate-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0189_example_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        let result = vec![5, 6, 7, 1, 2, 3, 4];

        Solution::rotate(&mut nums, k);

        assert_eq!(nums, result);
    }

    #[test]
    fn test_0189_example_2() {
        let mut nums = vec![-1, -100, 3, 99];
        let k = 2;
        let result = vec![3, 99, -1, -100];

        Solution::rotate(&mut nums, k);

        assert_eq!(nums, result);
    }
}
