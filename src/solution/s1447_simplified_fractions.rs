/**
 * [1447] Simplified Fractions
 *
 * Given an integer n, return a list of all simplified fractions between 0 and 1 (exclusive) such that the denominator is less-than-or-equal-to n. You can return the answer in any order.
 *  
 * Example 1:
 *
 * Input: n = 2
 * Output: ["1/2"]
 * Explanation: "1/2" is the only unique fraction with a denominator less-than-or-equal-to 2.
 *
 * Example 2:
 *
 * Input: n = 3
 * Output: ["1/2","1/3","2/3"]
 *
 * Example 3:
 *
 * Input: n = 4
 * Output: ["1/2","1/3","1/4","2/3","3/4"]
 * Explanation: "2/4" is not a simplified fraction because it can be simplified to "1/2".
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/simplified-fractions/
// discuss: https://leetcode.com/problems/simplified-fractions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        let mut result = vec![];

        for i in 2..n + 1 {
            for j in 1..i {
                if Self::gcd(i, j) == 1 {
                    result.push(j.to_string() + "/" + &i.to_string());
                }
            }
        }

        result
    }
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1447_example_1() {
        let n = 2;

        let result = vec_string!["1/2"];

        assert_eq_sorted!(Solution::simplified_fractions(n), result);
    }

    #[test]
    fn test_1447_example_2() {
        let n = 3;

        let result = vec_string!["1/2", "1/3", "2/3"];

        assert_eq_sorted!(Solution::simplified_fractions(n), result);
    }

    #[test]
    fn test_1447_example_3() {
        let n = 4;

        let result = vec_string!["1/2", "1/3", "1/4", "2/3", "3/4"];

        assert_eq_sorted!(Solution::simplified_fractions(n), result);
    }
}
