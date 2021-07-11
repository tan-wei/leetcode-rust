/**
 * [216] Combination Sum III
 *
 * Find all valid combinations of k numbers that sum up to n such that the following conditions are true:
 *
 * 	Only numbers 1 through 9 are used.
 * 	Each number is used at most once.
 *
 * Return a list of all possible valid combinations. The list must not contain the same combination twice, and the combinations may be returned in any order.
 *  
 * Example 1:
 *
 * Input: k = 3, n = 7
 * Output: [[1,2,4]]
 * Explanation:
 * 1 + 2 + 4 = 7
 * There are no other valid combinations.
 * Example 2:
 *
 * Input: k = 3, n = 9
 * Output: [[1,2,6],[1,3,5],[2,3,4]]
 * Explanation:
 * 1 + 2 + 6 = 9
 * 1 + 3 + 5 = 9
 * 2 + 3 + 4 = 9
 * There are no other valid combinations.
 *
 * Example 3:
 *
 * Input: k = 4, n = 1
 * Output: []
 * Explanation: There are no valid combinations.
 * Using 4 different numbers in the range [1,9], the smallest sum we can get is 1+2+3+4 = 10 and since 10 > 1, there are no valid combination.
 *
 * Example 4:
 *
 * Input: k = 3, n = 2
 * Output: []
 * Explanation: There are no valid combinations.
 *
 * Example 5:
 *
 * Input: k = 9, n = 45
 * Output: [[1,2,3,4,5,6,7,8,9]]
 * Explanation:
 * 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 = 45
 * There are no other valid combinations.
 *
 *  
 * Constraints:
 *
 * 	2 <= k <= 9
 * 	1 <= n <= 60
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/combination-sum-iii/
// discuss: https://leetcode.com/problems/combination-sum-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut comb = Vec::new();
        Self::dfs_helper(k, n, 1, &mut comb, &mut res);
        res
    }

    fn dfs_helper(k: i32, n: i32, start: i32, comb: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if k == 0 && n == 0 {
            res.push(comb.clone());
            return;
        }
        if k < 0 || n < 0 {
            return;
        }
        for i in start..10 {
            comb.push(i);
            Self::dfs_helper(k - 1, n - i, i + 1, comb, res);
            comb.pop();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0216_example_1() {
        let k = 3;
        let n = 7;
        let result = vec![vec![1, 2, 4]];

        assert_eq_sorted!(Solution::combination_sum3(k, n), result);
    }

    #[test]
    fn test_0216_example_2() {
        let k = 3;
        let n = 9;
        let result = vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]];

        assert_eq_sorted!(Solution::combination_sum3(k, n), result);
    }

    #[test]
    fn test_0216_example_3() {
        let k = 9;
        let n = 45;
        let result = vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9]];

        assert_eq_sorted!(Solution::combination_sum3(k, n), result);
    }
}
