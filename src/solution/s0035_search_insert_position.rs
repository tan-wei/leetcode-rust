/**
 * [35] Search Insert Position
 *
 * Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
 *  
 * Example 1:
 * Input: nums = [1,3,5,6], target = 5
 * Output: 2
 * Example 2:
 * Input: nums = [1,3,5,6], target = 2
 * Output: 1
 * Example 3:
 * Input: nums = [1,3,5,6], target = 7
 * Output: 4
 * Example 4:
 * Input: nums = [1,3,5,6], target = 0
 * Output: 0
 * Example 5:
 * Input: nums = [1], target = 0
 * Output: 0
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-10^4 <= nums[i] <= 10^4
 * 	nums contains distinct values sorted in ascending order.
 * 	-10^4 <= target <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/search-insert-position/
// discuss: https://leetcode.com/problems/search-insert-position/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0035_example_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let result = 2;

        assert_eq!(Solution::search_insert(nums, target), result);
    }

    #[test]
    fn test_0035_example_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let result = 1;

        assert_eq!(Solution::search_insert(nums, target), result);
    }

    #[test]
    fn test_0035_example_3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let result = 4;

        assert_eq!(Solution::search_insert(nums, target), result);
    }

    #[test]
    fn test_0035_example_4() {
        let nums = vec![1, 3, 5, 6];
        let target = 0;
        let result = 0;

        assert_eq!(Solution::search_insert(nums, target), result);
    }

    #[test]
    fn test_0035_example_5() {
        let nums = vec![1];
        let target = 0;
        let result = 0;

        assert_eq!(Solution::search_insert(nums, target), result);
    }
}
