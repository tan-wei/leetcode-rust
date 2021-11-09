/**
 * [0402] Remove K Digits
 *
 * Given string num representing a non-negative integer num, and an integer k, return the smallest possible integer after removing k digits from num.
 *  
 * Example 1:
 *
 * Input: num = "1432219", k = 3
 * Output: "1219"
 * Explanation: Remove the three digits 4, 3, and 2 to form the new number 1219 which is the smallest.
 *
 * Example 2:
 *
 * Input: num = "10200", k = 1
 * Output: "200"
 * Explanation: Remove the leading 1 and the number is 200. Note that the output must not contain leading zeroes.
 *
 * Example 3:
 *
 * Input: num = "10", k = 2
 * Output: "0"
 * Explanation: Remove all the digits from the number and it is left with nothing which is 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= num.length <= 10^5
 * 	num consists of only digits.
 * 	num does not have any leading zeros except for the zero itself.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-k-digits/
// discuss: https://leetcode.com/problems/remove-k-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/remove-k-digits/discuss/629739/Rust-0ms-code-with-comments
    pub fn remove_kdigits(num: String, k: i32) -> String {
        // Treat string `res` as a stack
        let (mut k, mut res) = (k as usize, "".to_string());

        for n in num.chars() {
            // `res.pop()` to make sure digits in `res` are in ascending order
            while k > 0 && !res.is_empty() && n < res.chars().last().unwrap() {
                k -= 1;
                res.pop();
            }
            // removing leading zeros
            if !res.is_empty() || n != '0' {
                res.push(n);
            }
        }

        // make sure remove k digits in total
        while !res.is_empty() && k > 0 {
            res.pop();
            k -= 1;
        }

        if !res.is_empty() {
            res
        } else {
            "0".to_string()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0402_example_1() {
        let num = "1432219".to_string();
        let k = 3;
        let result = "1219".to_string();

        assert_eq!(Solution::remove_kdigits(num, k), result);
    }

    #[test]
    fn test_0402_example_2() {
        let num = "10200".to_string();
        let k = 1;
        let result = "200".to_string();

        assert_eq!(Solution::remove_kdigits(num, k), result);
    }

    #[test]
    fn test_0402_example_3() {
        let num = "10".to_string();
        let k = 2;
        let result = "0".to_string();

        assert_eq!(Solution::remove_kdigits(num, k), result);
    }
}
