/**
 * [0743] Network Delay Time
 *
 * You are given a network of n nodes, labeled from 1 to n. You are also given times, a list of travel times as directed edges times[i] = (ui, vi, wi), where ui is the source node, vi is the target node, and wi is the time it takes for a signal to travel from source to target.
 * We will send a signal from a given node k. Return the minimum time it takes for all the n nodes to receive the signal. If it is impossible for all the n nodes to receive the signal, return -1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/05/23/931_example_1.png" style="width: 217px; height: 239px;" />
 * Input: times = [[2,1,1],[2,3,1],[3,4,1]], n = 4, k = 2
 * Output: 2
 *
 * Example 2:
 *
 * Input: times = [[1,2,1]], n = 2, k = 1
 * Output: 1
 *
 * Example 3:
 *
 * Input: times = [[1,2,1]], n = 2, k = 2
 * Output: -1
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= n <= 100
 * 	1 <= times.length <= 6000
 * 	times[i].length == 3
 * 	1 <= ui, vi <= n
 * 	ui != vi
 * 	0 <= wi <= 100
 * 	All the pairs (ui, vi) are unique. (i.e., no multiple edges.)
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/network-delay-time/
// discuss: https://leetcode.com/problems/network-delay-time/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/network-delay-time/discuss/1189167/Rust-dijkstra-solution
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let adjacency_list = {
            let mut res = vec![vec![]; n + 1];
            for time in times {
                let u = time[0] as usize;
                let v = time[1] as usize;
                let w = time[2];
                res[u].push((v, w));
            }
            res
        };

        let mut distances = vec![std::i32::MAX; n + 1];
        distances[k] = 0;
        let mut visited = vec![false; n + 1];
        let mut pq = std::collections::BinaryHeap::new();
        pq.push((std::cmp::Reverse(0), k));
        while let Some((std::cmp::Reverse(distance), u)) = pq.pop() {
            if visited[u] {
                continue;
            }
            for &(v, w) in &adjacency_list[u] {
                let new_distance = w + distance;
                if new_distance < distances[v] {
                    distances[v] = new_distance;
                    pq.push((std::cmp::Reverse(new_distance), v));
                }
            }
            visited[u] = true;
        }
        if !visited[1..].iter().all(|&a| a) {
            return -1;
        }
        *distances[1..].iter().max().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0743_example_1() {
        let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
        let n = 4;
        let k = 2;
        let result = 2;

        assert_eq!(Solution::network_delay_time(times, n, k), result);
    }

    #[test]
    fn test_0743_example_2() {
        let times = vec![vec![1, 2, 1]];
        let n = 2;
        let k = 1;
        let result = 1;

        assert_eq!(Solution::network_delay_time(times, n, k), result);
    }

    #[test]
    fn test_0743_example_3() {
        let times = vec![vec![1, 2, 1]];
        let n = 2;
        let k = 2;
        let result = -1;

        assert_eq!(Solution::network_delay_time(times, n, k), result);
    }
}
