/**
 * [2520] Count the Digits That Divide a Number
 *
 * Given an integer num, return the number of digits in num that divide num.
 * An integer val divides nums if nums % val == 0.
 *  
 * Example 1:
 *
 * Input: num = 7
 * Output: 1
 * Explanation: 7 divides itself, hence the answer is 1.
 *
 * Example 2:
 *
 * Input: num = 121
 * Output: 2
 * Explanation: 121 is divisible by 1, but not 2. Since 1 occurs twice as a digit, we return 2.
 *
 * Example 3:
 *
 * Input: num = 1248
 * Output: 4
 * Explanation: 1248 is divisible by all of its digits, hence the answer is 4.
 *
 *  
 * Constraints:
 *
 * 	1 <= num <= 10^9
 * 	num does not contain 0 as one of its digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-digits-that-divide-a-number/
// discuss: https://leetcode.com/problems/count-the-digits-that-divide-a-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        num.to_string()
            .bytes()
            .filter(|x| num % (x - 48) as i32 == 0)
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2520_example_1() {
        let num = 7;

        let result = 1;

        assert_eq!(Solution::count_digits(num), result);
    }

    #[test]
    fn test_2520_example_2() {
        let num = 121;

        let result = 2;

        assert_eq!(Solution::count_digits(num), result);
    }

    #[test]
    fn test_2520_example_3() {
        let num = 1248;

        let result = 4;

        assert_eq!(Solution::count_digits(num), result);
    }
}
