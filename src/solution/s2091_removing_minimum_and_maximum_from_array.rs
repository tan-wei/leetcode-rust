/**
 * [2091] Removing Minimum and Maximum From Array
 *
 * You are given a 0-indexed array of distinct integers nums.
 * There is an element in nums that has the lowest value and an element that has the highest value. We call them the minimum and maximum respectively. Your goal is to remove both these elements from the array.
 * A deletion is defined as either removing an element from the front of the array or removing an element from the back of the array.
 * Return the minimum number of deletions it would take to remove both the minimum and maximum element from the array.
 *  
 * Example 1:
 *
 * Input: nums = [2,<u>10</u>,7,5,4,<u>1</u>,8,6]
 * Output: 5
 * Explanation:
 * The minimum element in the array is nums[5], which is 1.
 * The maximum element in the array is nums[1], which is 10.
 * We can remove both the minimum and maximum by removing 2 elements from the front and 3 elements from the back.
 * This results in 2 + 3 = 5 deletions, which is the minimum number possible.
 *
 * Example 2:
 *
 * Input: nums = [0,<u>-4</u>,<u>19</u>,1,8,-2,-3,5]
 * Output: 3
 * Explanation:
 * The minimum element in the array is nums[1], which is -4.
 * The maximum element in the array is nums[2], which is 19.
 * We can remove both the minimum and maximum by removing 3 elements from the front.
 * This results in only 3 deletions, which is the minimum number possible.
 *
 * Example 3:
 *
 * Input: nums = [<u>101</u>]
 * Output: 1
 * Explanation:  
 * There is only one element in the array, which makes it both the minimum and maximum element.
 * We can remove it with 1 deletion.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-10^5 <= nums[i] <= 10^5
 * 	The integers in nums are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/removing-minimum-and-maximum-from-array/
// discuss: https://leetcode.com/problems/removing-minimum-and-maximum-from-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2091_example_1() {
        let nums = vec![2, 10, 7, 5, 4, 1, 8, 6];

        let result = 5;

        assert_eq!(Solution::minimum_deletions(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2091_example_2() {
        let nums = vec![0, -4, 19, 1, 8, -2, -3, 5];

        let result = 3;

        assert_eq!(Solution::minimum_deletions(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2091_example_3() {
        let nums = vec![101];

        let result = 1;

        assert_eq!(Solution::minimum_deletions(nums), result);
    }
}
