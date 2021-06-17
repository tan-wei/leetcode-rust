/**
 * [179] Largest Number
 *
 * Given a list of non-negative integers nums, arrange them such that they form the largest number.
 * Note: The result may be very large, so you need to return a string instead of an integer.
 *  
 * Example 1:
 *
 * Input: nums = [10,2]
 * Output: "210"
 *
 * Example 2:
 *
 * Input: nums = [3,30,34,5,9]
 * Output: "9534330"
 *
 * Example 3:
 *
 * Input: nums = [1]
 * Output: "1"
 *
 * Example 4:
 *
 * Input: nums = [10]
 * Output: "10"
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	0 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-number/
// discuss: https://leetcode.com/problems/largest-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut vs = nums.iter().map(|n| n.to_string()).collect::<Vec<String>>();
        vs.sort_by(|a, b| format!("{}{}", b, a).cmp(&format!("{}{}", a, b)));
        let mut result = vs.join("");
        if result.trim_start_matches("0").is_empty() {
            return "0".to_string();
        }
        result.to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0179_example_1() {
        let nums = vec![10, 2];
        let result = "210".to_string();

        assert_eq!(Solution::largest_number(nums), result);
    }

    #[test]
    fn test_0179_example_2() {
        let nums = vec![3, 30, 34, 5, 9];
        let result = "9534330".to_string();

        assert_eq!(Solution::largest_number(nums), result);
    }

    #[test]
    fn test_0179_example_3() {
        let nums = vec![1];
        let result = "1".to_string();

        assert_eq!(Solution::largest_number(nums), result);
    }

    #[test]
    fn test_0179_example_4() {
        let nums = vec![10];
        let result = "10".to_string();

        assert_eq!(Solution::largest_number(nums), result);
    }
}
