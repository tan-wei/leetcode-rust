/**
 * [202] Happy Number
 *
 * Write an algorithm to determine if a number n is happy.
 * A happy number is a number defined by the following process:
 *
 * 	Starting with any positive integer, replace the number by the sum of the squares of its digits.
 * 	Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
 * 	Those numbers for which this process ends in 1 are happy.
 *
 * Return true if n is a happy number, and false if not.
 *  
 * Example 1:
 *
 * Input: n = 19
 * Output: true
 * Explanation:
 * 1^2 + 9^2 = 82
 * 8^2 + 2^2 = 68
 * 6^2 + 8^2 = 100
 * 1^2 + 0^2 + 0^2 = 1
 *
 * Example 2:
 *
 * Input: n = 2
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/happy-number/
// discuss: https://leetcode.com/problems/happy-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen = std::collections::HashSet::new();
        let mut n = n;
        loop {
            n = format!("{}", n)
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .fold(0, |acc, cur| acc + (cur * cur) as i32);
            if n == 1 {
                return true;
            } else if seen.contains(&n) {
                return false;
            }
            seen.insert(n);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0202_example_1() {
        let n = 19;
        let result = true;

        assert_eq!(Solution::is_happy(n), result);
    }

    #[test]
    fn test_0202_example_2() {
        let n = 2;
        let result = false;

        assert_eq!(Solution::is_happy(n), result);
    }
}
