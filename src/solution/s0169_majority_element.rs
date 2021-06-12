/**
 * [169] Majority Element
 *
 * Given an array nums of size n, return the majority element.
 * The majority element is the element that appears more than &lfloor;n / 2&rfloor; times. You may assume that the majority element always exists in the array.
 *  
 * Example 1:
 * Input: nums = [3,2,3]
 * Output: 3
 * Example 2:
 * Input: nums = [2,2,1,1,1,2,2]
 * Output: 2
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 5 * 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 *
 *  
 * Follow-up: Could you solve the problem in linear time and in O(1) space?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/majority-element/
// discuss: https://leetcode.com/problems/majority-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candi = 0;
        let mut count = 0;

        for num in nums.iter() {
            if num == &candi {
                count += 1;
            } else if count == 0 {
                candi = *num;
                count = 1;
            } else {
                count -= 1;
            }
        }

        candi
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0169_example_1() {
        let nums = vec![3, 2, 3];
        let result = 3;

        assert_eq!(Solution::majority_element(nums), result);
    }

    #[test]
    fn test_0169_example_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let result = 2;

        assert_eq!(Solution::majority_element(nums), result);
    }
}
