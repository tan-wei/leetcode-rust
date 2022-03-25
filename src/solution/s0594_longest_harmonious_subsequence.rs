/**
 * [0594] Longest Harmonious Subsequence
 *
 * We define a harmonious array as an array where the difference between its maximum value and its minimum value is exactly 1.
 *
 * Given an integer array nums, return the length of its longest harmonious subsequence among all its possible subsequences.
 *
 * A subsequence of array is a sequence that can be derived from the array by deleting some or no elements without changing the order of the remaining elements.
 *
 *  
 * Example 1:
 *
 *
 * Input: nums = [1,3,2,2,5,2,3,7]
 * Output: 5
 * Explanation: The longest harmonious subsequence is [3,2,2,2,3].
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,2,3,4]
 * Output: 2
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [1,1,1,1]
 * Output: 0
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= nums.length <= 2 * 10^4
 * 	-10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-harmonious-subsequence/
// discuss: https://leetcode.com/problems/longest-harmonious-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut hm = std::collections::HashMap::new();
        nums.iter()
            .for_each(|&num| *hm.entry(num).or_insert(0) += 1);
        hm.iter().fold(0, |acc, (&num, &count)| {
            hm.get(&(num + 1)).map_or(acc, |c| acc.max(count + c))
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0594_example_1() {
        let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
        let result = 5;

        assert_eq!(Solution::find_lhs(nums), result);
    }

    #[test]
    fn test_0594_example_2() {
        let nums = vec![1, 2, 3, 4];
        let result = 2;

        assert_eq!(Solution::find_lhs(nums), result);
    }

    #[test]
    fn test_0594_example_3() {
        let nums = vec![1, 1, 1, 1];
        let result = 0;

        assert_eq!(Solution::find_lhs(nums), result);
    }
}
