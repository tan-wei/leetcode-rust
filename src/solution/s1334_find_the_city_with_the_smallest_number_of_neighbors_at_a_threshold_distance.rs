/**
 * [1334] Find the City With the Smallest Number of Neighbors at a Threshold Distance
 *
 * There are n cities numbered from 0 to n-1. Given the array edges where edges[i] = [fromi, toi, weighti] represents a bidirectional and weighted edge between cities fromi and toi, and given the integer distanceThreshold.
 * Return the city with the smallest number of cities that are reachable through some path and whose distance is at most distanceThreshold, If there are multiple such cities, return the city with the greatest number.
 * Notice that the distance of a path connecting cities i and j is equal to the sum of the edges' weights along that path.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/16/find_the_city_01.png" style="width: 300px; height: 225px;" />
 * Input: n = 4, edges = [[0,1,3],[1,2,1],[1,3,4],[2,3,1]], distanceThreshold = 4
 * Output: 3
 * Explanation: The figure above describes the graph.
 * The neighboring cities at a distanceThreshold = 4 for each city are:
 * City 0 -> [City 1, City 2]
 * City 1 -> [City 0, City 2, City 3]
 * City 2 -> [City 0, City 1, City 3]
 * City 3 -> [City 1, City 2]
 * Cities 0 and 3 have 2 neighboring cities at a distanceThreshold = 4, but we have to return city 3 since it has the greatest number.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/16/find_the_city_02.png" style="width: 300px; height: 225px;" />
 * Input: n = 5, edges = [[0,1,2],[0,4,8],[1,2,3],[1,4,2],[2,3,1],[3,4,1]], distanceThreshold = 2
 * Output: 0
 * Explanation: The figure above describes the graph.
 * The neighboring cities at a distanceThreshold = 2 for each city are:
 * City 0 -> [City 1]
 * City 1 -> [City 0, City 4]
 * City 2 -> [City 3, City 4]
 * City 3 -> [City 2, City 4]
 * City 4 -> [City 1, City 2, City 3]
 * The city 0 has 1 neighboring city at a distanceThreshold = 2.
 *
 *
 * Constraints:
 *
 * 	2 <= n <= 100
 * 	1 <= edges.length <= n * (n - 1) / 2
 * 	edges[i].length == 3
 * 	0 <= fromi < toi < n
 * 	1 <= weighti, distanceThreshold <= 10^4
 * 	All pairs (fromi, toi) are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/
// discuss: https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/solutions/2468177/rust-solution-using-floyd-warshall/
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let inf = 1_000_000_000;
        let mut memo = vec![vec![inf; n]; n];
        for a in edges {
            let from = a[0] as usize;
            let to = a[1] as usize;
            let cost = a[2];
            memo[from][to] = cost;
            memo[to][from] = cost;
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    memo[i][j] = std::cmp::min(memo[i][j], memo[i][k] + memo[k][j]);
                }
            }
        }

        let mut scores = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                if memo[i][j] <= distance_threshold {
                    scores[i] += 1;
                }
            }
        }

        let mut min = 100000;
        let mut target_index = 0;
        for i in 0..n {
            if min >= scores[i] {
                min = scores[i];
                target_index = i;
            }
        }
        target_index as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1334_example_1() {
        let n = 4;
        let edges = vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]];
        let distance_threshold = 4;

        let result = 3;

        assert_eq!(
            Solution::find_the_city(n, edges, distance_threshold),
            result
        );
    }

    #[test]
    fn test_1334_example_2() {
        let n = 5;
        let edges = vec![
            vec![0, 1, 2],
            vec![0, 4, 8],
            vec![1, 2, 3],
            vec![1, 4, 2],
            vec![2, 3, 1],
            vec![3, 4, 1],
        ];
        let distance_threshold = 2;

        let result = 0;

        assert_eq!(
            Solution::find_the_city(n, edges, distance_threshold),
            result
        );
    }
}
