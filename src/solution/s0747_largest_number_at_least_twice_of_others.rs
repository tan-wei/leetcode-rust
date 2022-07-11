/**
 * [0747] Largest Number At Least Twice of Others
 *
 * You are given an integer array nums where the largest integer is unique.
 * Determine whether the largest element in the array is at least twice as much as every other number in the array. If it is, return the index of the largest element, or return -1 otherwise.
 *  
 * Example 1:
 *
 * Input: nums = [3,6,1,0]
 * Output: 1
 * Explanation: 6 is the largest integer.
 * For every other number in the array x, 6 is at least twice as big as x.
 * The index of value 6 is 1, so we return 1.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4]
 * Output: -1
 * Explanation: 4 is less than twice the value of 3, so we return -1.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 50
 * 	0 <= nums[i] <= 100
 * 	The largest element in nums is unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-number-at-least-twice-of-others/
// discuss: https://leetcode.com/problems/largest-number-at-least-twice-of-others/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => -1,
            1 => 0,
            _ => {
                let mut largest = -1;
                let mut index = -1;
                let mut second_largest = -1;
                for i in 0..nums.len() {
                    if nums[i] > largest {
                        second_largest = largest;
                        largest = nums[i];
                        index = i as i32;
                    } else if nums[i] > second_largest {
                        second_largest = nums[i];
                    }
                }
                if largest >= second_largest * 2 {
                    index
                } else {
                    -1
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0747_example_1() {
        let nums = vec![3, 6, 1, 0];
        let result = 1;

        assert_eq!(Solution::dominant_index(nums), result);
    }

    #[test]
    fn test_0747_example_2() {
        let nums = vec![1, 2, 3, 4];
        let result = -1;

        assert_eq!(Solution::dominant_index(nums), result);
    }
}
