/**
 * [0600] Non-negative Integers without Consecutive Ones
 *
 * Given a positive integer n, return the number of the integers in the range [0, n] whose binary representations do not contain consecutive ones.
 *  
 * Example 1:
 *
 * Input: n = 5
 * Output: 5
 * Explanation:
 * Here are the non-negative integers <= 5 with their corresponding binary representations:
 * 0 : 0
 * 1 : 1
 * 2 : 10
 * 3 : 11
 * 4 : 100
 * 5 : 101
 * Among them, only integer 3 disobeys the rule (two consecutive ones) and the other 5 satisfy the rule.
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 2
 *
 * Example 3:
 *
 * Input: n = 2
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/non-negative-integers-without-consecutive-ones/
// discuss: https://leetcode.com/problems/non-negative-integers-without-consecutive-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/non-negative-integers-without-consecutive-ones/discuss/103766/C%2B%2B-4-lines-DPFibonacci-6-ms
    pub fn find_integers(n: i32) -> i32 {
        const FB: [i32; 31] = [
            2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,
            17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309,
            3524578,
        ];

        if n < 3 {
            return n + 1;
        }

        for bt in (0..30).rev() {
            if (n & (1 << bt)) != 0 {
                return if (n & (1 << (bt - 1))) != 0 {
                    FB[bt]
                } else {
                    FB[bt - 1] + Self::find_integers((n & !(1 << bt)))
                };
            }
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0600_example_1() {
        let n = 5;
        let result = 5;

        assert_eq!(Solution::find_integers(n), result);
    }

    #[test]
    fn test_0600_example_2() {
        let n = 1;
        let result = 2;

        assert_eq!(Solution::find_integers(n), result);
    }

    #[test]
    fn test_0600_example_3() {
        let n = 2;
        let result = 3;

        assert_eq!(Solution::find_integers(n), result);
    }

    #[test]
    fn test_0600_additional_1() {
        let n = 1000000000;
        let result = 2178309;

        assert_eq!(Solution::find_integers(n), result);
    }
}
