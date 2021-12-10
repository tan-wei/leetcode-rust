/**
 * [0446] Arithmetic Slices II - Subsequence
 *
 * Given an integer array nums, return the number of all the arithmetic subsequences of nums.
 * A sequence of numbers is called arithmetic if it consists of at least three elements and if the difference between any two consecutive elements is the same.
 *
 * 	For example, [1, 3, 5, 7, 9], [7, 7, 7, 7], and [3, -1, -5, -9] are arithmetic sequences.
 * 	For example, [1, 1, 2, 5, 7] is not an arithmetic sequence.
 *
 * A subsequence of an array is a sequence that can be formed by removing some elements (possibly none) of the array.
 *
 * 	For example, [2,5,10] is a subsequence of [1,2,1,<u>2</u>,4,1,<u>5</u>,<u>10</u>].
 *
 * The test cases are generated so that the answer fits in 32-bit integer.
 *  
 * Example 1:
 *
 * Input: nums = [2,4,6,8,10]
 * Output: 7
 * Explanation: All arithmetic subsequence slices are:
 * [2,4,6]
 * [4,6,8]
 * [6,8,10]
 * [2,4,6,8]
 * [4,6,8,10]
 * [2,4,6,8,10]
 * [2,6,10]
 *
 * Example 2:
 *
 * Input: nums = [7,7,7,7,7]
 * Output: 16
 * Explanation: Any subsequence of this array is arithmetic.
 *
 *  
 * Constraints:
 *
 * 	1  <= nums.length <= 1000
 * 	-2^31 <= nums[i] <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/arithmetic-slices-ii-subsequence/
// discuss: https://leetcode.com/problems/arithmetic-slices-ii-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/arithmetic-slices-ii-subsequence/discuss/1455360/Rust-solution
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut d: Vec<std::collections::HashMap<_, _>> = Vec::with_capacity(n);
        for &b in &nums {
            let mut cur = std::collections::HashMap::new();
            for (h, &a) in d.iter().zip(&nums) {
                let step = a as i64 - b as i64;
                *cur.entry(step).or_default() += h.get(&step).map_or(1, |&x| x + 1);
            }
            d.push(cur);
        }
        (d.iter().flat_map(|h| h.values()).sum::<usize>() - n * (n - 1) / 2) as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0446_example_1() {
        let nums = vec![2, 4, 6, 8, 10];
        let result = 7;

        assert_eq!(Solution::number_of_arithmetic_slices(nums), result);
    }

    #[test]
    fn test_0446_example_2() {
        let nums = vec![7, 7, 7, 7, 7];
        let result = 16;

        assert_eq!(Solution::number_of_arithmetic_slices(nums), result);
    }
}
