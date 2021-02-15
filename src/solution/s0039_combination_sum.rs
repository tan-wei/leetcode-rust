/**
 * [39] Combination Sum
 *
 * Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.
 * The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the frequency of at least one of the chosen numbers is different.
 * It is guaranteed that the number of unique combinations that sum up to target is less than 150 combinations for the given input.
 *  
 * Example 1:
 *
 * Input: candidates = [2,3,6,7], target = 7
 * Output: [[2,2,3],[7]]
 * Explanation:
 * 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
 * 7 is a candidate, and 7 = 7.
 * These are the only two combinations.
 *
 * Example 2:
 *
 * Input: candidates = [2,3,5], target = 8
 * Output: [[2,2,2,2],[2,3,3],[3,5]]
 *
 * Example 3:
 *
 * Input: candidates = [2], target = 1
 * Output: []
 *
 * Example 4:
 *
 * Input: candidates = [1], target = 1
 * Output: [[1]]
 *
 * Example 5:
 *
 * Input: candidates = [1], target = 2
 * Output: [[1,1]]
 *
 *  
 * Constraints:
 *
 * 	1 <= candidates.length <= 30
 * 	1 <= candidates[i] <= 200
 * 	All elements of candidates are distinct.
 * 	1 <= target <= 500
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/combination-sum/
// discuss: https://leetcode.com/problems/combination-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
        for i in start_idx..seq.len() {
            let item = seq[i];
            if target - item < 0 {
                continue;
            }
            let mut new_vec = curr.clone();
            new_vec.push(item);
            if target == item {
                result.push(new_vec);
            } else {
                Self::dfs(seq, target - item, new_vec, result, i);
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
    fn test_0039_example_1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let result = vec![vec![2, 2, 3], vec![7]];

        assert_eq_sorted!(Solution::combination_sum(candidates, target), result);
    }

    #[test]
    #[ignore]
    fn test_0039_example_2() {
        let candidates = vec![2, 3, 5];
        let target = 8;
        let result = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];

        assert_eq_sorted!(Solution::combination_sum(candidates, target), result);
    }

    #[test]
    #[ignore]
    fn test_0039_example_3() {
        let candidates = vec![2];
        let target = 1;
        let result: Vec<Vec<_>> = vec![];

        assert_eq_sorted!(Solution::combination_sum(candidates, target), result);
    }

    #[test]
    #[ignore]
    fn test_0039_example_4() {
        let candidates = vec![1];
        let target = 1;
        let result = vec![vec![1]];

        assert_eq_sorted!(Solution::combination_sum(candidates, target), result);
    }

    #[test]
    #[ignore]
    fn test_0039_example_5() {
        let candidates = vec![1];
        let target = 2;
        let result: Vec<Vec<_>> = vec![vec![1, 1]];

        assert_eq_sorted!(Solution::combination_sum(candidates, target), result);
    }
}
