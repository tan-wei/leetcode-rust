/**
 * [1636] Sort Array by Increasing Frequency
 *
 * Given an array of integers nums, sort the array in increasing order based on the frequency of the values. If multiple values have the same frequency, sort them in decreasing order.
 * Return the sorted array.
 *  
 * Example 1:
 *
 * Input: nums = [1,1,2,2,2,3]
 * Output: [3,1,1,2,2,2]
 * Explanation: '3' has a frequency of 1, '1' has a frequency of 2, and '2' has a frequency of 3.
 *
 * Example 2:
 *
 * Input: nums = [2,3,1,3,2]
 * Output: [1,3,3,2,2]
 * Explanation: '2' and '3' both have a frequency of 2, so they are sorted in decreasing order.
 *
 * Example 3:
 *
 * Input: nums = [-1,1,-6,4,5,-6,1,4,1]
 * Output: [5,-1,4,4,-6,-6,1,1,1]
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	-100 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-array-by-increasing-frequency/
// discuss: https://leetcode.com/problems/sort-array-by-increasing-frequency/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .fold([0; 201], |mut acc, num| {
                acc[(num + 100) as usize] += 1;
                acc
            })
            .iter()
            .enumerate()
            .filter(|&(_, n)| *n != 0)
            .map(|(i, n)| (n, std::cmp::Reverse(i as i32 - 100)))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .flat_map(|(n, std::cmp::Reverse(i))| std::iter::repeat(i).take(*n))
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1636_example_1() {
        let nums = vec![1, 1, 2, 2, 2, 3];

        let result = vec![3, 1, 1, 2, 2, 2];

        assert_eq!(Solution::frequency_sort(nums), result);
    }

    #[test]
    fn test_1636_example_2() {
        let nums = vec![2, 3, 1, 3, 2];

        let result = vec![1, 3, 3, 2, 2];

        assert_eq!(Solution::frequency_sort(nums), result);
    }

    #[test]
    fn test_1636_example_3() {
        let nums = vec![-1, 1, -6, 4, 5, -6, 1, 4, 1];

        let result = vec![5, -1, 4, 4, -6, -6, 1, 1, 1];

        assert_eq!(Solution::frequency_sort(nums), result);
    }
}
