/**
 * [0922] Sort Array By Parity II
 *
 * Given an array of integers nums, half of the integers in nums are odd, and the other half are even.
 * Sort the array so that whenever nums[i] is odd, i is odd, and whenever nums[i] is even, i is even.
 * Return any answer array that satisfies this condition.
 *  
 * Example 1:
 *
 * Input: nums = [4,2,5,7]
 * Output: [4,5,2,7]
 * Explanation: [4,7,2,5], [2,5,4,7], [2,7,4,5] would also have been accepted.
 *
 * Example 2:
 *
 * Input: nums = [2,3]
 * Output: [2,3]
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 2 * 10^4
 * 	nums.length is even.
 * 	Half of the integers in nums are even.
 * 	0 <= nums[i] <= 1000
 *
 *  
 * Follow Up: Could you solve it in-place?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-array-by-parity-ii/
// discuss: https://leetcode.com/problems/sort-array-by-parity-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .filter(|&num| num % 2 == 0)
            .zip(nums.iter().filter(|&num| num % 2 != 0))
            .flat_map(|(&even, &odd)| vec![even, odd].into_iter())
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0922_example_1() {
        let nums = vec![4, 2, 5, 7];
        let result = vec![4, 5, 2, 7];

        assert_eq!(Solution::sort_array_by_parity_ii(nums), result);
    }

    #[test]
    fn test_0922_example_2() {
        let nums = vec![2, 3];
        let result = vec![2, 3];

        assert_eq!(Solution::sort_array_by_parity_ii(nums), result);
    }
}
