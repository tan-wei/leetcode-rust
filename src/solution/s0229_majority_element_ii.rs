/**
 * [229] Majority Element II
 *
 * Given an integer array of size n, find all elements that appear more than &lfloor; n/3 &rfloor; times.
 * Follow-up: Could you solve the problem in linear time and in O(1) space?
 *  
 * Example 1:
 *
 * Input: nums = [3,2,3]
 * Output: [3]
 *
 * Example 2:
 *
 * Input: nums = [1]
 * Output: [1]
 *
 * Example 3:
 *
 * Input: nums = [1,2]
 * Output: [1,2]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5 * 10^4
 * 	-10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/majority-element-ii/
// discuss: https://leetcode.com/problems/majority-element-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut candi1 = 0;
        let mut candi2 = 1;
        let mut count1 = 0;
        let mut count2 = 0;
        for num in nums.iter() {
            if *num == candi1 {
                count1 += 1;
            } else if *num == candi2 {
                count2 += 1;
            } else if count1 == 0 {
                candi1 = *num;
                count1 = 1;
            } else if count2 == 0 {
                candi2 = *num;
                count2 = 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }

        count1 = 0;
        count2 = 0;
        for num in nums.iter() {
            if *num == candi1 {
                count1 += 1;
            } else if *num == candi2 {
                count2 += 1;
            }
        }

        let mut result = vec![];

        if count1 > nums.len() / 3 {
            result.push(candi1);
        }
        if count2 > nums.len() / 3 {
            result.push(candi2);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0229_example_1() {
        let nums = vec![3, 2, 3];
        let result = vec![3];

        assert_eq_sorted!(Solution::majority_element(nums), result);
    }

    #[test]
    fn test_0229_example_2() {
        let nums = vec![1];
        let result = vec![1];

        assert_eq_sorted!(Solution::majority_element(nums), result);
    }

    #[test]
    fn test_0229_example_3() {
        let nums = vec![1, 2];
        let result = vec![1, 2];

        assert_eq_sorted!(Solution::majority_element(nums), result);
    }
}
