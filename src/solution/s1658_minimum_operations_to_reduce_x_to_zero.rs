/**
 * [1658] Minimum Operations to Reduce X to Zero
 *
 * You are given an integer array nums and an integer x. In one operation, you can either remove the leftmost or the rightmost element from the array nums and subtract its value from x. Note that this modifies the array for future operations.
 * Return the minimum number of operations to reduce x to exactly 0 if it is possible, otherwise, return -1.
 *  
 * Example 1:
 *
 * Input: nums = [1,1,4,2,3], x = 5
 * Output: 2
 * Explanation: The optimal solution is to remove the last two elements to reduce x to zero.
 *
 * Example 2:
 *
 * Input: nums = [5,6,7,8,9], x = 4
 * Output: -1
 *
 * Example 3:
 *
 * Input: nums = [3,2,20,1,1,3], x = 10
 * Output: 5
 * Explanation: The optimal solution is to remove the last three elements and the first two elements (5 operations in total) to reduce x to zero.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^4
 * 	1 <= x <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/
// discuss: https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let (mut data, mut s) = (std::collections::HashMap::new(), 0);

        data.insert(0, 0);

        for i in 0..nums.len() {
            s += nums[i];
            data.insert(s, i + 1);
        }

        if s < x {
            return -1;
        }

        let (mut result, mut s) = (usize::MAX, 0);

        for i in 0..nums.len() {
            if data.contains_key(&(x - s)) {
                result = result.min(i + data.get(&(x - s)).unwrap());
            }
            s += nums[nums.len() - 1 - i];
            if s > x {
                break;
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
    fn test_1658_example_1() {
        let nums = vec![1, 1, 4, 2, 3];
        let x = 5;

        let result = 2;

        assert_eq!(Solution::min_operations(nums, x), result);
    }

    #[test]
    fn test_1658_example_2() {
        let nums = vec![5, 6, 7, 8, 9];
        let x = 4;

        let result = -1;

        assert_eq!(Solution::min_operations(nums, x), result);
    }

    #[test]
    fn test_1658_example_3() {
        let nums = vec![3, 2, 20, 1, 1, 3];
        let x = 10;

        let result = 5;

        assert_eq!(Solution::min_operations(nums, x), result);
    }
}
