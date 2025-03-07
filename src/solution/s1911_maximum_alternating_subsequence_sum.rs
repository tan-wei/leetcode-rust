/**
 * [1911] Maximum Alternating Subsequence Sum
 *
 * The alternating sum of a 0-indexed array is defined as the sum of the elements at even indices minus the sum of the elements at odd indices.
 *
 *
 * 	For example, the alternating sum of [4,2,5,3] is (4 + 5) - (2 + 3) = 4.
 *
 *
 * Given an array nums, return the maximum alternating sum of any subsequence of nums (after reindexing the elements of the subsequence).
 *
 *
 *
 *
 * A subsequence of an array is a new array generated from the original array by deleting some elements (possibly none) without changing the remaining elements' relative order. For example, [2,7,4] is a subsequence of [4,<u>2</u>,3,<u>7</u>,2,1,<u>4</u>] (the underlined elements), while [2,4,2] is not.
 *
 *  
 * Example 1:
 *
 *
 * Input: nums = [<u>4</u>,<u>2</u>,<u>5</u>,3]
 * Output: 7
 * Explanation: It is optimal to choose the subsequence [4,2,5] with alternating sum (4 + 5) - 2 = 7.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [5,6,7,<u>8</u>]
 * Output: 8
 * Explanation: It is optimal to choose the subsequence [8] with alternating sum 8.
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [<u>6</u>,2,<u>1</u>,2,4,<u>5</u>]
 * Output: 10
 * Explanation: It is optimal to choose the subsequence [6,1,5] with alternating sum (6 + 5) - 1 = 10.
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-alternating-subsequence-sum/
// discuss: https://leetcode.com/problems/maximum-alternating-subsequence-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-alternating-subsequence-sum/solutions/3416719/rust-solution/
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let nums = nums.into_iter().map(|v| v as i64).collect::<Vec<i64>>();
        let mut a = nums[0];
        let mut b = 0;

        for i in 1..n {
            let v = nums[i];
            b = b.max(a - v);
            a = a.max(b + v);
        }

        a.max(b)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1911_example_1() {
        let nums = vec![4, 2, 5, 3];

        let result = 7;

        assert_eq!(Solution::max_alternating_sum(nums), result);
    }

    #[test]
    fn test_1911_example_2() {
        let nums = vec![5, 6, 7, 8];

        let result = 8;

        assert_eq!(Solution::max_alternating_sum(nums), result);
    }

    #[test]
    fn test_1911_example_3() {
        let nums = vec![6, 2, 1, 2, 4, 5];

        let result = 10;

        assert_eq!(Solution::max_alternating_sum(nums), result);
    }
}
