/**
 * [2039] The Time When the Network Becomes Idle
 *
 * There is a network of n servers, labeled from 0 to n - 1. You are given a 2D integer array edges, where edges[i] = [ui, vi] indicates there is a message channel between servers ui and vi, and they can pass any number of messages to each other directly in one second. You are also given a 0-indexed integer array patience of length n.
 * All servers are connected, i.e., a message can be passed from one server to any other server(s) directly or indirectly through the message channels.
 * The server labeled 0 is the master server. The rest are data servers. Each data server needs to send its message to the master server for processing and wait for a reply. Messages move between servers optimally, so every message takes the least amount of time to arrive at the master server. The master server will process all newly arrived messages instantly and send a reply to the originating server via the reversed path the message had gone through.
 * At the beginning of second 0, each data server sends its message to be processed. Starting from second 1, at the beginning of every second, each data server will check if it has received a reply to the message it sent (including any newly arrived replies) from the master server:
 *
 * 	If it has not, it will resend the message periodically. The data server i will resend the message every patience[i] second(s), i.e., the data server i will resend the message if patience[i] second(s) have elapsed since the last time the message was sent from this server.
 * 	Otherwise, no more resending will occur from this server.
 *
 * The network becomes idle when there are no messages passing between servers or arriving at servers.
 * Return the earliest second starting from which the network becomes idle.
 *  
 * Example 1:
 * <img alt="example 1" src="https://assets.leetcode.com/uploads/2021/09/22/quiet-place-example1.png" style="width: 750px; height: 384px;" />
 * Input: edges = [[0,1],[1,2]], patience = [0,2,1]
 * Output: 8
 * Explanation:
 * At (the beginning of) second 0,
 * - Data server 1 sends its message (denoted 1A) to the master server.
 * - Data server 2 sends its message (denoted 2A) to the master server.
 * At second 1,
 * - Message 1A arrives at the master server. Master server processes message 1A instantly and sends a reply 1A back.
 * - Server 1 has not received any reply. 1 second (1 < patience[1] = 2) elapsed since this server has sent the message, therefore it does not resend the message.
 * - Server 2 has not received any reply. 1 second (1 == patience[2] = 1) elapsed since this server has sent the message, therefore it resends the message (denoted 2B).
 * At second 2,
 * - The reply 1A arrives at server 1. No more resending will occur from server 1.
 * - Message 2A arrives at the master server. Master server processes message 2A instantly and sends a reply 2A back.
 * - Server 2 resends the message (denoted 2C).
 * ...
 * At second 4,
 * - The reply 2A arrives at server 2. No more resending will occur from server 2.
 * ...
 * At second 7, reply 2D arrives at server 2.
 * Starting from the beginning of the second 8, there are no messages passing between servers or arriving at servers.
 * This is the time when the network becomes idle.
 *
 * Example 2:
 * <img alt="example 2" src="https://assets.leetcode.com/uploads/2021/09/04/network_a_quiet_place_2.png" style="width: 100px; height: 85px;" />
 * Input: edges = [[0,1],[0,2],[1,2]], patience = [0,10,10]
 * Output: 3
 * Explanation: Data servers 1 and 2 receive a reply back at the beginning of second 2.
 * From the beginning of the second 3, the network becomes idle.
 *
 *  
 * Constraints:
 *
 * 	n == patience.length
 * 	2 <= n <= 10^5
 * 	patience[0] == 0
 * 	1 <= patience[i] <= 10^5 for 1 <= i < n
 * 	1 <= edges.length <= min(10^5, n * (n - 1) / 2)
 * 	edges[i].length == 2
 * 	0 <= ui, vi < n
 * 	ui != vi
 * 	There are no duplicate edges.
 * 	Each server can directly or indirectly reach another server.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/the-time-when-the-network-becomes-idle/
// discuss: https://leetcode.com/problems/the-time-when-the-network-becomes-idle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2039_example_1() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let patience = vec![0, 2, 1];

        let result = 3;

        assert_eq!(Solution::network_becomes_idle(edges, patience), result);
    }

    #[test]
    #[ignore]
    fn test_2039_example_2() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
        let patience = vec![0, 10, 10];

        let result = 3;

        assert_eq!(Solution::network_becomes_idle(edges, patience), result);
    }
}
