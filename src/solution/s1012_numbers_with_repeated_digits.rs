/**
 * [1012] Numbers With Repeated Digits
 *
 * Given an integer n, return the number of positive integers in the range [1, n] that have at least one repeated digit.
 *  
 * Example 1:
 *
 * Input: n = 20
 * Output: 1
 * Explanation: The only positive number (<= 20) with at least 1 repeated digit is 11.
 *
 * Example 2:
 *
 * Input: n = 100
 * Output: 10
 * Explanation: The positive numbers (<= 100) with atleast 1 repeated digit are 11, 22, 33, 44, 55, 66, 77, 88, 99, and 100.
 *
 * Example 3:
 *
 * Input: n = 1000
 * Output: 262
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/numbers-with-repeated-digits/
// discuss: https://leetcode.com/problems/numbers-with-repeated-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        n - Self::num_not_dup_digits_at_most_n(n)
    }

    fn num_not_dup_digits_at_most_n(n: i32) -> i32 {
        let mut n = n;
        let mut digits = Vec::new();

        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        let k = digits.len();

        let mut used: [i32; 10] = [0; 10];
        let mut total = 0;

        for i in 1..k {
            total += 9 * Self::permutation(9, i as i32 - 1);
        }

        for i in 0..k {
            let i = k - 1 - i;
            let num = digits[i];

            for j in (if i == k - 1 { 1 } else { 0 })..num {
                if used[j as usize] != 0 {
                    continue;
                }
                total += Self::permutation((10 - k + i) as i32, i as i32);
            }

            used[num as usize] += 1;
            if used[num as usize] > 1 {
                break;
            }

            if i == 0 {
                total += 1;
            }
        }

        total
    }

    fn permutation(n: i32, k: i32) -> i32 {
        Self::factorial(n) / Self::factorial(n - k)
    }

    fn factorial(n: i32) -> i32 {
        match n {
            0 | 1 => 1,
            n @ _ => n * Self::factorial(n - 1),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1012_example_1() {
        let n = 20;
        let result = 1;

        assert_eq!(Solution::num_dup_digits_at_most_n(n), result);
    }

    #[test]
    fn test_1012_example_2() {
        let n = 100;
        let result = 10;

        assert_eq!(Solution::num_dup_digits_at_most_n(n), result);
    }

    #[test]
    fn test_1012_example_3() {
        let n = 1000;
        let result = 262;

        assert_eq!(Solution::num_dup_digits_at_most_n(n), result);
    }
}
