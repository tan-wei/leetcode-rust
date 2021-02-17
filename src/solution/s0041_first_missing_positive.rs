/**
 * [41] First Missing Positive
 *
 * Given an unsorted integer array nums, find the smallest missing positive integer.
 *  
 * Example 1:
 * Input: nums = [1,2,0]
 * Output: 3
 * Example 2:
 * Input: nums = [3,4,-1,1]
 * Output: 2
 * Example 3:
 * Input: nums = [7,8,9,11,12]
 * Output: 1
 *  
 * Constraints:
 *
 * 	0 <= nums.length <= 300
 * 	-2^31 <= nums[i] <= 2^31 - 1
 *
 *  
 * Follow up: Could you implement an algorithm that runs in O(n) time and uses constant extra space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/first-missing-positive/
// discuss: https://leetcode.com/problems/first-missing-positive/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut seen = vec![false; n];

        for &num in nums.iter() {
            let num = num as usize;
            if num > 0 && num <= n {
                seen[num - 1] = true;
            }
        }

        seen.into_iter().position(|b| !b).unwrap_or(n) as i32 + 1
    }

    pub fn first_missing_positive_v2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        (0..n).for_each(|i| {
            let mut x = nums[i] as usize;
            while x > 0 && x <= n && x as i32 != nums[x - 1] {
                nums[i] = nums[x - 1];
                nums[x - 1] = x as i32;
                x = nums[i] as usize;
            }
        });
        (0..n).position(|i| nums[i] != i as i32 + 1).unwrap_or(n) as i32 + 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0041_example_1() {
        let nums = vec![1, 2, 0];

        assert_eq!(Solution::first_missing_positive(nums.clone()), 3);
        assert_eq!(Solution::first_missing_positive_v2(nums.clone()), 3);
    }
    #[test]
    fn test_0041_example_2() {
        let nums = vec![3, 4, -1, 1];

        assert_eq!(Solution::first_missing_positive(nums.clone()), 2);
        assert_eq!(Solution::first_missing_positive_v2(nums.clone()), 2);
    }
    #[test]
    fn test_0041_example_3() {
        let nums = vec![7, 8, 9, 11, 12];

        assert_eq!(Solution::first_missing_positive(nums.clone()), 1);
        assert_eq!(Solution::first_missing_positive_v2(nums.clone()), 1);
    }
}
