/**
 * [2239] Find Closest Number to Zero
 *
 * Given an integer array nums of size n, return the number with the value closest to 0 in nums. If there are multiple answers, return the number with the largest value.
 *  
 * Example 1:
 *
 * Input: nums = [-4,-2,1,4,8]
 * Output: 1
 * Explanation:
 * The distance from -4 to 0 is |-4| = 4.
 * The distance from -2 to 0 is |-2| = 2.
 * The distance from 1 to 0 is |1| = 1.
 * The distance from 4 to 0 is |4| = 4.
 * The distance from 8 to 0 is |8| = 8.
 * Thus, the closest number to 0 in the array is 1.
 *
 * Example 2:
 *
 * Input: nums = [2,-1,1]
 * Output: 1
 * Explanation: 1 and -1 are both the closest numbers to 0, so 1 being larger is returned.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 1000
 * 	-10^5 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-closest-number-to-zero/
// discuss: https://leetcode.com/problems/find-closest-number-to-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut diff = i32::MAX;
        let mut result = 0;

        for num in nums {
            match diff.cmp(&num.abs()) {
                std::cmp::Ordering::Greater => {
                    diff = num.abs();
                    result = num;
                }
                std::cmp::Ordering::Equal if num > result => result = num,
                _ => {}
            };
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2239_example_1() {
        let nums = vec![-4, -2, 1, 4, 8];

        let result = 1;

        assert_eq!(Solution::find_closest_number(nums), result);
    }

    #[test]
    fn test_2239_example_2() {
        let nums = vec![2, -1, 1];

        let result = 1;

        assert_eq!(Solution::find_closest_number(nums), result);
    }
}
