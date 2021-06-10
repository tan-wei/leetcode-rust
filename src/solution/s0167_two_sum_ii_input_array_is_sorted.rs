/**
 * [167] Two Sum II - Input array is sorted
 *
 * Given an array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number.
 * Return the indices of the two numbers (1-indexed) as an integer array answer of size 2, where 1 <= answer[0] < answer[1] <= numbers.length.
 * The tests are generated such that there is exactly one solution. You may not use the same element twice.
 *  
 * Example 1:
 *
 * Input: numbers = [2,7,11,15], target = 9
 * Output: [1,2]
 * Explanation: The sum of 2 and 7 is 9. Therefore index1 = 1, index2 = 2.
 *
 * Example 2:
 *
 * Input: numbers = [2,3,4], target = 6
 * Output: [1,3]
 *
 * Example 3:
 *
 * Input: numbers = [-1,0], target = -1
 * Output: [1,2]
 *
 *  
 * Constraints:
 *
 * 	2 <= numbers.length <= 3 * 10^4
 * 	-1000 <= numbers[i] <= 1000
 * 	numbers is sorted in non-decreasing order.
 * 	-1000 <= target <= 1000
 * 	The tests are generated such that there is exactly one solution.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
// discuss: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            if numbers[left] + numbers[right] == target {
                return vec![(left + 1) as i32, (right + 1) as i32];
            } else if numbers[left] + numbers[right] > target {
                right -= 1;
            } else {
                left += 1;
            }
        }

        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0167_example_1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let result = vec![1, 2];

        assert_eq!(Solution::two_sum(numbers, target), result);
    }

    #[test]
    fn test_0167_example_2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        let result = vec![1, 3];

        assert_eq!(Solution::two_sum(numbers, target), result);
    }

    #[test]
    fn test_0167_example_3() {
        let numbers = vec![-1, 0];
        let target = -1;
        let result = vec![1, 2];

        assert_eq!(Solution::two_sum(numbers, target), result);
    }
}
