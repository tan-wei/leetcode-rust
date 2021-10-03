/**
 * [354] Russian Doll Envelopes
 *
 * You are given a 2D array of integers envelopes where envelopes[i] = [wi, hi] represents the width and the height of an envelope.
 * One envelope can fit into another if and only if both the width and height of one envelope are greater than the other envelope's width and height.
 * Return the maximum number of envelopes you can Russian doll (i.e., put one inside the other).
 * Note: You cannot rotate an envelope.
 *  
 * Example 1:
 *
 * Input: envelopes = [[5,4],[6,4],[6,7],[2,3]]
 * Output: 3
 * Explanation: The maximum number of envelopes you can Russian doll is 3 ([2,3] => [5,4] => [6,7]).
 *
 * Example 2:
 *
 * Input: envelopes = [[1,1],[1,1],[1,1]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= envelopes.length <= 5000
 * 	envelopes[i].length == 2
 * 	1 <= wi, hi <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/russian-doll-envelopes/
// discuss: https://leetcode.com/problems/russian-doll-envelopes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/russian-doll-envelopes/discuss/1134368/Rust-solution
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes
            .iter()
            .map(|envelope| (envelope[0], std::cmp::Reverse(envelope[1])))
            .collect::<Vec<_>>();
        envelopes.sort_unstable();
        let mut dp = Vec::new();
        for &(_, std::cmp::Reverse(h)) in &envelopes {
            if let Some(i) = dp.binary_search(&h).err() {
                if i < dp.len() {
                    dp[i] = h;
                } else {
                    dp.push(h);
                }
            }
        }
        dp.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0354_example_1() {
        let envelopes = vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]];
        let result = 3;

        assert_eq!(Solution::max_envelopes(envelopes), result);
    }

    #[test]
    fn test_0354_example_2() {
        let envelopes = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
        let result = 1;

        assert_eq!(Solution::max_envelopes(envelopes), result);
    }
}
