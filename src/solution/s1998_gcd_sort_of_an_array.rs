/**
 * [1998] GCD Sort of an Array
 *
 * You are given an integer array nums, and you can perform the following operation any number of times on nums:
 *
 * 	Swap the positions of two elements nums[i] and nums[j] if gcd(nums[i], nums[j]) > 1 where gcd(nums[i], nums[j]) is the greatest common divisor of nums[i] and nums[j].
 *
 * Return true if it is possible to sort nums in non-decreasing order using the above swap method, or false otherwise.
 *  
 * Example 1:
 *
 * Input: nums = [7,21,3]
 * Output: true
 * Explanation: We can sort [7,21,3] by performing the following operations:
 * - Swap 7 and 21 because gcd(7,21) = 7. nums = [<u>21</u>,<u>7</u>,3]
 * - Swap 21 and 3 because gcd(21,3) = 3. nums = [<u>3</u>,7,<u>21</u>]
 *
 * Example 2:
 *
 * Input: nums = [5,2,6,2]
 * Output: false
 * Explanation: It is impossible to sort the array because 5 cannot be swapped with any other element.
 *
 * Example 3:
 *
 * Input: nums = [10,5,9,3,15]
 * Output: true
 * We can sort [10,5,9,3,15] by performing the following operations:
 * - Swap 10 and 15 because gcd(10,15) = 5. nums = [<u>15</u>,5,9,3,<u>10</u>]
 * - Swap 15 and 3 because gcd(15,3) = 3. nums = [<u>3</u>,5,9,<u>15</u>,10]
 * - Swap 10 and 15 because gcd(10,15) = 5. nums = [3,5,9,<u>10</u>,<u>15</u>]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 3 * 10^4
 * 	2 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/gcd-sort-of-an-array/
// discuss: https://leetcode.com/problems/gcd-sort-of-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn gcd_sort(nums: Vec<i32>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1998_example_1() {
        let nums = vec![7, 21, 3];

        let result = true;

        assert_eq!(Solution::gcd_sort(nums), result);
    }

    #[test]
    #[ignore]
    fn test_1998_example_2() {
        let nums = vec![5, 2, 6, 2];

        let result = false;

        assert_eq!(Solution::gcd_sort(nums), result);
    }

    #[test]
    #[ignore]
    fn test_1998_example_3() {
        let nums = vec![10, 5, 9, 3, 15];

        let result = true;

        assert_eq!(Solution::gcd_sort(nums), result);
    }
}
