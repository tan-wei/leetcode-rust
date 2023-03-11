/**
 * [1000] Minimum Cost to Merge Stones
 *
 * There are n piles of stones arranged in a row. The i^th pile has stones[i] stones.
 * A move consists of merging exactly k consecutive piles into one pile, and the cost of this move is equal to the total number of stones in these k piles.
 * Return the minimum cost to merge all piles of stones into one pile. If it is impossible, return -1.
 *  
 * Example 1:
 *
 * Input: stones = [3,2,4,1], k = 2
 * Output: 20
 * Explanation: We start with [3, 2, 4, 1].
 * We merge [3, 2] for a cost of 5, and we are left with [5, 4, 1].
 * We merge [4, 1] for a cost of 5, and we are left with [5, 5].
 * We merge [5, 5] for a cost of 10, and we are left with [10].
 * The total cost was 20, and this is the minimum possible.
 *
 * Example 2:
 *
 * Input: stones = [3,2,4,1], k = 3
 * Output: -1
 * Explanation: After any merge operation, there are 2 piles left, and we can't merge anymore.  So the task is impossible.
 *
 * Example 3:
 *
 * Input: stones = [3,5,1,2,6], k = 3
 * Output: 25
 * Explanation: We start with [3, 5, 1, 2, 6].
 * We merge [5, 1, 2] for a cost of 8, and we are left with [3, 8, 6].
 * We merge [3, 8, 6] for a cost of 17, and we are left with [17].
 * The total cost was 25, and this is the minimum possible.
 *
 *  
 * Constraints:
 *
 * 	n == stones.length
 * 	1 <= n <= 30
 * 	1 <= stones[i] <= 100
 * 	2 <= k <= 30
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-to-merge-stones/
// discuss: https://leetcode.com/problems/minimum-cost-to-merge-stones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-cost-to-merge-stones/solutions/850829/0ms-rust-beats-100/
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        if (stones.len() - 1) % (k - 1) != 0 {
            return -1;
        }
        let mut dp = vec![vec![0; stones.len()]; stones.len()];
        let sum = {
            let mut sum = Vec::with_capacity(stones.len() + 1);
            sum.push(0);
            let mut cur = 0;
            sum.extend(stones.iter().map(|n| {
                cur = cur + n;
                cur.clone()
            }));
            sum
        };
        for len in (k - 1)..stones.len() {
            for i in 0..(stones.len() - len) {
                let j = i + len;
                dp[i][j] = (i..j)
                    .step_by(k - 1)
                    .map(|l| dp[i][l] + dp[l + 1][j])
                    .min()
                    .unwrap();
                if (j - i) % (k - 1) == 0 {
                    dp[i][j] += sum[j + 1] - sum[i];
                }
            }
        }
        dp[0][stones.len() - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1000_example_1() {
        let stones = vec![3, 2, 4, 1];
        let k = 2;
        let result = 20;

        assert_eq!(Solution::merge_stones(stones, k), result);
    }

    #[test]
    fn test_1000_example_2() {
        let stones = vec![3, 2, 4, 1];
        let k = 3;
        let result = -1;

        assert_eq!(Solution::merge_stones(stones, k), result);
    }

    #[test]
    fn test_1000_example_3() {
        let stones = vec![3, 5, 1, 2, 6];
        let k = 3;
        let result = 25;

        assert_eq!(Solution::merge_stones(stones, k), result);
    }
}
