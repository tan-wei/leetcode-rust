/**
 * [1748] Sum of Unique Elements
 *
 * You are given an integer array nums. The unique elements of an array are the elements that appear exactly once in the array.
 * Return the sum of all the unique elements of nums.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,2]
 * Output: 4
 * Explanation: The unique elements are [1,3], and the sum is 4.
 *
 * Example 2:
 *
 * Input: nums = [1,1,1,1,1]
 * Output: 0
 * Explanation: There are no unique elements, and the sum is 0.
 *
 * Example 3:
 *
 * Input: nums = [1,2,3,4,5]
 * Output: 15
 * Explanation: The unique elements are [1,2,3,4,5], and the sum is 15.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	1 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-unique-elements/
// discuss: https://leetcode.com/problems/sum-of-unique-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let counter: std::collections::HashMap<i32, i32> =
            nums.iter()
                .fold(std::collections::HashMap::new(), |mut hm, number| {
                    *hm.entry(*number).or_insert(0) += 1;
                    hm
                });

        counter
            .iter()
            .filter(|(num, count)| *count == &1)
            .map(|(num, count)| num)
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1748_example_1() {
        let nums = vec![1, 2, 3, 2];

        let result = 4;

        assert_eq!(Solution::sum_of_unique(nums), result);
    }

    #[test]
    fn test_1748_example_2() {
        let nums = vec![1, 1, 1, 1, 1];

        let result = 0;

        assert_eq!(Solution::sum_of_unique(nums), result);
    }

    #[test]
    fn test_1748_example_3() {
        let nums = vec![1, 2, 3, 4, 5];

        let result = 15;

        assert_eq!(Solution::sum_of_unique(nums), result);
    }
}
