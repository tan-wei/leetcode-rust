/**
 * [1923] Longest Common Subpath
 *
 * There is a country of n cities numbered from 0 to n - 1. In this country, there is a road connecting every pair of cities.
 * There are m friends numbered from 0 to m - 1 who are traveling through the country. Each one of them will take a path consisting of some cities. Each path is represented by an integer array that contains the visited cities in order. The path may contain a city more than once, but the same city will not be listed consecutively.
 * Given an integer n and a 2D integer array paths where paths[i] is an integer array representing the path of the i^th friend, return the length of the longest common subpath that is shared by every friend's path, or 0 if there is no common subpath at all.
 * A subpath of a path is a contiguous sequence of cities within that path.
 *  
 * Example 1:
 *
 * Input: n = 5, paths = [[0,1,<u>2,3</u>,4],
 *                        [<u>2,3</u>,4],
 *                        [4,0,1,<u>2,3</u>]]
 * Output: 2
 * Explanation: The longest common subpath is [2,3].
 *
 * Example 2:
 *
 * Input: n = 3, paths = [[0],[1],[2]]
 * Output: 0
 * Explanation: There is no common subpath shared by the three paths.
 *
 * Example 3:
 *
 * Input: n = 5, paths = [[<u>0</u>,1,2,3,4],
 *                        [4,3,2,1,<u>0</u>]]
 * Output: 1
 * Explanation: The possible longest common subpaths are [0], [1], [2], [3], and [4]. All have a length of 1.
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 * 	m == paths.length
 * 	2 <= m <= 10^5
 * 	sum(paths[i].length) <= 10^5
 * 	0 <= paths[i][j] < n
 * 	The same city is not listed multiple times consecutively in paths[i].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-common-subpath/
// discuss: https://leetcode.com/problems/longest-common-subpath/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_common_subpath(n: i32, paths: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1923_example_1() {
        let n = 5;
        let paths = vec![vec![0, 1, 2, 3, 4], vec![2, 3, 4], vec![4, 0, 1, 2, 3]];

        let result = 2;

        assert_eq!(Solution::longest_common_subpath(n, paths), result);
    }

    #[test]
    #[ignore]
    fn test_1923_example_2() {
        let n = 3;
        let paths = vec![vec![0], vec![1], vec![2]];

        let result = 0;

        assert_eq!(Solution::longest_common_subpath(n, paths), result);
    }

    #[test]
    #[ignore]
    fn test_1923_example_3() {
        let n = 5;
        let paths = vec![vec![0, 1, 2, 3, 4], vec![4, 3, 2, 1]];

        let result = 1;

        assert_eq!(Solution::longest_common_subpath(n, paths), result);
    }
}
