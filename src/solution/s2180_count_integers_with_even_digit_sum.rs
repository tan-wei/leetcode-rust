/**
 * [2180] Count Integers With Even Digit Sum
 *
 * Given a positive integer num, return the number of positive integers less than or equal to num whose digit sums are even.
 * The digit sum of a positive integer is the sum of all its digits.
 *  
 * Example 1:
 *
 * Input: num = 4
 * Output: 2
 * Explanation:
 * The only integers less than or equal to 4 whose digit sums are even are 2 and 4.    
 *
 * Example 2:
 *
 * Input: num = 30
 * Output: 14
 * Explanation:
 * The 14 integers less than or equal to 30 whose digit sums are even are
 * 2, 4, 6, 8, 11, 13, 15, 17, 19, 20, 22, 24, 26, and 28.
 *
 *  
 * Constraints:
 *
 * 	1 <= num <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-integers-with-even-digit-sum/
// discuss: https://leetcode.com/problems/count-integers-with-even-digit-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        (2..=num)
            .filter(|i| {
                let (mut sum, mut x) = (0, *i);
                while x > 0 {
                    sum += x % 10;
                    x /= 10;
                }
                (sum & 1).eq(&0)
            })
            .count() as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2180_example_1() {
        let num = 4;

        let result = 2;

        assert_eq!(Solution::count_even(num), result);
    }

    #[test]
    fn test_2180_example_2() {
        let num = 30;

        let result = 14;

        assert_eq!(Solution::count_even(num), result);
    }
}
