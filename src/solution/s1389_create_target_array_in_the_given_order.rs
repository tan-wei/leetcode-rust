/**
 * [1389] Create Target Array in the Given Order
 *
 * Given two arrays of integers nums and index. Your task is to create target array under the following rules:
 *
 * 	Initially target array is empty.
 * 	From left to right read nums[i] and index[i], insert at index index[i] the value nums[i] in target array.
 * 	Repeat the previous step until there are no elements to read in nums and index.
 *
 * Return the target array.
 * It is guaranteed that the insertion operations will be valid.
 *  
 * Example 1:
 *
 * Input: nums = [0,1,2,3,4], index = [0,1,2,2,1]
 * Output: [0,4,1,3,2]
 * Explanation:
 * nums       index     target
 * 0            0        [0]
 * 1            1        [0,1]
 * 2            2        [0,1,2]
 * 3            2        [0,1,3,2]
 * 4            1        [0,4,1,3,2]
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4,0], index = [0,1,2,3,0]
 * Output: [0,1,2,3,4]
 * Explanation:
 * nums       index     target
 * 1            0        [1]
 * 2            1        [1,2]
 * 3            2        [1,2,3]
 * 4            3        [1,2,3,4]
 * 0            0        [0,1,2,3,4]
 *
 * Example 3:
 *
 * Input: nums = [1], index = [0]
 * Output: [1]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length, index.length <= 100
 * 	nums.length == index.length
 * 	0 <= nums[i] <= 100
 * 	0 <= index[i] <= i
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/create-target-array-in-the-given-order/
// discuss: https://leetcode.com/problems/create-target-array-in-the-given-order/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut target = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            target.insert(index[i] as usize, nums[i]);
        }

        target
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1389_example_1() {
        let nums = vec![0, 1, 2, 3, 4];
        let index = vec![0, 1, 2, 2, 1];

        let result = vec![0, 4, 1, 3, 2];

        assert_eq!(Solution::create_target_array(nums, index), result);
    }

    #[test]
    fn test_1389_example_2() {
        let nums = vec![1, 2, 3, 4, 0];
        let index = vec![0, 1, 2, 3, 0];

        let result = vec![0, 1, 2, 3, 4];

        assert_eq!(Solution::create_target_array(nums, index), result);
    }

    #[test]
    fn test_1389_example_3() {
        let nums = vec![1];
        let index = vec![0];

        let result = vec![1];

        assert_eq!(Solution::create_target_array(nums, index), result);
    }
}
