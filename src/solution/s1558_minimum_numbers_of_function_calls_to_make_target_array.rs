/**
 * [1558] Minimum Numbers of Function Calls to Make Target Array
 *
 * You are given an integer array nums. You have an integer array arr of the same length with all values set to 0 initially. You also have the following modify function:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/10/sample_2_1887.png" style="width: 573px; height: 294px;" />
 * You want to use the modify function to convert arr to nums using the minimum number of calls.
 * Return the minimum number of function calls to make nums from arr.
 * The test cases are generated so that the answer fits in a 32-bit signed integer.
 *  
 * Example 1:
 *
 * Input: nums = [1,5]
 * Output: 5
 * Explanation: Increment by 1 (second element): [0, 0] to get [0, 1] (1 operation).
 * Double all the elements: [0, 1] -> [0, 2] -> [0, 4] (2 operations).
 * Increment by 1 (both elements)  [0, 4] -> [1, 4] -> [1, 5] (2 operations).
 * Total of operations: 1 + 2 + 2 = 5.
 *
 * Example 2:
 *
 * Input: nums = [2,2]
 * Output: 3
 * Explanation: Increment by 1 (both elements) [0, 0] -> [0, 1] -> [1, 1] (2 operations).
 * Double all the elements: [1, 1] -> [2, 2] (1 operation).
 * Total of operations: 2 + 1 = 3.
 *
 * Example 3:
 *
 * Input: nums = [4,2,5]
 * Output: 6
 * Explanation: (initial)[0,0,0] -> [1,0,0] -> [1,0,1] -> [2,0,2] -> [2,1,2] -> [4,2,4] -> [4,2,5](nums).
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-numbers-of-function-calls-to-make-target-array/
// discuss: https://leetcode.com/problems/minimum-numbers-of-function-calls-to-make-target-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max = 0;
        for &num in nums.iter() {
            let mut cv = num;
            let mut temp = 0;
            while cv > 0 {
                if cv % 2 == 0 {
                    cv /= 2;
                    temp += 1;
                } else {
                    cv -= 1;
                    result += 1;
                }
            }
            max = max.max(temp);
        }

        result + max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1558_example_1() {
        let nums = vec![1, 5];

        let result = 5;

        assert_eq!(Solution::min_operations(nums), result);
    }

    #[test]
    fn test_1558_example_2() {
        let nums = vec![2, 2];

        let result = 3;

        assert_eq!(Solution::min_operations(nums), result);
    }

    #[test]
    fn test_1558_example_3() {
        let nums = vec![4, 2, 5];

        let result = 6;

        assert_eq!(Solution::min_operations(nums), result);
    }
}
