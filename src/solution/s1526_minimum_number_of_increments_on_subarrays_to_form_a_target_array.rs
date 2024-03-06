/**
 * [1526] Minimum Number of Increments on Subarrays to Form a Target Array
 *
 * You are given an integer array target. You have an integer array initial of the same size as target with all elements initially zeros.
 * In one operation you can choose any subarray from initial and increment each value by one.
 * Return the minimum number of operations to form a target array from initial.
 * The test cases are generated so that the answer fits in a 32-bit integer.
 *  
 * Example 1:
 *
 * Input: target = [1,2,3,2,1]
 * Output: 3
 * Explanation: We need at least 3 operations to form the target array from the initial array.
 * [<u>0,0,0,0,0</u>] increment 1 from index 0 to 4 (inclusive).
 * [1,<u>1,1,1</u>,1] increment 1 from index 1 to 3 (inclusive).
 * [1,2,<u>2</u>,2,1] increment 1 at index 2.
 * [1,2,3,2,1] target array is formed.
 *
 * Example 2:
 *
 * Input: target = [3,1,1,2]
 * Output: 4
 * Explanation: [<u>0,0,0,0</u>] -> [1,1,1,<u>1</u>] -> [<u>1</u>,1,1,2] -> [<u>2</u>,1,1,2] -> [3,1,1,2]
 *
 * Example 3:
 *
 * Input: target = [3,1,5,4,2]
 * Output: 7
 * Explanation: [<u>0,0,0,0,0</u>] -> [<u>1</u>,1,1,1,1] -> [<u>2</u>,1,1,1,1] -> [3,1,<u>1,1,1</u>] -> [3,1,<u>2,2</u>,2] -> [3,1,<u>3,3</u>,2] -> [3,1,<u>4</u>,4,2] -> [3,1,5,4,2].
 *
 *  
 * Constraints:
 *
 * 	1 <= target.length <= 10^5
 * 	1 <= target[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array/
// discuss: https://leetcode.com/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        target
            .iter()
            .zip(target.iter().skip(1))
            .fold(target[0], |acc, z| acc + std::cmp::max(0, (z.1 - z.0)))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1526_example_1() {
        let target = vec![1, 2, 3, 2, 1];

        let result = 3;

        assert_eq!(Solution::min_number_operations(target), result);
    }

    #[test]
    fn test_1526_example_2() {
        let target = vec![3, 1, 1, 2];

        let result = 4;

        assert_eq!(Solution::min_number_operations(target), result);
    }

    #[test]
    fn test_1526_example_3() {
        let target = vec![3, 1, 5, 4, 2];

        let result = 7;

        assert_eq!(Solution::min_number_operations(target), result);
    }
}
