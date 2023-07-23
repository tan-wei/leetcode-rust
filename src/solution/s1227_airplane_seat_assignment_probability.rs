/**
 * [1227] Airplane Seat Assignment Probability
 *
 * n passengers board an airplane with exactly n seats. The first passenger has lost the ticket and picks a seat randomly. But after that, the rest of the passengers will:
 *
 * 	Take their own seat if it is still available, and
 * 	Pick other seats randomly when they find their seat occupied
 *
 * Return the probability that the n^th person gets his own seat.
 *  
 * Example 1:
 *
 * Input: n = 1
 * Output: 1.00000
 * Explanation: The first person can only get the first seat.
 * Example 2:
 *
 * Input: n = 2
 * Output: 0.50000
 * Explanation: The second person has a probability of 0.5 to get the second seat (when first person gets the first seat).
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/airplane-seat-assignment-probability/
// discuss: https://leetcode.com/problems/airplane-seat-assignment-probability/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        if n == 1 {
            1.0
        } else {
            0.5
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1227_example_1() {
        let n = 1;
        let result = 1.00000;

        assert_f64_near!(Solution::nth_person_gets_nth_seat(n), result);
    }

    #[test]
    fn test_1227_example_2() {
        let n = 2;
        let result = 0.50000;

        assert_f64_near!(Solution::nth_person_gets_nth_seat(n), result);
    }
}
