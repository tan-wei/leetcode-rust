/**
 * [357] Count Numbers with Unique Digits
 *
 * Given an integer n, return the count of all numbers with unique digits, x, where 0 <= x < 10^n.
 *  
 * Example 1:
 *
 * Input: n = 2
 * Output: 91
 * Explanation: The answer should be the total numbers in the range of 0 &le; x < 100, excluding 11,22,33,44,55,66,77,88,99
 *
 * Example 2:
 *
 * Input: n = 0
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-numbers-with-unique-digits/
// discuss: https://leetcode.com/problems/count-numbers-with-unique-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let factorials = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
        let mut n = n.min(10) as usize;
        let mut ret = 1;

        while n > 0 {
            ret += 9 * factorials[9] / factorials[10 - n];
            n -= 1;
        }

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0357_example_1() {
        let n = 2;
        let result = 91;

        assert_eq!(Solution::count_numbers_with_unique_digits(n), result);
    }

    #[test]
    fn test_0357_example_2() {
        let n = 0;
        let result = 1;

        assert_eq!(Solution::count_numbers_with_unique_digits(n), result);
    }
}
