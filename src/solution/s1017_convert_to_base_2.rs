/**
 * [1017] Convert to Base -2
 *
 * Given an integer n, return a binary string representing its representation in base -2.
 * Note that the returned string should not have leading zeros unless the string is "0".
 *  
 * Example 1:
 *
 * Input: n = 2
 * Output: "110"
 * Explantion: (-2)^2 + (-2)^1 = 2
 *
 * Example 2:
 *
 * Input: n = 3
 * Output: "111"
 * Explantion: (-2)^2 + (-2)^1 + (-2)^0 = 3
 *
 * Example 3:
 *
 * Input: n = 4
 * Output: "100"
 * Explantion: (-2)^2 = 4
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/convert-to-base-2/
// discuss: https://leetcode.com/problems/convert-to-base-2/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn base_neg2(n: i32) -> String {
        let mut n = n;
        let mut test_number = 2i64;
        while test_number <= n as i64 {
            if (n & test_number as i32) != 0 {
                n += (test_number << 1) as i32;
            }
            test_number <<= 2;
        }
        format!("{:b}", n)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1017_example_1() {
        let n = 2;
        let result = "110".to_string();

        assert_eq!(Solution::base_neg2(n), result);
    }

    #[test]
    fn test_1017_example_2() {
        let n = 3;
        let result = "111".to_string();

        assert_eq!(Solution::base_neg2(n), result);
    }

    #[test]
    fn test_1017_example_3() {
        let n = 4;
        let result = "100".to_string();

        assert_eq!(Solution::base_neg2(n), result);
    }
}
