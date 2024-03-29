/**
 * [0902] Numbers At Most N Given Digit Set
 *
 * Given an array of digits which is sorted in non-decreasing order. You can write numbers using each digits[i] as many times as we want. For example, if digits = ['1','3','5'], we may write numbers such as '13', '551', and '1351315'.
 * Return the number of positive integers that can be generated that are less than or equal to a given integer n.
 *  
 * Example 1:
 *
 * Input: digits = ["1","3","5","7"], n = 100
 * Output: 20
 * Explanation:
 * The 20 numbers that can be written are:
 * 1, 3, 5, 7, 11, 13, 15, 17, 31, 33, 35, 37, 51, 53, 55, 57, 71, 73, 75, 77.
 *
 * Example 2:
 *
 * Input: digits = ["1","4","9"], n = 1000000000
 * Output: 29523
 * Explanation:
 * We can write 3 one digit numbers, 9 two digit numbers, 27 three digit numbers,
 * 81 four digit numbers, 243 five digit numbers, 729 six digit numbers,
 * 2187 seven digit numbers, 6561 eight digit numbers, and 19683 nine digit numbers.
 * In total, this is 29523 integers that can be written using the digits array.
 *
 * Example 3:
 *
 * Input: digits = ["7"], n = 8
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= digits.length <= 9
 * 	digits[i].length == 1
 * 	digits[i] is a digit from '1' to '9'.
 * 	All the values in digits are unique.
 * 	digits is sorted in non-decreasing order.
 * 	1 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/numbers-at-most-n-given-digit-set/
// discuss: https://leetcode.com/problems/numbers-at-most-n-given-digit-set/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let digits: Vec<char> = digits.iter().map(|s| s.chars().next().unwrap()).collect();
        let n = n.to_string();
        let mut dp = vec![0; n.len() + 1];
        dp[n.len()] = 1;
        for (i, c) in n.chars().rev().enumerate() {
            for &d in digits.iter() {
                match d.cmp(&c) {
                    std::cmp::Ordering::Less => dp[n.len() - i - 1] += digits.len().pow(i as u32),
                    std::cmp::Ordering::Equal => dp[n.len() - i - 1] += dp[n.len() - i],
                    std::cmp::Ordering::Greater => {}
                }
            }
        }
        (dp[0]
            + (1..n.len())
                .map(|i| digits.len().pow(i as u32))
                .sum::<usize>()) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0902_example_1() {
        let digits = vec_string!["1", "3", "5", "7"];
        let n = 100;
        let result = 20;

        assert_eq!(Solution::at_most_n_given_digit_set(digits, n), result);
    }

    #[test]
    fn test_0902_example_2() {
        let digits = vec_string!["1", "4", "9"];
        let n = 1000000000;
        let result = 29523;

        assert_eq!(Solution::at_most_n_given_digit_set(digits, n), result);
    }

    #[test]
    fn test_0902_example_3() {
        let digits = vec_string!["7"];
        let n = 8;
        let result = 1;

        assert_eq!(Solution::at_most_n_given_digit_set(digits, n), result);
    }
}
