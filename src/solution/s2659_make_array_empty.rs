/**
 * [2659] Make Array Empty
 *
 * You are given an integer array nums containing distinct numbers, and you can perform the following operations until the array is empty:
 *
 * 	If the first element has the smallest value, remove it
 * 	Otherwise, put the first element at the end of the array.
 *
 * Return an integer denoting the number of operations it takes to make nums empty.
 *  
 * Example 1:
 *
 * Input: nums = [3,4,-1]
 * Output: 5
 *
 *
 *
 * 			Operation
 * 			Array
 *
 *
 *
 *
 * 			1
 * 			[4, -1, 3]
 *
 *
 * 			2
 * 			[-1, 3, 4]
 *
 *
 * 			3
 * 			[3, 4]
 *
 *
 * 			4
 * 			[4]
 *
 *
 * 			5
 * 			[]
 *
 *
 *
 * Example 2:
 *
 * Input: nums = [1,2,4,3]
 * Output: 5
 *
 *
 *
 * 			Operation
 * 			Array
 *
 *
 *
 *
 * 			1
 * 			[2, 4, 3]
 *
 *
 * 			2
 * 			[4, 3]
 *
 *
 * 			3
 * 			[3, 4]
 *
 *
 * 			4
 * 			[4]
 *
 *
 * 			5
 * 			[]
 *
 *
 *
 * Example 3:
 *
 * Input: nums = [1,2,3]
 * Output: 3
 *
 *
 *
 * 			Operation
 * 			Array
 *
 *
 *
 *
 * 			1
 * 			[2, 3]
 *
 *
 * 			2
 * 			[3]
 *
 *
 * 			3
 * 			[]
 *
 *
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 	All values in nums are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/make-array-empty/
// discuss: https://leetcode.com/problems/make-array-empty/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_operations_to_empty_array(nums: Vec<i32>) -> i64 {
        0i64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2659_example_1() {
        let nums = vec![3, 4, -1];

        let result = 5;

        assert_eq!(Solution::count_operations_to_empty_array(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2659_example_2() {
        let nums = vec![1, 2, 4, 3];

        let result = 5;

        assert_eq!(Solution::count_operations_to_empty_array(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2659_example_3() {
        let nums = vec![1, 2, 3];

        let result = 3;

        assert_eq!(Solution::count_operations_to_empty_array(nums), result);
    }
}
