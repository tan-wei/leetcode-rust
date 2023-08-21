/**
 * [1282] Group the People Given the Group Size They Belong To
 *
 * There are n people that are split into some unknown number of groups. Each person is labeled with a unique ID from 0 to n - 1.
 * You are given an integer array groupSizes, where groupSizes[i] is the size of the group that person i is in. For example, if groupSizes[1] = 3, then person 1 must be in a group of size 3.
 * Return a list of groups such that each person i is in a group of size groupSizes[i].
 * Each person should appear in exactly one group, and every person must be in a group. If there are multiple answers, return any of them. It is guaranteed that there will be at least one valid solution for the given input.
 *
 * Example 1:
 *
 * Input: groupSizes = [3,3,3,3,3,1,3]
 * Output: [[5],[0,1,2],[3,4,6]]
 * Explanation:
 * The first group is [5]. The size is 1, and groupSizes[5] = 1.
 * The second group is [0,1,2]. The size is 3, and groupSizes[0] = groupSizes[1] = groupSizes[2] = 3.
 * The third group is [3,4,6]. The size is 3, and groupSizes[3] = groupSizes[4] = groupSizes[6] = 3.
 * Other possible solutions are [[2,1,6],[5],[0,4,3]] and [[5],[0,6,2],[4,3,1]].
 *
 * Example 2:
 *
 * Input: groupSizes = [2,1,3,3,3,2]
 * Output: [[1],[0,5],[2,3,4]]
 *
 *
 * Constraints:
 *
 * 	groupSizes.length == n
 * 	1 <= n <= 500
 * 	1 <= groupSizes[i] <= n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/
// discuss: https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let n = group_sizes.len();
        let mut result = vec![];
        let mut memo = vec![vec![]; 501];

        for i in 0..n {
            let num = group_sizes[i] as usize;
            memo[num].push(i as i32);
            if memo[num].len() == num {
                result.push(memo[num].clone());
                memo[num] = vec![];
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
    fn test_1282_example_1() {
        let group_sizes = vec![3, 3, 3, 3, 3, 1, 3];
        let result = vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]];

        assert_eq_sorted!(Solution::group_the_people(group_sizes), result);
    }

    #[test]
    fn test_1282_example_2() {
        let group_sizes = vec![2, 1, 3, 3, 3, 2];
        let result = vec![vec![1], vec![0, 5], vec![2, 3, 4]];

        assert_eq_sorted!(Solution::group_the_people(group_sizes), result);
    }
}
