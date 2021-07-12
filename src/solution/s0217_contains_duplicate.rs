/**
 * [217] Contains Duplicate
 *
 * Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
 *  
 * Example 1:
 * Input: nums = [1,2,3,1]
 * Output: true
 * Example 2:
 * Input: nums = [1,2,3,4]
 * Output: false
 * Example 3:
 * Input: nums = [1,1,1,3,3,4,3,2,4,2]
 * Output: true
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/contains-duplicate/
// discuss: https://leetcode.com/problems/contains-duplicate/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.iter().collect::<std::collections::HashSet<_>>().len() != nums.len()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0217_example_1() {
        let nums = vec![1, 2, 3, 1];
        let result = true;

        assert_eq!(Solution::contains_duplicate(nums), result);
    }

    #[test]
    fn test_0217_example_2() {
        let nums = vec![1, 2, 3, 4];
        let result = false;

        assert_eq!(Solution::contains_duplicate(nums), result);
    }

    #[test]
    fn test_0217_example_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let result = true;

        assert_eq!(Solution::contains_duplicate(nums), result);
    }
}
