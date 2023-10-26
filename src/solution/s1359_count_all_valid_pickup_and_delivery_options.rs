/**
 * [1359] Count All Valid Pickup and Delivery Options
 *
 * Given n orders, each order consists of a pickup and a delivery service.
 * Count all valid pickup/delivery possible sequences such that delivery(i) is always after of pickup(i).
 * Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 1
 * Output: 1
 * Explanation: Unique order (P1, D1), Delivery 1 always is after of Pickup 1.
 *
 * Example 2:
 *
 * Input: n = 2
 * Output: 6
 * Explanation: All possible orders:
 * (P1,P2,D1,D2), (P1,P2,D2,D1), (P1,D1,P2,D2), (P2,P1,D1,D2), (P2,P1,D2,D1) and (P2,D2,P1,D1).
 * This is an invalid order (P1,D2,P2,D1) because Pickup 2 is after of Delivery 2.
 *
 * Example 3:
 *
 * Input: n = 3
 * Output: 90
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 500
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/
// discuss: https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        (2..n as u64 * 2).step_by(2).fold(1, |acc, len| {
            (acc * (len + 1) * (2 + len) / 2) % 1_000_000_007
        }) as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1359_example_1() {
        let n = 1;

        let result = 1;

        assert_eq!(Solution::count_orders(n), result);
    }

    #[test]
    fn test_1359_example_2() {
        let n = 2;

        let result = 6;

        assert_eq!(Solution::count_orders(n), result);
    }

    #[test]
    fn test_1359_example_3() {
        let n = 3;

        let result = 90;

        assert_eq!(Solution::count_orders(n), result);
    }
}
