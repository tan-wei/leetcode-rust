/**
 * [0456] 132 Pattern
 *
 * Given an array of n integers nums, a 132 pattern is a subsequence of three integers nums[i], nums[j] and nums[k] such that i < j < k and nums[i] < nums[k] < nums[j].
 * Return true if there is a 132 pattern in nums, otherwise, return false.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4]
 * Output: false
 * Explanation: There is no 132 pattern in the sequence.
 *
 * Example 2:
 *
 * Input: nums = [3,1,4,2]
 * Output: true
 * Explanation: There is a 132 pattern in the sequence: [1, 4, 2].
 *
 * Example 3:
 *
 * Input: nums = [-1,3,2,0]
 * Output: true
 * Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1, 3, 0] and [-1, 2, 0].
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 2 * 10^5
 * 	-10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/132-pattern/
// discuss: https://leetcode.com/problems/132-pattern/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack = Vec::new();
        let mut k = i32::min_value();
        for num in nums.into_iter().rev() {
            if k > num {
                return true;
            }
            while stack.len() > 0 && *stack.last().unwrap() < num {
                k = stack.pop().unwrap();
            }
            stack.push(num);
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0456_example_1() {
        let nums = vec![1, 2, 3, 4];
        let result = false;

        assert_eq!(Solution::find132pattern(nums), result);
    }

    #[test]
    fn test_0456_example_2() {
        let nums = vec![3, 1, 4, 2];
        let result = true;

        assert_eq!(Solution::find132pattern(nums), result);
    }

    #[test]
    fn test_0456_example_3() {
        let nums = vec![-1, 3, 2, 0];
        let result = true;

        assert_eq!(Solution::find132pattern(nums), result);
    }
}
