/**
 * [0565] Array Nesting
 *
 * You are given an integer array nums of length n where nums is a permutation of the numbers in the range [0, n - 1].
 * You should build a set s[k] = {nums[k], nums[nums[k]], nums[nums[nums[k]]], ... } subjected to the following rule:
 *
 * 	The first element in s[k] starts with the selection of the element nums[k] of index = k.
 * 	The next element in s[k] should be nums[nums[k]], and then nums[nums[nums[k]]], and so on.
 * 	We stop adding right before a duplicate element occurs in s[k].
 *
 * Return the longest length of a set s[k].
 *  
 * Example 1:
 *
 * Input: nums = [5,4,0,3,1,6,2]
 * Output: 4
 * Explanation:
 * nums[0] = 5, nums[1] = 4, nums[2] = 0, nums[3] = 3, nums[4] = 1, nums[5] = 6, nums[6] = 2.
 * One of the longest sets s[k]:
 * s[0] = {nums[0], nums[5], nums[6], nums[2]} = {5, 6, 2, 0}
 *
 * Example 2:
 *
 * Input: nums = [0,1,2]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	0 <= nums[i] < nums.length
 * 	All the values of nums are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/array-nesting/
// discuss: https://leetcode.com/problems/array-nesting/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        (0..nums.len())
            .scan(vec![0; nums.len()], |state, i| {
                let (mut x, mut j) = (0, i);
                while state[nums[j] as usize] != -1 {
                    x += 1;
                    j = nums[j] as usize;
                    state[j] = -1;
                }
                Some(x)
            })
            .max()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0565_example_1() {
        let nums = vec![5, 4, 0, 3, 1, 6, 2];
        let result = 4;

        assert_eq!(Solution::array_nesting(nums), result);
    }

    #[test]
    fn test_0565_example_2() {
        let nums = vec![0, 1, 2];
        let result = 1;

        assert_eq!(Solution::array_nesting(nums), result);
    }
}
