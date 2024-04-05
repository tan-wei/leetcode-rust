/**
 * [1547] Minimum Cost to Cut a Stick
 *
 * Given a wooden stick of length n units. The stick is labelled from 0 to n. For example, a stick of length 6 is labelled as follows:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/21/statement.jpg" style="width: 521px; height: 111px;" />
 * Given an integer array cuts where cuts[i] denotes a position you should perform a cut at.
 * You should perform the cuts in order, you can change the order of the cuts as you wish.
 * The cost of one cut is the length of the stick to be cut, the total cost is the sum of costs of all cuts. When you cut a stick, it will be split into two smaller sticks (i.e. the sum of their lengths is the length of the stick before the cut). Please refer to the first example for a better explanation.
 * Return the minimum total cost of the cuts.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/23/e1.jpg" style="width: 350px; height: 284px;" />
 * Input: n = 7, cuts = [1,3,4,5]
 * Output: 16
 * Explanation: Using cuts order = [1, 3, 4, 5] as in the input leads to the following scenario:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/21/e11.jpg" style="width: 350px; height: 284px;" />
 * The first cut is done to a rod of length 7 so the cost is 7. The second cut is done to a rod of length 6 (i.e. the second part of the first cut), the third is done to a rod of length 4 and the last cut is to a rod of length 3. The total cost is 7 + 6 + 4 + 3 = 20.
 * Rearranging the cuts to be [3, 5, 1, 4] for example will lead to a scenario with total cost = 16 (as shown in the example photo 7 + 4 + 3 + 2 = 16).
 * Example 2:
 *
 * Input: n = 9, cuts = [5,6,1,4,2]
 * Output: 22
 * Explanation: If you try the given cuts ordering the cost will be 25.
 * There are much ordering with total cost <= 25, for example, the order [4, 6, 5, 2, 1] has total cost = 22 which is the minimum possible.
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 10^6
 * 	1 <= cuts.length <= min(n - 1, 100)
 * 	1 <= cuts[i] <= n - 1
 * 	All the integers in cuts array are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-to-cut-a-stick/
// discuss: https://leetcode.com/problems/minimum-cost-to-cut-a-stick/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-cost-to-cut-a-stick/solutions/3167350/just-a-runnable-solution/
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut cuts = cuts;

        cuts.push(0);
        cuts.push(n as i32);
        cuts.sort_unstable();

        let m = cuts.len();
        let mut dp = vec![vec![0; m]; m];

        for (i, item) in dp.iter_mut().enumerate().take(m) {
            item[i] = 0;
        }

        for (i, item) in dp.iter_mut().enumerate().take(m - 1) {
            item[i + 1] = 0;
        }

        for len in 2..m {
            for i in 0..m - len {
                let j = i + len;
                let mut min = std::i32::MAX;

                for k in i + 1..j {
                    let cost = dp[i][k] + dp[k][j] + cuts[j] - cuts[i];
                    if cost < min {
                        min = cost;
                    }
                }
                dp[i][j] = min;
            }
        }

        dp[0][m - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1547_example_1() {
        let n = 7;
        let cuts = vec![1, 3, 4, 5];

        let result = 16;

        assert_eq!(Solution::min_cost(n, cuts), result);
    }

    #[test]
    fn test_1547_example_2() {
        let n = 9;
        let cuts = vec![5, 6, 1, 4, 2];

        let result = 22;

        assert_eq!(Solution::min_cost(n, cuts), result);
    }
}
