/**
* [1887] Reduction Operations to Make the Array Elements Equal
*
* Given an integer array nums, your goal is to make all elements in nums equal. To complete one operation, follow these steps:
* <ol>
* 	Find the largest value in nums. Let its index be i (0-indexed) and its value be largest. If there are multiple elements with the largest value, pick the smallest i.
* 	Find the next largest value in nums strictly smaller than largest. Let its value be nextLargest.
* 	Reduce nums[i] to nextLargest.

* Return the number of operations to make all elements in nums equal.
*
* Example 1:
*
* Input: nums = [5,1,3]
* Output: 3
* Explanation: It takes 3 operations to make all elements in nums equal:
* 1. largest = 5 at index 0. nextLargest = 3. Reduce nums[0] to 3. nums = [<u>3</u>,1,3].
* 2. largest = 3 at index 0. nextLargest = 1. Reduce nums[0] to 1. nums = [<u>1</u>,1,3].
* 3. largest = 3 at index 2. nextLargest = 1. Reduce nums[2] to 1. nums = [1,1,<u>1</u>].
*
* Example 2:
*
* Input: nums = [1,1,1]
* Output: 0
* Explanation: All elements in nums are already equal.
*
* Example 3:
*
* Input: nums = [1,1,2,2,3]
* Output: 4
* Explanation: It takes 4 operations to make all elements in nums equal:
* 1. largest = 3 at index 4. nextLargest = 2. Reduce nums[4] to 2. nums = [1,1,2,2,<u>2</u>].
* 2. largest = 2 at index 2. nextLargest = 1. Reduce nums[2] to 1. nums = [1,1,<u>1</u>,2,2].
* 3. largest = 2 at index 3. nextLargest = 1. Reduce nums[3] to 1. nums = [1,1,1,<u>1</u>,2].
* 4. largest = 2 at index 4. nextLargest = 1. Reduce nums[4] to 1. nums = [1,1,1,1,<u>1</u>].
*
*
* Constraints:
*
* 	1 <= nums.length <= 5 * 10^4
* 	1 <= nums[i] <= 5 * 10^4
*
*/
pub struct Solution {}

// problem: https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal/
// discuss: https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1887_example_1() {
        let nums = vec![5, 1, 3];

        let result = 3;

        assert_eq!(Solution::reduction_operations(nums), result);
    }

    #[test]
    #[ignore]
    fn test_1887_example_2() {
        let nums = vec![1, 1, 1];

        let result = 0;

        assert_eq!(Solution::reduction_operations(nums), result);
    }

    #[test]
    #[ignore]
    fn test_1887_example_3() {
        let nums = vec![1, 1, 2, 2, 3];

        let result = 4;

        assert_eq!(Solution::reduction_operations(nums), result);
    }
}
