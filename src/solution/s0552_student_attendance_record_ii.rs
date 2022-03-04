/**
 * [0552] Student Attendance Record II
 *
 * An attendance record for a student can be represented as a string where each character signifies whether the student was absent, late, or present on that day. The record only contains the following three characters:
 *
 * 	'A': Absent.
 * 	'L': Late.
 * 	'P': Present.
 *
 * Any student is eligible for an attendance award if they meet both of the following criteria:
 *
 * 	The student was absent ('A') for strictly fewer than 2 days total.
 * 	The student was never late ('L') for 3 or more consecutive days.
 *
 * Given an integer n, return the number of possible attendance records of length n that make a student eligible for an attendance award. The answer may be very large, so return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 2
 * Output: 8
 * Explanation: There are 8 records with length 2 that are eligible for an award:
 * "PP", "AP", "PA", "LP", "PL", "AL", "LA", "LL"
 * Only "AA" is not eligible because there are 2 absences (there need to be fewer than 2).
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 3
 *
 * Example 3:
 *
 * Input: n = 10101
 * Output: 183236316
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/student-attendance-record-ii/
// discuss: https://leetcode.com/problems/student-attendance-record-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
const MOD: i64 = 1_000_000_007;

impl Solution {
    // Credit: https://rustgym.com/leetcode/552
    pub fn check_record(n: i32) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 3;
        }
        let mut p = vec![0; n + 1];
        let mut l = vec![0; n + 1];
        p[1] = 1;
        l[1] = 1;
        p[2] = 2;
        l[2] = 2;
        for i in 3..=n {
            p[i] = l[i - 1] + p[i - 1];
            p[i] %= MOD;
            l[i] = p[i - 1] + p[i - 2];
            l[i] %= MOD;
        }
        let mut lp = vec![0; n + 1];
        lp[0] = 1;
        for i in 1..=n {
            lp[i] = l[i] + p[i];
            lp[i] %= MOD;
        }

        let mut res: i64 = lp[n];
        for i in 0..n {
            res += (lp[i]) * (lp[n - 1 - i]);
            res %= MOD;
        }
        res as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0552_example_1() {
        let n = 2;
        let result = 8;

        assert_eq!(Solution::check_record(n), result);
    }

    #[test]
    fn test_0552_example_2() {
        let n = 1;
        let result = 3;

        assert_eq!(Solution::check_record(n), result);
    }

    #[test]
    fn test_0552_example_3() {
        let n = 10101;
        let result = 183236316;

        assert_eq!(Solution::check_record(n), result);
    }
}
