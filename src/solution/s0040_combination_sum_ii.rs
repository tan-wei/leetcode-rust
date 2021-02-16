/**
 * [40] Combination Sum II
 *
 * Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target.
 * Each number in candidates may only be used once in the combination.
 * Note: The solution set must not contain duplicate combinations.
 *  
 * Example 1:
 *
 * Input: candidates = [10,1,2,7,6,1,5], target = 8
 * Output:
 * [
 * [1,1,6],
 * [1,2,5],
 * [1,7],
 * [2,6]
 * ]
 *
 * Example 2:
 *
 * Input: candidates = [2,5,2,1,2], target = 5
 * Output:
 * [
 * [1,2,2],
 * [5]
 * ]
 *
 *  
 * Constraints:
 *
 * 	1 <= candidates.length <= 100
 * 	1 <= candidates[i] <= 50
 * 	1 <= target <= 30
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/combination-sum-ii/
// discuss: https://leetcode.com/problems/combination-sum-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut seq = candidates;
        let mut res = Vec::new();
        seq.sort_unstable_by(|a, b| b.cmp(a));
        let mut vec = Vec::new();
        Self::dfs(&seq, target, vec, &mut res, 0);
        res
    }

    fn dfs(
        seq: &Vec<i32>,
        target: i32,
        mut curr: Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        start_idx: usize,
    ) {
        let mut i = start_idx;
        while i < seq.len() {
            let item = seq[i];
            if target - item < 0 {
                i += 1;
                continue;
            }
            let mut new_vec = curr.clone();
            new_vec.push(item);
            if target == item {
                result.push(new_vec);
            } else {
                Self::dfs(seq, target - item, new_vec, result, i + 1);
            }
            // skip duplicate result
            while i < seq.len() && seq[i] == item {
                i += 1;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0040_example_1() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let result = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];

        assert_eq_sorted!(Solution::combination_sum2(candidates, target), result);
    }

    #[test]
    #[ignore]
    fn test_0040_example_2() {
        let candidates = vec![2, 5, 2, 1, 2];
        let target = 5;
        let result = vec![vec![1, 2, 2], vec![5]];

        assert_eq_sorted!(Solution::combination_sum2(candidates, target), result);
    }
}
