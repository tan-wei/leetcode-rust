/**
 * [0753] Cracking the Safe
 *
 * There is a safe protected by a password. The password is a sequence of n digits where each digit can be in the range [0, k - 1].
 * The safe has a peculiar way of checking the password. When you enter in a sequence, it checks the most recent n digits that were entered each time you type a digit.
 *
 * 	For example, the correct password is "345" and you enter in "012345":
 *
 * 		After typing 0, the most recent 3 digits is "0", which is incorrect.
 * 		After typing 1, the most recent 3 digits is "01", which is incorrect.
 * 		After typing 2, the most recent 3 digits is "012", which is incorrect.
 * 		After typing 3, the most recent 3 digits is "123", which is incorrect.
 * 		After typing 4, the most recent 3 digits is "234", which is incorrect.
 * 		After typing 5, the most recent 3 digits is "345", which is correct and the safe unlocks.
 *
 *
 *
 * Return any string of minimum length that will unlock the safe at some point of entering it.
 *  
 * Example 1:
 *
 * Input: n = 1, k = 2
 * Output: "10"
 * Explanation: The password is a single digit, so enter each digit. "01" would also unlock the safe.
 *
 * Example 2:
 *
 * Input: n = 2, k = 2
 * Output: "01100"
 * Explanation: For each possible password:
 * - "00" is typed in starting from the 4^th digit.
 * - "01" is typed in starting from the 1^st digit.
 * - "10" is typed in starting from the 3^rd digit.
 * - "11" is typed in starting from the 2^nd digit.
 * Thus "01100" will unlock the safe. "01100", "10011", and "11001" would also unlock the safe.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 4
 * 	1 <= k <= 10
 * 	1 <= k^n <= 4096
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/cracking-the-safe/
// discuss: https://leetcode.com/problems/cracking-the-safe/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/cracking-the-safe/discuss/892038/Rust-translated-0ms-100
    pub fn crack_safe(n: i32, k: i32) -> String {
        let m = k.pow((n - 1) as u32);
        let mut dp = vec![0i32; (m * k) as usize];
        for i in 0..k as usize {
            for q in 0..m as usize {
                dp[i * m as usize + q] = (q * k as usize + i) as i32;
            }
        }
        let mut result = String::new();

        for i in 0..(m * k) as usize {
            let mut j = i;
            while dp[j] >= 0 {
                result.push(((j as i32 / m) as u8 + b'0') as char);
                let v = dp[j];
                dp[j] = -1;
                j = v as usize;
            }
        }

        for _ in 0..n as usize - 1 {
            result.push('0');
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_0753_example_1() {
        let n = 1;
        let k = 2;
        let result = "10".to_string();

        assert_eq!(Solution::crack_safe(n, k), result);
    }

    #[ignore]
    #[test]
    fn test_0753_example_2() {
        let n = 2;
        let k = 2;
        let result = "01100".to_string();

        assert_eq!(Solution::crack_safe(n, k), result);
    }
}
