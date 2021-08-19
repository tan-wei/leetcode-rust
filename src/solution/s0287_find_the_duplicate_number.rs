/**
 * [287] Find the Duplicate Number
 *
 * Given an array of integers nums containing n + 1 integers where each integer is in the range [1, n] inclusive.
 * There is only one repeated number in nums, return this repeated number.
 * You must solve the problem without modifying the array nums and uses only constant extra space.
 *  
 * Example 1:
 * Input: nums = [1,3,4,2,2]
 * Output: 2
 * Example 2:
 * Input: nums = [3,1,3,4,2]
 * Output: 3
 * Example 3:
 * Input: nums = [1,1]
 * Output: 1
 * Example 4:
 * Input: nums = [1,1,2]
 * Output: 1
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 * 	nums.length == n + 1
 * 	1 <= nums[i] <= n
 * 	All the integers in nums appear only once except for precisely one integer which appears two or more times.
 *
 *  
 * Follow up:
 *
 * 	How can we prove that at least one duplicate number must exist in nums?
 * 	Can you solve the problem in linear runtime complexity?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-duplicate-number/
// discuss: https://leetcode.com/problems/find-the-duplicate-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len();

        while low < high {
            let mut count = 0;
            let mid = (high - low) / 2 + low;
            for &num in nums.iter() {
                if (num as usize) <= mid {
                    count += 1;
                }
            }

            if count <= mid {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0287_example_1() {
        let nums = vec![1, 3, 4, 2, 2];
        let result = 2;

        assert_eq!(Solution::find_duplicate(nums), result);
    }

    #[test]
    fn test_0287_example_2() {
        let nums = vec![3, 1, 3, 4, 2];
        let result = 3;

        assert_eq!(Solution::find_duplicate(nums), result);
    }

    #[test]
    fn test_0287_example_3() {
        let nums = vec![1, 1];
        let result = 1;

        assert_eq!(Solution::find_duplicate(nums), result);
    }

    #[test]
    fn test_0287_example_4() {
        let nums = vec![1, 1, 2];
        let result = 1;

        assert_eq!(Solution::find_duplicate(nums), result);
    }
}
