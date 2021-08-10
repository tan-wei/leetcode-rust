/**
 * [264] Ugly Number II
 *
 * An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
 * Given an integer n, return the n^th ugly number.
 *  
 * Example 1:
 *
 * Input: n = 10
 * Output: 12
 * Explanation: [1, 2, 3, 4, 5, 6, 8, 9, 10, 12] is the sequence of the first 10 ugly numbers.
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 1
 * Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 1690
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ugly-number-ii/
// discuss: https://leetcode.com/problems/ugly-number-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly_numbers = vec![1];
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        while (ugly_numbers.len() < n as usize) {
            let last = std::cmp::min(
                ugly_numbers[x] * 2,
                std::cmp::min(ugly_numbers[y] * 3, ugly_numbers[z] * 5),
            );

            ugly_numbers.push(last);

            if (last == ugly_numbers[x] * 2) {
                x += 1;
            }
            if (last == ugly_numbers[y] * 3) {
                y += 1;
            }
            if (last == ugly_numbers[z] * 5) {
                z += 1;
            }
        }

        *ugly_numbers.last().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0264_example_1() {
        let n = 10;
        let result = 12;

        assert_eq!(Solution::nth_ugly_number(n), result);
    }

    #[test]
    fn test_0264_example_2() {
        let n = 1;
        let result = 1;

        assert_eq!(Solution::nth_ugly_number(n), result);
    }
}
