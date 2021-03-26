/**
 * [78] Subsets
 *
 * Given an integer array nums of unique elements, return all possible subsets (the power set).
 * The solution set must not contain duplicate subsets. Return the solution in any order.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3]
 * Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
 *
 * Example 2:
 *
 * Input: nums = [0]
 * Output: [[],[0]]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10
 * 	-10 <= nums[i] <= 10
 * 	All the numbers of nums are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subsets/
// discuss: https://leetcode.com/problems/subsets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        (0..2u32.pow(nums.len() as u32))
            .into_iter()
            .map(|x| {
                format!("{:0num$b}", x, num = nums.len())
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '1')
                    .map(|(i, _)| nums[i])
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0078_example_1() {
        let nums = vec![1, 2, 3];
        let result = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];

        assert_eq_sorted!(Solution::subsets(nums), result);
    }

    #[test]
    fn test_0078_example_2() {
        let nums = vec![0];
        let result = vec![vec![], vec![0]];

        assert_eq_sorted!(Solution::subsets(nums), result);
    }
}
