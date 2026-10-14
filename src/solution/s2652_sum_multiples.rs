/**
 * [2652] Sum Multiples
 *
 * Given a positive integer n, find the sum of all integers in the range [1, n] inclusive that are divisible by 3, 5, or 7.
 * Return an integer denoting the sum of all numbers in the given range satisfying the constraint.
 *  
 * Example 1:
 *
 * Input: n = 7
 * Output: 21
 * Explanation: Numbers in the range [1, 7] that are divisible by 3, 5, or 7 are 3, 5, 6, 7. The sum of these numbers is 21.
 *
 * Example 2:
 *
 * Input: n = 10
 * Output: 40
 * Explanation: Numbers in the range [1, 10] that are divisible by 3, 5, or 7 are 3, 5, 6, 7, 9, 10. The sum of these numbers is 40.
 *
 * Example 3:
 *
 * Input: n = 9
 * Output: 30
 * Explanation: Numbers in the range [1, 9] that are divisible by 3, 5, or 7 are 3, 5, 6, 7, 9. The sum of these numbers is 30.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^3
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-multiples/
// discuss: https://leetcode.com/problems/sum-multiples/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        let mut sum = 0;
        let mut num = n;

        while num > 0 {
            if num % 3 == 0 || num % 5 == 0 || num % 7 == 0 {
                sum += num;
            }
            num -= 1;
        }

        sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2652_example_1() {
        let n = 7;

        let result = 21;

        assert_eq!(Solution::sum_of_multiples(n), result);
    }

    #[test]
    fn test_2652_example_2() {
        let n = 10;

        let result = 40;

        assert_eq!(Solution::sum_of_multiples(n), result);
    }

    #[test]
    fn test_2652_example_3() {
        let n = 9;

        let result = 30;

        assert_eq!(Solution::sum_of_multiples(n), result);
    }
}
