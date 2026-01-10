/**
 * [1822] Sign of the Product of an Array
 *
 * Implement a function signFunc(x) that returns:
 *
 * 	1 if x is positive.
 * 	-1 if x is negative.
 * 	0 if x is equal to 0.
 *
 * You are given an integer array nums. Let product be the product of all values in the array nums.
 * Return signFunc(product).
 *  
 * Example 1:
 *
 * Input: nums = [-1,-2,-3,-4,3,2,1]
 * Output: 1
 * Explanation: The product of all values in the array is 144, and signFunc(144) = 1
 *
 * Example 2:
 *
 * Input: nums = [1,5,0,2,-3]
 * Output: 0
 * Explanation: The product of all values in the array is 0, and signFunc(0) = 0
 *
 * Example 3:
 *
 * Input: nums = [-1,1,-1,1,-1]
 * Output: -1
 * Explanation: The product of all values in the array is -1, and signFunc(-1) = -1
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	-100 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sign-of-the-product-of-an-array/
// discuss: https://leetcode.com/problems/sign-of-the-product-of-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.iter().fold(1, |acc, num| {
            if num < &0 {
                acc * -1
            } else {
                if num == &0 { acc * 0 } else { acc * 1 }
            }
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1822_example_1() {
        let nums = vec![-1, -2, -3, -4, 3, 2, 1];

        let result = 1;

        assert_eq!(Solution::array_sign(nums), result);
    }

    #[test]
    fn test_1822_example_2() {
        let nums = vec![1, 5, 0, 2, -3];

        let result = 0;

        assert_eq!(Solution::array_sign(nums), result);
    }

    #[test]
    fn test_1822_example_3() {
        let nums = vec![-1, 1, -1, 1, -1];

        let result = -1;

        assert_eq!(Solution::array_sign(nums), result);
    }
}
