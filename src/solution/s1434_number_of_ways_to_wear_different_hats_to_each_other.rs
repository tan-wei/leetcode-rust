/**
 * [1434] Number of Ways to Wear Different Hats to Each Other
 *
 * There are n people and 40 types of hats labeled from 1 to 40.
 * Given a 2D integer array hats, where hats[i] is a list of all hats preferred by the i^th person.
 * Return the number of ways that the n people wear different hats to each other.
 * Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: hats = [[3,4],[4,5],[5]]
 * Output: 1
 * Explanation: There is only one way to choose hats given the conditions.
 * First person choose hat 3, Second person choose hat 4 and last one hat 5.
 *
 * Example 2:
 *
 * Input: hats = [[3,5,1],[3,5]]
 * Output: 4
 * Explanation: There are 4 ways to choose hats:
 * (3,5), (5,3), (1,3) and (1,5)
 *
 * Example 3:
 *
 * Input: hats = [[1,2,3,4],[1,2,3,4],[1,2,3,4],[1,2,3,4]]
 * Output: 24
 * Explanation: Each person can choose hats labeled from 1 to 4.
 * Number of Permutations of (1,2,3,4) = 24.
 *
 *  
 * Constraints:
 *
 * 	n == hats.length
 * 	1 <= n <= 10
 * 	1 <= hats[i].length <= 40
 * 	1 <= hats[i][j] <= 40
 * 	hats[i] contains a list of unique integers.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-to-wear-different-hats-to-each-other/
// discuss: https://leetcode.com/problems/number-of-ways-to-wear-different-hats-to-each-other/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/number-of-ways-to-wear-different-hats-to-each-other/solutions/3112072/just-a-runnable-solution/
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![-1; 1024]; 41];
        let n = hats.len();
        let mut h2p = vec![vec![]; 41];

        for (i, item) in hats.iter().enumerate().take(n) {
            for &hat in item {
                h2p[hat as usize].push(i);
            }
        }

        Self::dfs_helper(&hats, &h2p, (1 << n) - 1, 1, 0, &mut dp)
    }

    fn dfs_helper(
        hats: &Vec<Vec<i32>>,
        h2p: &Vec<Vec<usize>>,
        all_mask: i32,
        hat: usize,
        assigned_people: i32,
        dp: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if assigned_people == all_mask {
            return 1;
        }

        if hat > 40 {
            return 0;
        }

        if dp[hat][assigned_people as usize] != -1 {
            return dp[hat][assigned_people as usize];
        }

        let mut answer = Self::dfs_helper(hats, h2p, all_mask, hat + 1, assigned_people, dp);

        for &p in &h2p[hat] {
            if ((assigned_people >> p) & 1) == 1 {
                continue;
            }
            answer +=
                Self::dfs_helper(hats, h2p, all_mask, hat + 1, assigned_people | (1 << p), dp);
            answer %= 1_000_000_007;
        }

        dp[hat][assigned_people as usize] = answer;

        answer
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1434_example_1() {
        let hats = vec![vec![3, 4], vec![4, 5], vec![5]];

        let result = 1;

        assert_eq!(Solution::number_ways(hats), result);
    }

    #[test]
    fn test_1434_example_2() {
        let hats = vec![vec![3, 5, 1], vec![3, 5]];

        let result = 4;

        assert_eq!(Solution::number_ways(hats), result);
    }

    #[test]
    fn test_1434_example_3() {
        let hats = vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
        ];

        let result = 24;

        assert_eq!(Solution::number_ways(hats), result);
    }
}
