/**
 * [0507] Perfect Number
 *
 * A <a href="https://en.wikipedia.org/wiki/Perfect_number" target="_blank">perfect number</a> is a positive integer that is equal to the sum of its positive divisors, excluding the number itself. A divisor of an integer x is an integer that can divide x evenly.
 * Given an integer n, return true if n is a perfect number, otherwise return false.
 *  
 * Example 1:
 *
 * Input: num = 28
 * Output: true
 * Explanation: 28 = 1 + 2 + 4 + 7 + 14
 * 1, 2, 4, 7, and 14 are all divisors of 28.
 *
 * Example 2:
 *
 * Input: num = 7
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= num <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/perfect-number/
// discuss: https://leetcode.com/problems/perfect-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        num >= 2
            && (1..)
                .take_while(|x| x * x <= num)
                .filter(|x| num % x == 0)
                .fold(0, |acc, x| {
                    acc + {
                        if x * x == num || x == 1 {
                            x
                        } else {
                            x + num / x
                        }
                    }
                })
                == num
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0507_example_1() {
        let num = 28;
        let result = true;

        assert_eq!(Solution::check_perfect_number(num), result);
    }

    #[test]
    fn test_0507_example_2() {
        let num = 7;
        let result = false;

        assert_eq!(Solution::check_perfect_number(num), result);
    }
}
