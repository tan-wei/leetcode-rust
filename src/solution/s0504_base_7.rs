/**
 * [0504] Base 7
 *
 * Given an integer num, return a string of its base 7 representation.
 *  
 * Example 1:
 * Input: num = 100
 * Output: "202"
 * Example 2:
 * Input: num = -7
 * Output: "-10"
 *  
 * Constraints:
 *
 * 	-10^7 <= num <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/base-7/
// discuss: https://leetcode.com/problems/base-7/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let mut num = num;
        if num == 0 {
            return "0".to_string();
        }
        let mut s = String::new();
        let sign = if num < 0 {
            num *= -1;
            "-".to_string()
        } else {
            "".to_string()
        };
        while num > 0 {
            s = (num % 7).to_string() + &s;
            num /= 7;
        }
        sign + &s
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0504_example_1() {
        let num = 100;
        let result = "202";

        assert_eq!(Solution::convert_to_base7(num), result);
    }

    #[test]
    fn test_0504_example_2() {
        let num = -7;
        let result = "-10";

        assert_eq!(Solution::convert_to_base7(num), result);
    }
}
