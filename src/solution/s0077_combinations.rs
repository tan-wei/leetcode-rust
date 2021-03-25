/**
 * [77] Combinations
 *
 * Given two integers n and k, return all possible combinations of k numbers out of 1 ... n.
 * You may return the answer in any order.
 *  
 * Example 1:
 *
 * Input: n = 4, k = 2
 * Output:
 * [
 *   [2,4],
 *   [3,4],
 *   [2,3],
 *   [1,2],
 *   [1,3],
 *   [1,4],
 * ]
 *
 * Example 2:
 *
 * Input: n = 1, k = 1
 * Output: [[1]]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 20
 * 	1 <= k <= n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/combinations/
// discuss: https://leetcode.com/problems/combinations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if k == 0 {
            return vec![vec![]];
        }

        let mut result = vec![];

        for i in k..=n {
            for mut pre in Self::combine(i - 1, k - 1) {
                pre.push(i);
                result.push(pre);
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0077_example_1() {
        let n = 4;
        let k = 2;
        let result = vec![
            vec![2, 4],
            vec![3, 4],
            vec![2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
        ];

        assert_eq_sorted!(Solution::combine(n, k), result);
    }

    #[test]
    fn test_0077_example_2() {
        let n = 1;
        let k = 1;
        let result = vec![vec![1]];

        assert_eq_sorted!(Solution::combine(n, k), result);
    }
}
