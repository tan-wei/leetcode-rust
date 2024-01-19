/**
 * [1464] Maximum Product of Two Elements in an Array
 *
 * Given the array of integers nums, you will choose two different indices i and j of that array. Return the maximum value of (nums[i]-1)*(nums[j]-1).
 *  
 * Example 1:
 *
 * Input: nums = [3,4,5,2]
 * Output: 12
 * Explanation: If you choose the indices i=1 and j=2 (indexed from 0), you will get the maximum value, that is, (nums[1]-1)*(nums[2]-1) = (4-1)*(5-1) = 3*4 = 12.
 *
 * Example 2:
 *
 * Input: nums = [1,5,4,5]
 * Output: 16
 * Explanation: Choosing the indices i=1 and j=3 (indexed from 0), you will get the maximum value of (5-1)*(5-1) = 16.
 *
 * Example 3:
 *
 * Input: nums = [3,7]
 * Output: 12
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 500
 * 	1 <= nums[i] <= 10^3
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/
// discuss: https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold([0, 0], |mut m, x| {
                if x > &m[0] {
                    m[1] = m[0];
                    m[0] = *x;
                } else if x == &m[0] {
                    m[1] = *x;
                } else {
                    if x > &m[1] {
                        m[1] = *x;
                    }
                }
                m
            })
            .iter()
            .fold(1, |mut res, x| res * (x - 1))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1464_example_1() {
        let nums = vec![3, 4, 5, 2];

        let result = 12;

        assert_eq!(Solution::max_product(nums), result);
    }

    #[test]
    fn test_1464_example_2() {
        let nums = vec![1, 5, 4, 5];

        let result = 16;

        assert_eq!(Solution::max_product(nums), result);
    }

    #[test]
    fn test_1464_example_3() {
        let nums = vec![3, 7];

        let result = 12;

        assert_eq!(Solution::max_product(nums), result);
    }
}
