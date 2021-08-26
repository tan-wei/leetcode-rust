/**
 * [300] Longest Increasing Subsequence
 *
 * Given an integer array nums, return the length of the longest strictly increasing subsequence.
 * A subsequence is a sequence that can be derived from an array by deleting some or no elements without changing the order of the remaining elements. For example, [3,6,2,7] is a subsequence of the array [0,3,1,6,2,2,7].
 *  
 * Example 1:
 *
 * Input: nums = [10,9,2,5,3,7,101,18]
 * Output: 4
 * Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
 *
 * Example 2:
 *
 * Input: nums = [0,1,0,3,2,3]
 * Output: 4
 *
 * Example 3:
 *
 * Input: nums = [7,7,7,7,7,7,7]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2500
 * 	-10^4 <= nums[i] <= 10^4
 *
 *  
 * Follow up: Can you come up with an algorithm that runs in O(n log(n)) time complexity?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-increasing-subsequence/
// discuss: https://leetcode.com/problems/longest-increasing-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut piles: Vec<i32> = vec![];
        for num in nums {
            if let Err(i) = piles.binary_search(&num) {
                if i < piles.len() {
                    piles[i] = num;
                } else {
                    piles.push(num);
                }
            }
        }
        piles.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0300_example_1() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let result = 4;

        assert_eq!(Solution::length_of_lis(nums), result);
    }

    #[test]
    fn test_0300_example_2() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        let result = 4;

        assert_eq!(Solution::length_of_lis(nums), result);
    }

    #[test]
    fn test_0300_example_3() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let result = 1;

        assert_eq!(Solution::length_of_lis(nums), result);
    }
}
