/**
 * [2295] Replace Elements in an Array
 *
 * You are given a 0-indexed array nums that consists of n distinct positive integers. Apply m operations to this array, where in the i^th operation you replace the number operations[i][0] with operations[i][1].
 * It is guaranteed that in the i^th operation:
 *
 * 	operations[i][0] exists in nums.
 * 	operations[i][1] does not exist in nums.
 *
 * Return the array obtained after applying all the operations.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,4,6], operations = [[1,3],[4,7],[6,1]]
 * Output: [3,2,7,1]
 * Explanation: We perform the following operations on nums:
 * - Replace the number 1 with 3. nums becomes [<u>3</u>,2,4,6].
 * - Replace the number 4 with 7. nums becomes [3,2,<u>7</u>,6].
 * - Replace the number 6 with 1. nums becomes [3,2,7,<u>1</u>].
 * We return the final array [3,2,7,1].
 *
 * Example 2:
 *
 * Input: nums = [1,2], operations = [[1,3],[2,1],[3,2]]
 * Output: [2,1]
 * Explanation: We perform the following operations to nums:
 * - Replace the number 1 with 3. nums becomes [<u>3</u>,2].
 * - Replace the number 2 with 1. nums becomes [3,<u>1</u>].
 * - Replace the number 3 with 2. nums becomes [<u>2</u>,1].
 * We return the array [2,1].
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	m == operations.length
 * 	1 <= n, m <= 10^5
 * 	All the values of nums are distinct.
 * 	operations[i].length == 2
 * 	1 <= nums[i], operations[i][0], operations[i][1] <= 10^6
 * 	operations[i][0] will exist in nums when applying the i^th operation.
 * 	operations[i][1] will not exist in nums when applying the i^th operation.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/replace-elements-in-an-array/
// discuss: https://leetcode.com/problems/replace-elements-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/replace-elements-in-an-array/solutions/2991607/rust-concise-by-gvnpl-qalu/
    pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let mut hm = std::collections::HashMap::new();

        for i in 0..nums.len() {
            hm.insert(nums[i], i);
        }

        for o in operations {
            let i = hm.get(&o[0]).copied().unwrap_or(0);
            nums[i] = o[1];
            hm.insert(o[1], i);
        }

        nums
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2295_example_1() {
        let nums = vec![1, 2, 4, 6];
        let operations = vec![vec![1, 3], vec![4, 7], vec![6, 1]];

        let result = vec![3, 2, 7, 1];

        assert_eq!(Solution::array_change(nums, operations), result);
    }

    #[test]
    fn test_2295_example_2() {
        let nums = vec![1, 2];
        let operations = vec![vec![1, 3], vec![2, 1], vec![3, 2]];

        let result = vec![2, 1];

        assert_eq!(Solution::array_change(nums, operations), result);
    }
}
