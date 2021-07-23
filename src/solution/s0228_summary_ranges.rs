/**
 * [228] Summary Ranges
 *
 * You are given a sorted unique integer array nums.
 * Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That is, each element of nums is covered by exactly one of the ranges, and there is no integer x such that x is in one of the ranges but not in nums.
 * Each range [a,b] in the list should be output as:
 *
 * 	"a->b" if a != b
 * 	"a" if a == b
 *
 *  
 * Example 1:
 *
 * Input: nums = [0,1,2,4,5,7]
 * Output: ["0->2","4->5","7"]
 * Explanation: The ranges are:
 * [0,2] --> "0->2"
 * [4,5] --> "4->5"
 * [7,7] --> "7"
 *
 * Example 2:
 *
 * Input: nums = [0,2,3,4,6,8,9]
 * Output: ["0","2->4","6","8->9"]
 * Explanation: The ranges are:
 * [0,0] --> "0"
 * [2,4] --> "2->4"
 * [6,6] --> "6"
 * [8,9] --> "8->9"
 *
 * Example 3:
 *
 * Input: nums = []
 * Output: []
 *
 * Example 4:
 *
 * Input: nums = [-1]
 * Output: ["-1"]
 *
 * Example 5:
 *
 * Input: nums = [0]
 * Output: ["0"]
 *
 *  
 * Constraints:
 *
 * 	0 <= nums.length <= 20
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	All the values of nums are unique.
 * 	nums is sorted in ascending order.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/summary-ranges/
// discuss: https://leetcode.com/problems/summary-ranges/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return vec![];
        }

        let mut previous = nums[0] - 1;
        let mut start = nums[0];
        let mut result = vec![];

        for i in 0..nums.len() {
            if nums[i] != (previous + 1) {
                if start == nums[i - 1] {
                    result.push(format!("{}", start));
                } else {
                    result.push(format!("{}->{}", start, nums[i - 1]));
                }
                start = nums[i];
            }
            previous = nums[i];
        }

        if start == nums[nums.len() - 1] {
            result.push(format!("{}", start));
        } else {
            result.push(format!("{}->{}", start, nums[nums.len() - 1]));
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0228_example_1() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let result = vec_string!["0->2", "4->5", "7"];

        assert_eq!(Solution::summary_ranges(nums), result);
    }

    #[test]
    fn test_0228_example_2() {
        let nums = vec![0, 2, 3, 4, 6, 8, 9];
        let result = vec_string!["0", "2->4", "6", "8->9"];

        assert_eq!(Solution::summary_ranges(nums), result);
    }

    #[test]
    fn test_0228_example_3() {
        let nums = vec![];
        let result: Vec<String> = vec_string![];

        assert_eq!(Solution::summary_ranges(nums), result);
    }

    #[test]
    fn test_0228_example_4() {
        let nums = vec![-1];
        let result = vec_string!["-1"];

        assert_eq!(Solution::summary_ranges(nums), result);
    }

    #[test]
    fn test_0228_example_5() {
        let nums = vec![0];
        let result = vec_string!["0"];

        assert_eq!(Solution::summary_ranges(nums), result);
    }
}
