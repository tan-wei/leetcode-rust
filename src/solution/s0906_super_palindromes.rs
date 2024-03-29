/**
 * [0906] Super Palindromes
 *
 * Let's say a positive integer is a super-palindrome if it is a palindrome, and it is also the square of a palindrome.
 * Given two positive integers left and right represented as strings, return the number of super-palindromes integers in the inclusive range [left, right].
 *  
 * Example 1:
 *
 * Input: left = "4", right = "1000"
 * Output: 4
 * Explanation: 4, 9, 121, and 484 are superpalindromes.
 * Note that 676 is not a superpalindrome: 26 * 26 = 676, but 26 is not a palindrome.
 *
 * Example 2:
 *
 * Input: left = "1", right = "2"
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= left.length, right.length <= 18
 * 	left and right consist of only digits.
 * 	left and right cannot have leading zeros.
 * 	left and right represent integers in the range [1, 10^18 - 1].
 * 	left is less than or equal to right.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/super-palindromes/
// discuss: https://leetcode.com/problems/super-palindromes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/super-palindromes/solutions/1197431/rust-backtracking-solution/
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let mut result = 0;
        if let (Ok(l), Ok(r)) = (left.parse::<u64>(), right.parse::<u64>()) {
            {
                let mut v = Vec::with_capacity(6);
                Self::dfs_helper(&mut result, &mut v, &(l..=r), false);
            }
            {
                let mut v = Vec::with_capacity(6);
                Self::dfs_helper(&mut result, &mut v, &(l..=r), true);
            }
        }
        result
    }

    fn dfs_helper(
        answer: &mut i32,
        v: &mut Vec<u64>,
        range: &std::ops::RangeInclusive<u64>,
        odd: bool,
    ) {
        if v.len() == 6 {
            let digits = if let Some(pos) = v.iter().position(|&d| d != 0) {
                &v[pos..]
            } else {
                &v
            };
            let mut n = 0;
            for &d in digits.iter() {
                n = n * 10 + d;
            }
            for &d in digits.iter().rev().skip(if odd { 1 } else { 0 }) {
                n = n * 10 + d;
            }
            if range.contains(&(n.saturating_mul(n))) {
                *answer += 1;
            }
        } else {
            let sum = v.iter().map(|d| d * d).sum::<u64>();
            for u in 0..=9 {
                if sum * 2 + u * u * if odd { 1 } else { 2 } < 10 {
                    v.push(u);
                    Self::dfs_helper(answer, v, range, odd);
                    v.pop();
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0906_example_1() {
        let left = "4".to_string();
        let right = "1000".to_string();
        let result = 4;

        assert_eq!(Solution::superpalindromes_in_range(left, right), result);
    }

    #[test]
    fn test_0906_example_2() {
        let left = "4".to_string();
        let right = "1000".to_string();
        let result = 4;

        assert_eq!(Solution::superpalindromes_in_range(left, right), result);
    }
}
