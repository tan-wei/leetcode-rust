/**
 * [279] Perfect Squares
 *
 * Given an integer n, return the least number of perfect square numbers that sum to n.
 * A perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself. For example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.
 *  
 * Example 1:
 *
 * Input: n = 12
 * Output: 3
 * Explanation: 12 = 4 + 4 + 4.
 *
 * Example 2:
 *
 * Input: n = 13
 * Output: 2
 * Explanation: 13 = 4 + 9.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/perfect-squares/
// discuss: https://leetcode.com/problems/perfect-squares/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // NOTE: https://en.wikipedia.org/wiki/Lagrange%27s_four-square_theorem
    pub fn num_squares(n: i32) -> i32 {
        let is_sq = |x| {
            let k = (x as f64).sqrt() as i32;
            k * k == x
        };

        if is_sq(n) {
            return 1;
        }

        // 4^a(8b+7)
        let mut x = n;
        while x % 4 == 0 {
            x >>= 2;
        }
        if x % 8 == 7 {
            return 4;
        }
        let k = (n as f64).sqrt() as i32;
        for i in 1..=k {
            if is_sq(n - i * i) {
                return 2;
            }
        }
        3
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0279_example_1() {
        let n = 12;
        let result = 3;

        assert_eq!(Solution::num_squares(n), result);
    }

    #[test]
    fn test_0279_example_2() {
        let n = 13;
        let result = 2;

        assert_eq!(Solution::num_squares(n), result);
    }
}
