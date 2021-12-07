/**
 * [0442] Find All Duplicates in an Array
 *
 * Given an integer array nums of length n where all the integers of nums are in the range [1, n] and each integer appears once or twice, return an array of all the integers that appears twice.
 * You must write an algorithm that runs in O(n) time and uses only constant extra space.
 *  
 * Example 1:
 * Input: nums = [4,3,2,7,8,2,3,1]
 * Output: [2,3]
 * Example 2:
 * Input: nums = [1,1,2]
 * Output: [1]
 * Example 3:
 * Input: nums = [1]
 * Output: []
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 10^5
 * 	1 <= nums[i] <= n
 * 	Each element in nums appears once or twice.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-all-duplicates-in-an-array/
// discuss: https://leetcode.com/problems/find-all-duplicates-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut seq = [0].repeat(nums.len());
        nums.into_iter()
            .for_each(|num| seq[(num - 1) as usize] += 1);
        seq.into_iter()
            .enumerate()
            .filter(|&(x, y)| y >= 2)
            .map(|(x, y)| (x + 1) as i32)
            .collect::<Vec<i32>>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0442_example_1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let result = vec![2, 3];

        assert_eq!(Solution::find_duplicates(nums), result);
    }

    #[test]
    fn test_0442_example_2() {
        let nums = vec![1, 1, 2];
        let result = vec![1];

        assert_eq!(Solution::find_duplicates(nums), result);
    }

    #[test]
    fn test_0442_example_3() {
        let nums = vec![1];
        let result = vec![];

        assert_eq!(Solution::find_duplicates(nums), result);
    }
}
