/**
 * [2183] Count Array Pairs Divisible by K
 *
 * Given a 0-indexed integer array nums of length n and an integer k, return the number of pairs (i, j) such that:
 *
 * 	0 <= i < j <= n - 1 and
 * 	nums[i] * nums[j] is divisible by k.
 *
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4,5], k = 2
 * Output: 7
 * Explanation:
 * The 7 pairs of indices whose corresponding products are divisible by 2 are
 * (0, 1), (0, 3), (1, 2), (1, 3), (1, 4), (2, 3), and (3, 4).
 * Their products are 2, 4, 6, 8, 10, 12, and 20 respectively.
 * Other pairs such as (0, 2) and (2, 4) have products 3 and 15 respectively, which are not divisible by 2.    
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4], k = 5
 * Output: 0
 * Explanation: There does not exist any pair of indices whose corresponding product is divisible by 5.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i], k <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-array-pairs-divisible-by-k/
// discuss: https://leetcode.com/problems/count-array-pairs-divisible-by-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2183_example_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 2;

        let result = 7;

        assert_eq!(Solution::count_pairs(nums, k), result);
    }

    #[test]
    #[ignore]
    fn test_2183_example_2() {
        let nums = vec![1, 2, 3, 4];
        let k = 5;

        let result = 0;

        assert_eq!(Solution::count_pairs(nums, k), result);
    }
}
