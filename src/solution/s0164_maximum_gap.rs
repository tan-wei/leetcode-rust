/**
 * [164] Maximum Gap
 *
 * Given an integer array nums, return the maximum difference between two successive elements in its sorted form. If the array contains less than two elements, return 0.
 * You must write an algorithm that runs in linear time and uses linear extra space.
 *  
 * Example 1:
 *
 * Input: nums = [3,6,9,1]
 * Output: 3
 * Explanation: The sorted form of the array is [1,3,6,9], either (3,6) or (6,9) has the maximum difference 3.
 *
 * Example 2:
 *
 * Input: nums = [10]
 * Output: 0
 * Explanation: The array contains less than 2 elements, therefore return 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	0 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-gap/
// discuss: https://leetcode.com/problems/maximum-gap/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-gap/discuss/756567/Rust-translated
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        if n < 2 {
            return 0;
        }

        let mut bucket = vec![0; n];
        let mut e = 1;
        for i in 0..8 {
            let mut counts = [0; 16];
            for j in 0..n {
                counts[(&nums[j] / e % 16) as usize] += 1;
            }
            for j in 1..16 {
                counts[j] += counts[j - 1];
            }
            for j in (0..n).rev() {
                counts[(&nums[j] / e % 16) as usize] -= 1;
                bucket[counts[(&nums[j] / e % 16) as usize] as usize] = nums[j];
            }
            e <<= 4;
            std::mem::swap(&mut nums, &mut bucket);
        }

        let mut result = 0;

        for i in 1..n {
            result = result.max(nums[i] - nums[i - 1])
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0164_example_1() {
        let nums = vec![3, 6, 9, 1];
        let result = 3;

        assert_eq!(Solution::maximum_gap(nums), result);
    }

    #[test]
    fn test_0164_example_2() {
        let nums = vec![10];
        let result = 0;

        assert_eq!(Solution::maximum_gap(nums), result);
    }
}
