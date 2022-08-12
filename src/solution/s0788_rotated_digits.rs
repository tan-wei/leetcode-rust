/**
 * [0788] Rotated Digits
 *
 * An integer x is a good if after rotating each digit individually by 180 degrees, we get a valid number that is different from x. Each digit must be rotated - we cannot choose to leave it alone.
 * A number is valid if each digit remains a digit after rotation. For example:
 *
 * 	0, 1, and 8 rotate to themselves,
 * 	2 and 5 rotate to each other (in this case they are rotated in a different direction, in other words, 2 or 5 gets mirrored),
 * 	6 and 9 rotate to each other, and
 * 	the rest of the numbers do not rotate to any other number and become invalid.
 *
 * Given an integer n, return the number of good integers in the range [1, n].
 *  
 * Example 1:
 *
 * Input: n = 10
 * Output: 4
 * Explanation: There are four good numbers in the range [1, 10] : 2, 5, 6, 9.
 * Note that 1 and 10 are not good numbers, since they remain unchanged after rotating.
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 0
 *
 * Example 3:
 *
 * Input: n = 2
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rotated-digits/
// discuss: https://leetcode.com/problems/rotated-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut counter = 0;

        'outer: for orig in 1..=n {
            let mut i = 1;
            let mut new = 0;
            let mut tmp = orig;

            while tmp != 0 {
                let d = match tmp % 10 {
                    v @ 0 | v @ 1 | v @ 8 => v,
                    2 => 5,
                    5 => 2,
                    6 => 9,
                    9 => 6,
                    _ => continue 'outer,
                };

                new += d * i;
                i *= 10;
                tmp /= 10;
            }

            if new != orig {
                counter += 1;
            }
        }

        counter
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0788_example_1() {
        let n = 10;
        let result = 4;

        assert_eq!(Solution::rotated_digits(n), result);
    }

    #[test]
    fn test_0788_example_2() {
        let n = 1;
        let result = 0;

        assert_eq!(Solution::rotated_digits(n), result);
    }

    #[test]
    fn test_0788_example_3() {
        let n = 2;
        let result = 1;

        assert_eq!(Solution::rotated_digits(n), result);
    }
}
