/**
 * [0787] Cheapest Flights Within K Stops
 *
 * There are n cities connected by some number of flights. You are given an array flights where flights[i] = [fromi, toi, pricei] indicates that there is a flight from city fromi to city toi with cost pricei.
 * You are also given three integers src, dst, and k, return the cheapest price from src to dst with at most k stops. If there is no such route, return -1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-3drawio.png" style="width: 332px; height: 392px;" />
 * Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1
 * Output: 700
 * Explanation:
 * The graph is shown above.
 * The optimal path with at most 1 stop from city 0 to 3 is marked in red and has cost 100 + 600 = 700.
 * Note that the path through cities [0,1,2,3] is cheaper but is invalid because it uses 2 stops.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-1drawio.png" style="width: 332px; height: 242px;" />
 * Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 1
 * Output: 200
 * Explanation:
 * The graph is shown above.
 * The optimal path with at most 1 stop from city 0 to 2 is marked in red and has cost 100 + 100 = 200.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-2drawio.png" style="width: 332px; height: 242px;" />
 * Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 0
 * Output: 500
 * Explanation:
 * The graph is shown above.
 * The optimal path with no stops from city 0 to 2 is marked in red and has cost 500.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 100
 * 	0 <= flights.length <= (n * (n - 1) / 2)
 * 	flights[i].length == 3
 * 	0 <= fromi, toi < n
 * 	fromi != toi
 * 	1 <= pricei <= 10^4
 * 	There will not be any multiple flights between two cities.
 * 	0 <= src, dst, k < n
 * 	src != dst
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/cheapest-flights-within-k-stops/
// discuss: https://leetcode.com/problems/cheapest-flights-within-k-stops/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/cheapest-flights-within-k-stops/discuss/2372635/Rust-cheapest-and-best
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (n, src, dst) = (n as usize, src as usize, dst as usize);
        let mut q = std::iter::once((src, 0, 0)).collect::<std::collections::VecDeque<_>>();
        let (mut seen, mut graph) = (vec![i32::MAX; n], vec![vec![]; n]);

        for flight in flights {
            graph[flight[0] as usize].push((flight[1] as usize, flight[2]));
        }

        while let Some((x, stops, cost)) = q.pop_front() {
            graph[x].iter().for_each(|(to, fare)| {
                let new_cost = cost + *fare;
                if new_cost < seen[dst] && new_cost < seen[*to] && stops <= k {
                    seen[*to] = new_cost;
                    if to != &dst {
                        q.push_back((*to, stops + 1, new_cost));
                    }
                }
            })
        }

        (seen[dst] == i32::MAX).then(|| -1).unwrap_or(seen[dst])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0787_example_1() {
        let n = 4;
        let flights = vec![
            vec![0, 1, 100],
            vec![1, 2, 100],
            vec![2, 0, 100],
            vec![1, 3, 600],
            vec![2, 3, 200],
        ];
        let src = 0;
        let dst = 3;
        let k = 1;
        let result = 700;

        assert_eq!(
            Solution::find_cheapest_price(n, flights, src, dst, k),
            result
        );
    }

    #[test]
    fn test_0787_example_2() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 1;
        let result = 200;

        assert_eq!(
            Solution::find_cheapest_price(n, flights, src, dst, k),
            result
        );
    }

    #[test]
    fn test_0787_example_3() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 0;
        let result = 500;

        assert_eq!(
            Solution::find_cheapest_price(n, flights, src, dst, k),
            result
        );
    }
}
