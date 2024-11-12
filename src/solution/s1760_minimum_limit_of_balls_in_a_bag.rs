/**
 * [1760] Minimum Limit of Balls in a Bag
 *
 * You are given an integer array nums where the i^th bag contains nums[i] balls. You are also given an integer maxOperations.
 * You can perform the following operation at most maxOperations times:
 *
 * 	Take any bag of balls and divide it into two new bags with a positive number of balls.
 *
 * 		For example, a bag of 5 balls can become two new bags of 1 and 4 balls, or two new bags of 2 and 3 balls.
 *
 *
 *
 * Your penalty is the maximum number of balls in a bag. You want to minimize your penalty after the operations.
 * Return the minimum possible penalty after performing the operations.
 *  
 * Example 1:
 *
 * Input: nums = [9], maxOperatios = 2
 * Output: 3
 * Explanation:
 * - Divide the bag with 9 balls into two bags of sizes 6 and 3. [<u>9</u>] -> [6,3].
 * - Divide the bag with 6 balls into two bags of sizes 3 and 3. [<u>6</u>,3] -> [3,3,3].
 * The bag with the most number of balls has 3 balls, so your penalty is 3 and you should return 3.
 *
 * Example 2:
 *
 * Input: nums = [2,4,8,2], maxOperations = 4
 * Output: 2
 * Explanation:
 * - Divide the bag with 8 balls into two bags of sizes 4 and 4. [2,4,<u>8</u>,2] -> [2,4,4,4,2].
 * - Divide the bag with 4 balls into two bags of sizes 2 and 2. [2,<u>4</u>,4,4,2] -> [2,2,2,4,4,2].
 * - Divide the bag with 4 balls into two bags of sizes 2 and 2. [2,2,2,<u>4</u>,4,2] -> [2,2,2,2,2,4,2].
 * - Divide the bag with 4 balls into two bags of sizes 2 and 2. [2,2,2,2,2,<u>4</u>,2] -> [2,2,2,2,2,2,2,2].
 * The bag with the most number of balls has 2 balls, so your penalty is 2, and you should return 2.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= maxOperations, nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-limit-of-balls-in-a-bag/
// discuss: https://leetcode.com/problems/minimum-limit-of-balls-in-a-bag/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let (mut min, mut max) = (1, *nums.iter().max().unwrap());

        let mut penalty;

        while min < max {
            penalty = (min + max) / 2;
            if Self::can_fit_operations(penalty, &nums, max_operations) {
                max = penalty;
            } else {
                min = penalty + 1;
            }
        }

        min
    }
    fn can_fit_operations(penalty: i32, nums: &Vec<i32>, max: i32) -> bool {
        let mut result = 0;

        for num in nums {
            result += (num - 1) / penalty;
            if result > max {
                return false;
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1760_example_1() {
        let nums = vec![9];
        let max_operations = 2;

        let result = 3;

        assert_eq!(Solution::minimum_size(nums, max_operations), result);
    }

    #[test]
    fn test_1760_example_2() {
        let nums = vec![2, 4, 8, 2];
        let max_operations = 4;

        let result = 2;

        assert_eq!(Solution::minimum_size(nums, max_operations), result);
    }
}
