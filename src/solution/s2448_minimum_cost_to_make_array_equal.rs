/**
 * [2448] Minimum Cost to Make Array Equal
 *
 * You are given two 0-indexed arrays nums and cost consisting each of n positive integers.
 * You can do the following operation any number of times:
 *
 * 	Increase or decrease any element of the array nums by 1.
 *
 * The cost of doing one operation on the i^th element is cost[i].
 * Return the minimum total cost such that all the elements of the array nums become equal.
 *  
 * Example 1:
 *
 * Input: nums = [1,3,5,2], cost = [2,3,1,14]
 * Output: 8
 * Explanation: We can make all the elements equal to 2 in the following way:
 * - Increase the 0^th element one time. The cost is 2.
 * - Decrease the 1^<span style="font-size: 10.8333px;">st</span> element one time. The cost is 3.
 * - Decrease the 2^nd element three times. The cost is 1 + 1 + 1 = 3.
 * The total cost is 2 + 3 + 3 = 8.
 * It can be shown that we cannot make the array equal with a smaller cost.
 *
 * Example 2:
 *
 * Input: nums = [2,2,2,2,2], cost = [4,2,8,1,3]
 * Output: 0
 * Explanation: All the elements are already equal, so no operations are needed.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length == cost.length
 * 	1 <= n <= 10^5
 * 	1 <= nums[i], cost[i] <= 10^6
 * 	Test cases are generated in a way that the output doesn't exceed 2^53-1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-to-make-array-equal/
// discuss: https://leetcode.com/problems/minimum-cost-to-make-array-equal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2448_example_1() {
        let nums = vec![1, 3, 5, 2];
        let cost = vec![2, 3, 1, 14];

        let result = 8;

        assert_eq!(Solution::min_cost(nums, cost), result);
    }

    #[test]
    #[ignore]
    fn test_2448_example_2() {
        let nums = vec![2, 2, 2, 2, 2];
        let cost = vec![4, 2, 8, 1, 3];

        let result = 0;

        assert_eq!(Solution::min_cost(nums, cost), result);
    }
}
