/**
 * [1319] Number of Operations to Make Network Connected
 *
 * There are n computers numbered from 0 to n - 1 connected by ethernet cables connections forming a network where connections[i] = [ai, bi] represents a connection between computers ai and bi. Any computer can reach any other computer directly or indirectly through the network.
 * You are given an initial computer network connections. You can extract certain cables between two directly connected computers, and place them between any pair of disconnected computers to make them directly connected.
 * Return the minimum number of times you need to do this in order to make all the computers connected. If it is not possible, return -1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/02/sample_1_1677.png" style="width: 500px; height: 148px;" />
 * Input: n = 4, connections = [[0,1],[0,2],[1,2]]
 * Output: 1
 * Explanation: Remove cable between computer 1 and 2 and place between computers 1 and 3.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/02/sample_2_1677.png" style="width: 500px; height: 129px;" />
 * Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2],[1,3]]
 * Output: 2
 *
 * Example 3:
 *
 * Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2]]
 * Output: -1
 * Explanation: There are not enough cables.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 * 	1 <= connections.length <= min(n * (n - 1) / 2, 10^5)
 * 	connections[i].length == 2
 * 	0 <= ai, bi < n
 * 	ai != bi
 * 	There are no repeated connections.
 * 	No two computers are connected by more than one cable.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-operations-to-make-network-connected/
// discuss: https://leetcode.com/problems/number-of-operations-to-make-network-connected/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        if n - 1 > connections.len() as i32 {
            return -1;
        }
        let mut num_of_islands = 0;
        let mut map: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
        let mut visited: std::collections::HashSet<i32> = std::collections::HashSet::new();
        //creating adj. list
        for connection in connections {
            map.entry(connection[0]).or_default().push(connection[1]);
            map.entry(connection[1]).or_default().push(connection[0]);
        }
        //looping through all nodes
        for node in 0..n {
            //if a node was never visited we run dfs and increase num
            if !visited.contains(&node) {
                num_of_islands += 1;
                Self::dfs_helper(&map, &mut visited, node);
            }
        }
        num_of_islands - 1
    }

    fn dfs_helper(
        map: &std::collections::HashMap<i32, Vec<i32>>,
        mut visited: &mut std::collections::HashSet<i32>,
        curr: i32,
    ) {
        if visited.contains(&curr) {
            return;
        }
        visited.insert(curr);
        if let Some(neighbors) = map.get(&curr) {
            for neighbor in neighbors {
                Self::dfs_helper(&map, &mut visited, *neighbor);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1319_example_1() {
        let n = 4;
        let connections = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
        let result = 1;

        assert_eq!(Solution::make_connected(n, connections), result);
    }

    #[test]
    fn test_1319_example_2() {
        let n = 6;
        let connections = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]];
        let result = 2;

        assert_eq!(Solution::make_connected(n, connections), result);
    }

    #[test]
    fn test_1319_example_3() {
        let n = 6;
        let connections = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]];
        let result = -1;

        assert_eq!(Solution::make_connected(n, connections), result);
    }
}
