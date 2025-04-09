/**
 * [1955] Count Number of Special Subsequences
 *
 * A sequence is special if it consists of a positive number of 0s, followed by a positive number of 1s, then a positive number of 2s.
 *
 * 	For example, [0,1,2] and [0,0,1,1,1,2] are special.
 * 	In contrast, [2,1,0], [1], and [0,1,2,0] are not special.
 *
 * Given an array nums (consisting of only integers 0, 1, and 2), return the number of different subsequences that are special. Since the answer may be very large, return it modulo 10^9 + 7.
 * A subsequence of an array is a sequence that can be derived from the array by deleting some or no elements without changing the order of the remaining elements. Two subsequences are different if the set of indices chosen are different.
 *  
 * Example 1:
 *
 * Input: nums = [0,1,2,2]
 * Output: 3
 * Explanation: The special subsequences are bolded [<u>0</u>,<u>1</u>,<u>2</u>,2], [<u>0</u>,<u>1</u>,2,<u>2</u>], and [<u>0</u>,<u>1</u>,<u>2</u>,<u>2</u>].
 *
 * Example 2:
 *
 * Input: nums = [2,2,0,0]
 * Output: 0
 * Explanation: There are no special subsequences in [2,2,0,0].
 *
 * Example 3:
 *
 * Input: nums = [0,1,2,0,1,2]
 * Output: 7
 * Explanation: The special subsequences are bolded:
 * - [<u>0</u>,<u>1</u>,<u>2</u>,0,1,2]
 * - [<u>0</u>,<u>1</u>,2,0,1,<u>2</u>]
 * - [<u>0</u>,<u>1</u>,<u>2</u>,0,1,<u>2</u>]
 * - [<u>0</u>,<u>1</u>,2,0,<u>1</u>,<u>2</u>]
 * - [<u>0</u>,1,2,<u>0</u>,<u>1</u>,<u>2</u>]
 * - [<u>0</u>,1,2,0,<u>1</u>,<u>2</u>]
 * - [0,1,2,<u>0</u>,<u>1</u>,<u>2</u>]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 2
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-number-of-special-subsequences/
// discuss: https://leetcode.com/problems/count-number-of-special-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_special_subsequences(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1955_example_1() {
        let nums = vec![0, 1, 2, 2];

        let result = 3;

        assert_eq!(Solution::count_special_subsequences(nums), result);
    }

    #[test]
    #[ignore]
    fn test_1955_example_2() {
        let nums = vec![2, 2, 0, 0];

        let result = 0;

        assert_eq!(Solution::count_special_subsequences(nums), result);
    }

    #[test]
    #[ignore]
    fn test_1955_example_3() {
        let nums = vec![0, 1, 2, 0, 1, 2];

        let result = 7;

        assert_eq!(Solution::count_special_subsequences(nums), result);
    }
}
