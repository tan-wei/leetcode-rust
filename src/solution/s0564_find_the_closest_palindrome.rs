/**
 * [0564] Find the Closest Palindrome
 *
 * Given a string n representing an integer, return the closest integer (not including itself), which is a palindrome. If there is a tie, return the smaller one.
 * The closest is defined as the absolute difference minimized between two integers.
 *  
 * Example 1:
 *
 * Input: n = "123"
 * Output: "121"
 *
 * Example 2:
 *
 * Input: n = "1"
 * Output: "0"
 * Explanation: 0 and 2 are the closest palindromes but we return the smallest which is 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= n.length <= 18
 * 	n consists of only digits.
 * 	n does not have leading zeros.
 * 	n is representing an integer in the range [1, 10^18 - 1].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-closest-palindrome/
// discuss: https://leetcode.com/problems/find-the-closest-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://rustgym.com/leetcode/564
    pub fn nearest_palindromic(n: String) -> String {
        let size = n.len();
        let half = size / 2;
        let value = n.parse::<i64>().unwrap();
        let a = 10i64.pow(size as u32);
        let b = 10i64.pow(size as u32 - 1);
        let mut candidates = vec![a - 1, a + 1, b - 1, b + 1];
        if size % 2 == 0 {
            let left = n[..half].parse::<i64>().unwrap();
            candidates.push(Self::combine(left - 1, false));
            candidates.push(Self::combine(left, false));
            candidates.push(Self::combine(left + 1, false));
        } else {
            let left = n[..half + 1].parse::<i64>().unwrap();
            candidates.push(Self::combine(left - 1, true));
            candidates.push(Self::combine(left, true));
            candidates.push(Self::combine(left + 1, true));
        }
        let mut result = (i64::MAX, 0);
        for x in candidates {
            let diff = (value - x).abs();
            if diff == 0 {
                continue;
            }
            if (diff, x) < result {
                result = (diff, x);
            }
        }
        (result.1).to_string()
    }

    fn combine(left: i64, odd: bool) -> i64 {
        let l = left.to_string();
        l.chars()
            .chain(l.chars().rev().skip(if odd { 1 } else { 0 }))
            .collect::<String>()
            .parse::<i64>()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0564_example_1() {
        let n = "123".to_string();
        let result = "121".to_string();

        assert_eq!(Solution::nearest_palindromic(n), result);
    }

    #[test]
    fn test_0564_example_2() {
        let n = "1".to_string();
        let result = "0".to_string();

        assert_eq!(Solution::nearest_palindromic(n), result);
    }

    #[test]
    fn test_0564_additional_1() {
        let n = "8".to_string();
        let result = "7".to_string();

        assert_eq!(Solution::nearest_palindromic(n), result);
    }

    #[test]
    fn test_0564_additional_2() {
        let n = "10".to_string();
        let result = "9".to_string();

        assert_eq!(Solution::nearest_palindromic(n), result);
    }

    #[test]
    fn test_0564_additional_3() {
        let n = "11".to_string();
        let result = "9".to_string();

        assert_eq!(Solution::nearest_palindromic(n), result);
    }
}
