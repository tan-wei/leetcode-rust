/**
 * [0547] Number of Provinces
 *
 * There are n cities. Some of them are connected, while some are not. If city a is connected directly with city b, and city b is connected directly with city c, then city a is connected indirectly with city c.
 * A province is a group of directly or indirectly connected cities and no other cities outside of the group.
 * You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the i^th city and the j^th city are directly connected, and isConnected[i][j] = 0 otherwise.
 * Return the total number of provinces.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/24/graph1.jpg" style="width: 222px; height: 142px;" />
 * Input: isConnected = [[1,1,0],[1,1,0],[0,0,1]]
 * Output: 2
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/24/graph2.jpg" style="width: 222px; height: 142px;" />
 * Input: isConnected = [[1,0,0],[0,1,0],[0,0,1]]
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 200
 * 	n == isConnected.length
 * 	n == isConnected[i].length
 * 	isConnected[i][j] is 1 or 0.
 * 	isConnected[i][i] == 1
 * 	isConnected[i][j] == isConnected[j][i]
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-provinces/
// discuss: https://leetcode.com/problems/number-of-provinces/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();

        if n == 0 {
            return 0;
        }

        let mut visited = vec![0; n];
        let mut result = 0;

        for i in 0..n {
            if visited[i] == 0 {
                Self::dfs_helper(&is_connected, &mut visited, i);
                result += 1;
            }
        }

        result
    }

    fn dfs_helper(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<i32>, i: usize) {
        for j in 0..is_connected.len() {
            if is_connected[i][j] == 1 && visited[j] == 0 {
                visited[j] = 1;
                Self::dfs_helper(is_connected, visited, j);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0547_example_1() {
        let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        let result = 2;

        assert_eq!(Solution::find_circle_num(is_connected), result);
    }

    #[test]
    fn test_0547_example_2() {
        let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let result = 3;

        assert_eq!(Solution::find_circle_num(is_connected), result);
    }
}
