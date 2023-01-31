/**
 * [0961] N-Repeated Element in Size 2N Array
 *
 * You are given an integer array nums with the following properties:
 *
 * 	nums.length == 2 * n.
 * 	nums contains n + 1 unique elements.
 * 	Exactly one element of nums is repeated n times.
 *
 * Return the element that is repeated n times.
 *  
 * Example 1:
 * Input: nums = [1,2,3,3]
 * Output: 3
 * Example 2:
 * Input: nums = [2,1,2,5,3,2]
 * Output: 2
 * Example 3:
 * Input: nums = [5,1,5,2,5,3,5,4]
 * Output: 5
 *  
 * Constraints:
 *
 * 	2 <= n <= 5000
 * 	nums.length == 2 * n
 * 	0 <= nums[i] <= 10^4
 * 	nums contains n + 1 unique elements and one of them is repeated exactly n times.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/n-repeated-element-in-size-2n-array/
// discuss: https://leetcode.com/problems/n-repeated-element-in-size-2n-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut num = 0;

        for i in nums.iter() {
            if nums.iter().filter(|x| x == &i).count() > 1 {
                num = *i;
                break;
            }
        }

        num
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0961_example_1() {
        let nums = vec![1, 2, 3, 3];
        let result = 3;

        assert_eq!(Solution::repeated_n_times(nums), result);
    }

    #[test]
    fn test_0961_example_2() {
        let nums = vec![2, 1, 2, 5, 3, 2];
        let result = 2;

        assert_eq!(Solution::repeated_n_times(nums), result);
    }

    #[test]
    fn test_0961_example_3() {
        let nums = vec![5, 1, 5, 2, 5, 3, 5, 4];
        let result = 5;

        assert_eq!(Solution::repeated_n_times(nums), result);
    }
}
