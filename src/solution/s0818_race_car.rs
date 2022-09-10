/**
 * [0818] Race Car
 *
 * Your car starts at position 0 and speed +1 on an infinite number line. Your car can go into negative positions. Your car drives automatically according to a sequence of instructions 'A' (accelerate) and 'R' (reverse):
 *
 * 	When you get an instruction 'A', your car does the following:
 *
 * 		position += speed
 * 		speed *= 2
 *
 *
 * 	When you get an instruction 'R', your car does the following:
 *
 * 		If your speed is positive then speed = -1
 * 		otherwise speed = 1
 *
 * 	Your position stays the same.
 *
 * For example, after commands "AAR", your car goes to positions 0 --> 1 --> 3 --> 3, and your speed goes to 1 --> 2 --> 4 --> -1.
 * Given a target position target, return the length of the shortest sequence of instructions to get there.
 *  
 * Example 1:
 *
 * Input: target = 3
 * Output: 2
 * Explanation:
 * The shortest instruction sequence is "AA".
 * Your position goes from 0 --> 1 --> 3.
 *
 * Example 2:
 *
 * Input: target = 6
 * Output: 5
 * Explanation:
 * The shortest instruction sequence is "AAARA".
 * Your position goes from 0 --> 1 --> 3 --> 7 --> 7 --> 6.
 *
 *  
 * Constraints:
 *
 * 	1 <= target <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/race-car/
// discuss: https://leetcode.com/problems/race-car/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/race-car/discuss/827959/Rust-translated-0ms-100
    pub fn racecar(target: i32) -> i32 {
        fn number_of_leading_zeros(x: i32) -> i32 {
            let mut count = 0;
            let mut mask = -2147483648i32;
            while x & mask == 0 {
                count += 1;
                mask >>= 1;
            }
            count
        }

        let mut dp = vec![std::i32::MAX; target as usize + 4];
        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 4;
        dp[3] = 2;
        for t in 3..=target {
            let k = 32 - number_of_leading_zeros(t);
            if t == (1 << k) - 1 {
                dp[t as usize] = k;
                continue;
            }
            for j in 0..k - 1 {
                dp[t as usize] = std::cmp::min(
                    dp[t as usize],
                    dp[t as usize - (1 << (k - 1)) + (1 << j)] + k - 1 + j + 2,
                );
            }
            if (1 << k) - 1 - t < t {
                dp[t as usize] =
                    std::cmp::min(dp[t as usize], dp[(1 << k) - 1 - t as usize] + k + 1);
            }
        }

        dp[target as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0818_example_1() {
        let target = 3;
        let result = 2;

        assert_eq!(Solution::racecar(target), result);
    }

    #[test]
    fn test_0818_example_2() {
        let target = 6;
        let result = 5;

        assert_eq!(Solution::racecar(target), result);
    }
}
