/**
 * [0775] Global and Local Inversions
 *
 * You are given an integer array nums of length n which represents a permutation of all the integers in the range [0, n - 1].
 * The number of global inversions is the number of the different pairs (i, j) where:
 *
 * 	0 <= i < j < n
 * 	nums[i] > nums[j]
 *
 * The number of local inversions is the number of indices i where:
 *
 * 	0 <= i < n - 1
 * 	nums[i] > nums[i + 1]
 *
 * Return true if the number of global inversions is equal to the number of local inversions.
 *  
 * Example 1:
 *
 * Input: nums = [1,0,2]
 * Output: true
 * Explanation: There is 1 global inversion and 1 local inversion.
 *
 * Example 2:
 *
 * Input: nums = [1,2,0]
 * Output: false
 * Explanation: There are 2 global inversions and 1 local inversion.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 10^5
 * 	0 <= nums[i] < n
 * 	All the integers of nums are unique.
 * 	nums is a permutation of all the numbers in the range [0, n - 1].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/global-and-local-inversions/
// discuss: https://leetcode.com/problems/global-and-local-inversions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        nums.iter()
            .enumerate()
            .all(|(i, &n)| (n - i as i32).abs() <= 1)
    }

    // Credit: https://leetcode.com/problems/global-and-local-inversions/discuss/1143486/Rust-solution
    pub fn is_ideal_permutation_v2(nums: Vec<i32>) -> bool {
        nums.windows(3).all(|w| {
            (w[0] < w[1] && w[1] < w[2])
                || (w[0] - w[1] == 1 && w[1] < w[2])
                || (w[0] < w[1] && w[1] - w[2] == 1)
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0775_example_1() {
        let nums = vec![1, 0, 2];
        let result = true;

        assert_eq!(Solution::is_ideal_permutation(nums), result);

        let nums = vec![1, 0, 2];
        let result = true;

        assert_eq!(Solution::is_ideal_permutation_v2(nums), result);
    }

    #[test]
    fn test_0775_example_2() {
        let nums = vec![1, 2, 0];
        let result = false;

        assert_eq!(Solution::is_ideal_permutation(nums), result);

        let nums = vec![1, 2, 0];
        let result = false;

        assert_eq!(Solution::is_ideal_permutation_v2(nums), result);
    }
}
