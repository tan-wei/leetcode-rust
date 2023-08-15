/**
 * [1269] Number of Ways to Stay in the Same Place After Some Steps
 *
 * You have a pointer at index 0 in an array of size arrLen. At each step, you can move 1 position to the left, 1 position to the right in the array, or stay in the same place (The pointer should not be placed outside the array at any time).
 * Given two integers steps and arrLen, return the number of ways such that your pointer is still at index 0 after exactly steps steps. Since the answer may be too large, return it modulo 10^9 + 7.
 *
 * Example 1:
 *
 * Input: steps = 3, arrLen = 2
 * Output: 4
 * Explanation: There are 4 differents ways to stay at index 0 after 3 steps.
 * Right, Left, Stay
 * Stay, Right, Left
 * Right, Stay, Left
 * Stay, Stay, Stay
 *
 * Example 2:
 *
 * Input: steps = 2, arrLen = 4
 * Output: 2
 * Explanation: There are 2 differents ways to stay at index 0 after 2 steps
 * Right, Left
 * Stay, Stay
 *
 * Example 3:
 *
 * Input: steps = 4, arrLen = 2
 * Output: 8
 *
 *
 * Constraints:
 *
 * 	1 <= steps <= 500
 * 	1 <= arrLen <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/
// discuss: https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/solutions/3527268/rust-iterator-dp-bottom-up-space-optimized/
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let k = steps as usize;
        let n = k.min(arr_len as usize);
        (0..k)
            .fold(
                {
                    let mut memo: Vec<usize> = vec![0; n];
                    memo[0] = 1;
                    memo
                },
                |memo, _| {
                    (0..n)
                        .map(|i| {
                            let mut ways = memo[i];
                            if i > 0 {
                                ways = (ways + memo[i - 1]) % 1_000_000_007;
                            }
                            if i + 1 < n {
                                ways = (ways + memo[i + 1]) % 1_000_000_007;
                            }
                            ways
                        })
                        .collect()
                },
            )
            .get(0)
            .unwrap_or(&0)
            .clone() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1269_example_1() {
        let steps = 3;
        let arr_len = 2;
        let result = 4;

        assert_eq!(Solution::num_ways(steps, arr_len), result);
    }

    #[test]
    fn test_1269_example_2() {
        let steps = 2;
        let arr_len = 4;
        let result = 2;

        assert_eq!(Solution::num_ways(steps, arr_len), result);
    }

    #[test]
    fn test_1269_example_3() {
        let steps = 4;
        let arr_len = 2;
        let result = 8;

        assert_eq!(Solution::num_ways(steps, arr_len), result);
    }
}
