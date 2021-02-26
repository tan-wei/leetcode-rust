/**
 * [50] Pow(x, n)
 *
 * Implement <a href="http://www.cplusplus.com/reference/valarray/pow/" target="_blank">pow(x, n)</a>, which calculates x raised to the power n (i.e. x^<span style="font-size:10.8333px">n</span>).
 *  
 * Example 1:
 *
 * Input: x = 2.00000, n = 10
 * Output: 1024.00000
 *
 * Example 2:
 *
 * Input: x = 2.10000, n = 3
 * Output: 9.26100
 *
 * Example 3:
 *
 * Input: x = 2.00000, n = -2
 * Output: 0.25000
 * Explanation: 2^-2 = 1/2^2 = 1/4 = 0.25
 *
 *  
 * Constraints:
 *
 * 	-100.0 < x < 100.0
 * 	-2^31 <= n <= 2^31-1
 * 	-10^4 <= x^n <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/powx-n/
// discuss: https://leetcode.com/problems/powx-n/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        match n {
            0 => {
                if x != 0_f64 {
                    1.0
                } else {
                    f64::NAN
                }
            }
            1 => x,
            -1 => 1.0 / x,
            _ => {
                Solution::my_pow(x, n / 2) * Solution::my_pow(x, n / 2) * Solution::my_pow(x, n % 2)
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0050_example_1() {
        let x = 2.00000;
        let n = 10;
        let result = 1024.00000;

        assert_f64_near!(Solution::my_pow(x, n), result, 6);
    }

    #[test]
    fn test_0050_example_2() {
        let x = 2.10000;
        let n = 3;
        let result = 9.26100;

        assert_f64_near!(Solution::my_pow(x, n), result, 6);
    }

    #[test]
    fn test_0050_example_3() {
        let x = 2.00000;
        let n = -2;
        let result = 0.25;

        assert_f64_near!(Solution::my_pow(x, n), result, 6);
    }
}
