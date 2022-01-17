/**
 * [0491] Increasing Subsequences
 *
 * Given an integer array nums, return all the different possible increasing subsequences of the given array with at least two elements. You may return the answer in any order.
 * The given array may contain duplicates, and two equal integers should also be considered a special case of increasing sequence.
 *  
 * Example 1:
 *
 * Input: nums = [4,6,7,7]
 * Output: [[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]
 *
 * Example 2:
 *
 * Input: nums = [4,4,3,2,1]
 * Output: [[4,4]]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 15
 * 	-100 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/increasing-subsequences/
// discuss: https://leetcode.com/problems/increasing-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = std::collections::HashSet::new();
        Self::dfs_helper(&nums, 0, &vec![], &mut results);
        results.into_iter().collect()
    }

    fn dfs_helper(
        nums: &Vec<i32>,
        depth: usize,
        candidate: &Vec<i32>,
        results: &mut std::collections::HashSet<Vec<i32>>,
    ) {
        if depth >= nums.len() {
            return;
        }

        let val = nums[depth];

        if candidate.len() == 0 || val >= *candidate.last().unwrap() {
            let mut next_candidate = candidate.to_vec();
            next_candidate.push(val);
            Self::dfs_helper(nums, depth + 1, &next_candidate, results);
            if next_candidate.len() > 1 {
                results.insert(next_candidate);
            }
        }

        Self::dfs_helper(nums, depth + 1, candidate, results);
    }

    // credeit: https://leetcode.com/problems/increasing-subsequences/discuss/1333960/Rust-cheapest-and-best
    pub fn find_subsequences_v2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.into_iter()
            .fold(
                vec![vec![]]
                    .into_iter()
                    .collect::<std::collections::HashSet<Vec<i32>>>(),
                |seqs, n| {
                    seqs.into_iter()
                        .flat_map(|seq| {
                            if seq.last().unwrap_or(&-100) <= &n {
                                vec![seq.iter().cloned().chain(vec![n]).collect(), seq]
                            } else {
                                vec![seq]
                            }
                        })
                        .collect()
                },
            )
            .into_iter()
            .filter(|seq| seq.len() >= 2)
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0491_example_1() {
        let nums = vec![4, 6, 7, 7];
        let result = vec![
            vec![4, 6],
            vec![4, 6, 7],
            vec![4, 6, 7, 7],
            vec![4, 7],
            vec![4, 7, 7],
            vec![6, 7],
            vec![6, 7, 7],
            vec![7, 7],
        ];

        assert_eq_sorted!(Solution::find_subsequences(nums), result);

        let nums = vec![4, 6, 7, 7];
        let result = vec![
            vec![4, 6],
            vec![4, 6, 7],
            vec![4, 6, 7, 7],
            vec![4, 7],
            vec![4, 7, 7],
            vec![6, 7],
            vec![6, 7, 7],
            vec![7, 7],
        ];

        assert_eq_sorted!(Solution::find_subsequences_v2(nums), result);
    }

    #[test]
    fn test_0491_example_2() {
        let nums = vec![4, 4, 3, 2, 1];
        let result = vec![vec![4, 4]];

        assert_eq_sorted!(Solution::find_subsequences(nums), result);

        let nums = vec![4, 4, 3, 2, 1];
        let result = vec![vec![4, 4]];

        assert_eq_sorted!(Solution::find_subsequences_v2(nums), result);
    }
}
