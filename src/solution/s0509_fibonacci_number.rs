/**
 * [0509] Fibonacci Number
 *
 * The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. That is,
 *
 * F(0) = 0, F(1) = 1
 * F(n) = F(n - 1) + F(n - 2), for n > 1.
 *
 * Given n, calculate F(n).
 *  
 * Example 1:
 *
 * Input: n = 2
 * Output: 1
 * Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
 *
 * Example 2:
 *
 * Input: n = 3
 * Output: 2
 * Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
 *
 * Example 3:
 *
 * Input: n = 4
 * Output: 3
 * Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 30
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/fibonacci-number/
// discuss: https://leetcode.com/problems/fibonacci-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            n if n > 0 => {
                let mut a = 0;
                let mut b = 1;

                for i in 2..=n {
                    let f = a + b;
                    a = b;
                    b = f;
                }

                b
            }
            _ => panic!("n should be not nagtive!"),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0509_example_1() {
        let n = 2;
        let result = 1;

        assert_eq!(Solution::fib(n), result);
    }

    #[test]
    fn test_0509_example_2() {
        let n = 3;
        let result = 2;

        assert_eq!(Solution::fib(n), result);
    }

    #[test]
    fn test_0509_example_3() {
        let n = 4;
        let result = 3;

        assert_eq!(Solution::fib(n), result);
    }
}
