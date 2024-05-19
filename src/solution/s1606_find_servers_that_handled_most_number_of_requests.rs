/**
 * [1606] Find Servers That Handled Most Number of Requests
 *
 * You have k servers numbered from 0 to k-1 that are being used to handle multiple requests simultaneously. Each server has infinite computational capacity but cannot handle more than one request at a time. The requests are assigned to servers according to a specific algorithm:
 *
 * 	The i^th (0-indexed) request arrives.
 * 	If all servers are busy, the request is dropped (not handled at all).
 * 	If the (i % k)^th server is available, assign the request to that server.
 * 	Otherwise, assign the request to the next available server (wrapping around the list of servers and starting from 0 if necessary). For example, if the i^th server is busy, try to assign the request to the (i+1)^th server, then the (i+2)^th server, and so on.
 *
 * You are given a strictly increasing array arrival of positive integers, where arrival[i] represents the arrival time of the i^th request, and another array load, where load[i] represents the load of the i^th request (the time it takes to complete). Your goal is to find the busiest server(s). A server is considered busiest if it handled the most number of requests successfully among all the servers.
 * Return a list containing the IDs (0-indexed) of the busiest server(s). You may return the IDs in any order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/08/load-1.png" style="width: 389px; height: 221px;" />
 * Input: k = 3, arrival = [1,2,3,4,5], load = [5,2,3,3,3]
 * Output: [1]
 * Explanation:
 * All of the servers start out available.
 * The first 3 requests are handled by the first 3 servers in order.
 * Request 3 comes in. Server 0 is busy, so it's assigned to the next available server, which is 1.
 * Request 4 comes in. It cannot be handled since all servers are busy, so it is dropped.
 * Servers 0 and 2 handled one request each, while server 1 handled two requests. Hence server 1 is the busiest server.
 *
 * Example 2:
 *
 * Input: k = 3, arrival = [1,2,3,4], load = [1,2,1,2]
 * Output: [0]
 * Explanation:
 * The first 3 requests are handled by first 3 servers.
 * Request 3 comes in. It is handled by server 0 since the server is available.
 * Server 0 handled two requests, while servers 1 and 2 handled one request each. Hence server 0 is the busiest server.
 *
 * Example 3:
 *
 * Input: k = 3, arrival = [1,2,3], load = [10,12,11]
 * Output: [0,1,2]
 * Explanation: Each server handles a single request, so they are all considered the busiest.
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= 10^5
 * 	1 <= arrival.length, load.length <= 10^5
 * 	arrival.length == load.length
 * 	1 <= arrival[i], load[i] <= 10^9
 * 	arrival is strictly increasing.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-servers-that-handled-most-number-of-requests/
// discuss: https://leetcode.com/problems/find-servers-that-handled-most-number-of-requests/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/find-servers-that-handled-most-number-of-requests/solutions/942408/rust-binary-heap-and-btreeset/
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let k = k as usize;
        let mut servers = std::collections::BTreeSet::new();
        for i in 0..k {
            servers.insert(i);
        }

        let mut active_requests: std::collections::BinaryHeap<std::cmp::Reverse<(i32, usize)>> =
            std::collections::BinaryHeap::new();
        let mut iterator = arrival.iter().zip(load.iter()).peekable();
        let mut requests = vec![0; k];
        let mut i = 0;

        while let Some((&t2, &l)) = iterator.peek() {
            if active_requests.peek().is_none() || (active_requests.peek().unwrap().0).0 > t2 {
                let (time, load) = iterator.next().unwrap();

                if servers.len() > 0 {
                    let s = if let Some(server) = servers.range(i..).next() {
                        *server
                    } else {
                        *servers.range(..).next().unwrap()
                    };
                    servers.remove(&s);
                    requests[s] += 1;
                    active_requests.push(std::cmp::Reverse((time + load, s)));
                }
                i += 1;
                i %= k;
            } else {
                let std::cmp::Reverse((_, server)) = active_requests.pop().unwrap();
                servers.insert(server);
            }
        }

        let max_r = *requests.iter().max().unwrap();
        requests
            .iter()
            .enumerate()
            .filter(|x| *x.1 == max_r)
            .map(|x| x.0 as i32)
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1606_example_1() {
        let k = 3;
        let arrival = vec![1, 2, 3, 4, 5];
        let load = vec![5, 2, 3, 3, 3];

        let result = vec![1];

        assert_eq!(Solution::busiest_servers(k, arrival, load), result);
    }

    #[test]
    fn test_1606_example_2() {
        let k = 3;
        let arrival = vec![1, 2, 3, 4];
        let load = vec![1, 2, 1, 2];

        let result = vec![0];

        assert_eq!(Solution::busiest_servers(k, arrival, load), result);
    }

    #[test]
    fn test_1606_example_3() {
        let k = 3;
        let arrival = vec![1, 2, 3];
        let load = vec![10, 12, 11];

        let result = vec![0, 1, 2];

        assert_eq!(Solution::busiest_servers(k, arrival, load), result);
    }
}
