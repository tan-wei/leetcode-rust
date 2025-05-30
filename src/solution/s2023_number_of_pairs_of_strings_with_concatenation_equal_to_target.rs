/**
 * [2023] Number of Pairs of Strings With Concatenation Equal to Target
 *
 * Given an array of digit strings nums and a digit string target, return the number of pairs of indices (i, j) (where i != j) such that the concatenation of nums[i] + nums[j] equals target.
 *  
 * Example 1:
 *
 * Input: nums = ["777","7","77","77"], target = "7777"
 * Output: 4
 * Explanation: Valid pairs are:
 * - (0, 1): "777" + "7"
 * - (1, 0): "7" + "777"
 * - (2, 3): "77" + "77"
 * - (3, 2): "77" + "77"
 *
 * Example 2:
 *
 * Input: nums = ["123","4","12","34"], target = "1234"
 * Output: 2
 * Explanation: Valid pairs are:
 * - (0, 1): "123" + "4"
 * - (2, 3): "12" + "34"
 *
 * Example 3:
 *
 * Input: nums = ["1","1","1"], target = "11"
 * Output: 6
 * Explanation: Valid pairs are:
 * - (0, 1): "1" + "1"
 * - (1, 0): "1" + "1"
 * - (0, 2): "1" + "1"
 * - (2, 0): "1" + "1"
 * - (1, 2): "1" + "1"
 * - (2, 1): "1" + "1"
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 100
 * 	1 <= nums[i].length <= 100
 * 	2 <= target.length <= 100
 * 	nums[i] and target consist of digits.
 * 	nums[i] and target do not have leading zeros.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-pairs-of-strings-with-concatenation-equal-to-target/
// discuss: https://leetcode.com/problems/number-of-pairs-of-strings-with-concatenation-equal-to-target/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        nums.iter()
            .enumerate()
            .flat_map(|(i, a)| {
                nums.iter()
                    .enumerate()
                    .filter(move |(j, _)| i != *j)
                    .map(move |(_, b)| (a, b))
            })
            .filter(|&(a, b)| target.strip_prefix(a).is_some_and(|suffix| suffix == b))
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_example_1() {
        let nums = vec_string!["777", "7", "77", "77"];
        let target = "7777".to_string();

        let result = 4;

        assert_eq!(Solution::num_of_pairs(nums, target), result);
    }

    #[test]
    fn test_2023_example_2() {
        let nums = vec_string!["123", "4", "12", "34"];
        let target = "1234".to_string();

        let result = 2;

        assert_eq!(Solution::num_of_pairs(nums, target), result);
    }

    #[test]
    fn test_2023_example_3() {
        let nums = vec_string!["1", "1", "1"];
        let target = "11".to_string();

        let result = 6;

        assert_eq!(Solution::num_of_pairs(nums, target), result);
    }
}
