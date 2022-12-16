/**
 * [0915] Partition Array into Disjoint Intervals
 *
 * Given an integer array nums, partition it into two (contiguous) subarrays left and right so that:
 *
 * 	Every element in left is less than or equal to every element in right.
 * 	left and right are non-empty.
 * 	left has the smallest possible size.
 *
 * Return the length of left after such a partitioning.
 * Test cases are generated such that partitioning exists.
 *  
 * Example 1:
 *
 * Input: nums = [5,0,3,8,6]
 * Output: 3
 * Explanation: left = [5,0,3], right = [8,6]
 *
 * Example 2:
 *
 * Input: nums = [1,1,1,0,6,12]
 * Output: 4
 * Explanation: left = [1,1,1,0], right = [6,12]
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^6
 * 	There is at least one valid answer for the given input.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-array-into-disjoint-intervals/
// discuss: https://leetcode.com/problems/partition-array-into-disjoint-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut pre_max = nums[0];
        let mut total_max = nums[0];
        let mut pos = 0;
        for (i, n) in nums.iter().enumerate() {
            if pre_max > *n {
                pre_max = total_max;
                pos = i;
            } else if total_max < *n {
                total_max = *n;
            }
        }
        1 + pos as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0915_example_1() {
        let nums = vec![5, 0, 3, 8, 6];
        let result = 3;

        assert_eq!(Solution::partition_disjoint(nums), result);
    }

    #[test]
    fn test_0915_example_2() {
        let nums = vec![1, 1, 1, 0, 6, 12];
        let result = 4;

        assert_eq!(Solution::partition_disjoint(nums), result);
    }
}
