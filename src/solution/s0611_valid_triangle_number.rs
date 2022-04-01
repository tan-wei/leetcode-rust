/**
 * [0611] Valid Triangle Number
 *
 * Given an integer array nums, return the number of triplets chosen from the array that can make triangles if we take them as side lengths of a triangle.
 *  
 * Example 1:
 *
 * Input: nums = [2,2,3,4]
 * Output: 3
 * Explanation: Valid combinations are:
 * 2,3,4 (using the first 2)
 * 2,3,4 (using the second 2)
 * 2,2,3
 *
 * Example 2:
 *
 * Input: nums = [4,2,3,4]
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	0 <= nums[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-triangle-number/
// discuss: https://leetcode.com/problems/valid-triangle-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let mut nums = nums;

        nums.sort_unstable();

        let mut result = 0;
        for k in (2..nums.len()).rev() {
            let mut i = 0;
            let mut j = k - 1;
            while i < j {
                // any value x between i...j will satisfy nums[x] + nums[j] > nums[k]
                // and because nums[k] > nums[j] > nums[x] >= 0, they will always satisfy
                // nums[k] + nums[x] > nums[j] and nums[k] + nums[j] > nums[x]
                if nums[i] + nums[j] > nums[k] {
                    j -= 1;
                    result += j - i + 1;
                } else {
                    i += 1;
                }
            }
        }
        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0611_example_1() {
        let nums = vec![2, 2, 3, 4];
        let result = 3;

        assert_eq!(Solution::triangle_number(nums), result);
    }

    #[test]
    fn test_0611_example_2() {
        let nums = vec![4, 2, 3, 4];
        let result = 4;

        assert_eq!(Solution::triangle_number(nums), result);
    }
}
