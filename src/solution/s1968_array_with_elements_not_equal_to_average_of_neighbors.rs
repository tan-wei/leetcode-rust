/**
 * [1968] Array With Elements Not Equal to Average of Neighbors
 *
 * You are given a 0-indexed array nums of distinct integers. You want to rearrange the elements in the array such that every element in the rearranged array is not equal to the average of its neighbors.
 * More formally, the rearranged array should have the property such that for every i in the range 1 <= i < nums.length - 1, (nums[i-1] + nums[i+1]) / 2 is not equal to nums[i].
 * Return any rearrangement of nums that meets the requirements.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4,5]
 * Output: [1,2,4,5,3]
 * Explanation:
 * When i=1, nums[i] = 2, and the average of its neighbors is (1+4) / 2 = 2.5.
 * When i=2, nums[i] = 4, and the average of its neighbors is (2+5) / 2 = 3.5.
 * When i=3, nums[i] = 5, and the average of its neighbors is (4+3) / 2 = 3.5.
 *
 * Example 2:
 *
 * Input: nums = [6,2,0,9,7]
 * Output: [9,7,6,2,0]
 * Explanation:
 * When i=1, nums[i] = 7, and the average of its neighbors is (9+6) / 2 = 7.5.
 * When i=2, nums[i] = 6, and the average of its neighbors is (7+2) / 2 = 4.5.
 * When i=3, nums[i] = 2, and the average of its neighbors is (6+0) / 2 = 3.
 * Note that the original array [6,2,0,9,7] also satisfies the conditions.
 *  
 * Constraints:
 *
 * 	3 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/array-with-elements-not-equal-to-average-of-neighbors/
// discuss: https://leetcode.com/problems/array-with-elements-not-equal-to-average-of-neighbors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1968_example_1() {
        let nums = vec![1, 2, 3, 4, 5];

        let result = vec![1, 2, 4, 5, 3];

        assert_eq!(Solution::rearrange_array(nums), result);
    }

    #[test]
    #[ignore]
    fn test_1968_example_2() {
        let nums = vec![6, 2, 0, 9, 7];

        let result = vec![9, 7, 6, 2, 0];

        assert_eq!(Solution::rearrange_array(nums), result);
    }
}
