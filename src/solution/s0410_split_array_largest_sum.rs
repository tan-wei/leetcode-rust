/**
 * [0410] Split Array Largest Sum
 *
 * Given an array nums which consists of non-negative integers and an integer m, you can split the array into m non-empty continuous subarrays.
 * Write an algorithm to minimize the largest sum among these m subarrays.
 *  
 * Example 1:
 *
 * Input: nums = [7,2,5,10,8], m = 2
 * Output: 18
 * Explanation:
 * There are four ways to split nums into two subarrays.
 * The best way is to split it into [7,2,5] and [10,8],
 * where the largest sum among the two subarrays is only 18.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4,5], m = 2
 * Output: 9
 *
 * Example 3:
 *
 * Input: nums = [1,4,4], m = 3
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	0 <= nums[i] <= 10^6
 * 	1 <= m <= min(50, nums.length)
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/split-array-largest-sum/
// discuss: https://leetcode.com/problems/split-array-largest-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/split-array-largest-sum/discuss/1308438/Rust-binary-search-solution
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let n = nums.len();
        let mut right = nums.iter().sum::<i32>();
        let mut left = *nums.iter().max().unwrap();
        while left < right {
            let mid = left + (right - left) / 2;
            let mut acc = 0;
            let mut bucket_ct = 1;
            for i in 0..n {
                if acc + nums[i] <= mid {
                    acc += nums[i];
                } else {
                    acc = nums[i];
                    bucket_ct += 1;
                }
            }
            if bucket_ct > m {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0410_example_1() {
        let nums = vec![7, 2, 5, 10, 8];
        let m = 2;
        let result = 18;

        assert_eq!(Solution::split_array(nums, m), result);
    }

    #[test]
    fn test_0410_example_2() {
        let nums = vec![1, 2, 3, 4, 5];
        let m = 2;
        let result = 9;

        assert_eq!(Solution::split_array(nums, m), result);
    }

    #[test]
    fn test_0410_example_3() {
        let nums = vec![1, 4, 4];
        let m = 3;
        let result = 4;

        assert_eq!(Solution::split_array(nums, m), result);
    }
}
