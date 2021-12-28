/**
 * [0464] Can I Win
 *
 * In the "100 game" two players take turns adding, to a running total, any integer from 1 to 10. The player who first causes the running total to reach or exceed 100 wins.
 * What if we change the game so that players cannot re-use integers?
 * For example, two players might take turns drawing from a common pool of numbers from 1 to 15 without replacement until they reach a total >= 100.
 * Given two integers maxChoosableInteger and desiredTotal, return true if the first player to move can force a win, otherwise, return false. Assume both players play optimally.
 *  
 * Example 1:
 *
 * Input: maxChoosableInteger = 10, desiredTotal = 11
 * Output: false
 * Explanation:
 * No matter which integer the first player choose, the first player will lose.
 * The first player can choose an integer from 1 up to 10.
 * If the first player choose 1, the second player can only choose integers from 2 up to 10.
 * The second player will win by choosing 10 and get a total = 11, which is >= desiredTotal.
 * Same with other integers chosen by the first player, the second player will always win.
 *
 * Example 2:
 *
 * Input: maxChoosableInteger = 10, desiredTotal = 0
 * Output: true
 *
 * Example 3:
 *
 * Input: maxChoosableInteger = 10, desiredTotal = 1
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	1 <= maxChoosableInteger <= 20
 * 	0 <= desiredTotal <= 300
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/can-i-win/
// discuss: https://leetcode.com/problems/can-i-win/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/can-i-win/discuss/1318781/Rust-cheapest-and-best
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        let sum: i32 = (1..=max_choosable_integer).sum();
        if desired_total < 2 {
            true
        } else if sum < desired_total {
            false
        } else {
            let mut cache = std::collections::HashMap::new();
            Self::go(max_choosable_integer, 0, desired_total, &mut cache)
        }
    }

    fn go(m: i32, key: i32, total: i32, cache: &mut std::collections::HashMap<i32, bool>) -> bool {
        if let Some(result) = cache.get(&key) {
            *result
        } else if total <= 0 {
            false
        } else {
            for n in 1..=m {
                if key & (1 << n) == 0 && !Self::go(m, key | (1 << n), total - n, cache) {
                    cache.insert(key, true);
                    return true;
                }
            }
            cache.insert(key, false);
            false
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0464_example_1() {
        let max_choosable_integer = 10;
        let desired_total = 11;
        let result = false;

        assert_eq!(
            Solution::can_i_win(max_choosable_integer, desired_total),
            result
        );
    }
}
