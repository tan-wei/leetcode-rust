/**
 * [90] Subsets II
 *
 * Given an integer array nums that may contain duplicates, return all possible subsets (the power set).
 * The solution set must not contain duplicate subsets. Return the solution in any order.
 *  
 * Example 1:
 * Input: nums = [1,2,2]
 * Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
 * Example 2:
 * Input: nums = [0]
 * Output: [[],[0]]
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10
 * 	-10 <= nums[i] <= 10
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subsets-ii/
// discuss: https://leetcode.com/problems/subsets-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        (0..2usize.pow(nums.len() as u32))
            .map(|i| {
                let mut s = nums
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| (i >> j) % 2 == 1)
                    .map(|(_, p)| *p)
                    .collect::<Vec<i32>>();
                s.sort();
                s
            })
            .collect::<std::collections::HashSet<Vec<i32>>>()
            .into_iter()
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0090_example_1() {
        let nums = vec![1, 2, 2];
        let result = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2],
        ];

        assert_eq_sorted!(Solution::subsets_with_dup(nums), result);
    }

    #[test]
    fn test_0090_example_2() {
        let nums = vec![0];
        let result = vec![vec![], vec![0]];

        assert_eq_sorted!(Solution::subsets_with_dup(nums), result);
    }
}
