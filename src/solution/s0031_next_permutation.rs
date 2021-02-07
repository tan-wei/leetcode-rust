/**
 * [31] Next Permutation
 *
 * Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.
 * If such an arrangement is not possible, it must rearrange it as the lowest possible order (i.e., sorted in ascending order).
 * The replacement must be <a href="http://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in place</a> and use only constant extra memory.
 *  
 * Example 1:
 * Input: nums = [1,2,3]
 * Output: [1,3,2]
 * Example 2:
 * Input: nums = [3,2,1]
 * Output: [1,2,3]
 * Example 3:
 * Input: nums = [1,1,5]
 * Output: [1,5,1]
 * Example 4:
 * Input: nums = [1]
 * Output: [1]
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	0 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/next-permutation/
// discuss: https://leetcode.com/problems/next-permutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if let Some(i) = (1..nums.len()).rev().find(|&i| nums[i - 1] < nums[i]) {
            let j = (i..nums.len())
                .rev()
                .find(|&j| nums[i - 1] < nums[j])
                .unwrap();
            nums.swap(i - 1, j);
            nums[i..].reverse();
        } else {
            nums.reverse();
        };
    }

    pub fn next_permutation_v2(nums: &mut Vec<i32>) {
        let s = nums.as_mut_slice();
        let mut i = s.len() - 1;
        while i >= 1 && s[i - 1] >= s[i] {
            i -= 1;
        }

        if i >= 1 {
            let mut j = s.len() - 1;
            while s[j] <= s[i - 1] {
                j -= 1;
            }
            s.swap(i - 1, j);
        }
        s[i..].reverse();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0031_example_1() {
        // Version 1
        let mut nums = vec![1, 2, 3];
        let result = vec![1, 3, 2];

        Solution::next_permutation(&mut nums);

        assert_eq!(nums, result);

        // Version 2
        let mut nums = vec![1, 2, 3];
        let result = vec![1, 3, 2];

        Solution::next_permutation_v2(&mut nums);

        assert_eq!(nums, result);
    }

    #[test]
    fn test_0031_example_2() {
        // Version 1
        let mut nums = vec![3, 2, 1];
        let result = vec![1, 2, 3];

        Solution::next_permutation(&mut nums);

        assert_eq!(nums, result);

        // Version 2
        let mut nums = vec![3, 2, 1];
        let result = vec![1, 2, 3];

        Solution::next_permutation_v2(&mut nums);

        assert_eq!(nums, result);
    }

    #[test]
    fn test_0031_example_3() {
        // Version 1
        let mut nums = vec![1, 1, 5];
        let result = vec![1, 5, 1];

        Solution::next_permutation(&mut nums);

        assert_eq!(nums, result);

        // Version 2
        let mut nums = vec![1, 1, 5];
        let result = vec![1, 5, 1];

        Solution::next_permutation_v2(&mut nums);

        assert_eq!(nums, result);
    }

    #[test]
    fn test_0031_example_4() {
        // Version 1
        let mut nums = vec![1];
        let result = vec![1];

        Solution::next_permutation(&mut nums);

        assert_eq!(nums, result);

        // Version 2
        let mut nums = vec![1];
        let result = vec![1];

        Solution::next_permutation_v2(&mut nums);

        assert_eq!(nums, result);
    }
}
