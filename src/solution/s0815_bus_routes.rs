/**
 * [0815] Bus Routes
 *
 * You are given an array routes representing bus routes where routes[i] is a bus route that the i^th bus repeats forever.
 *
 * 	For example, if routes[0] = [1, 5, 7], this means that the 0^th bus travels in the sequence 1 -> 5 -> 7 -> 1 -> 5 -> 7 -> 1 -> ... forever.
 *
 * You will start at the bus stop source (You are not on any bus initially), and you want to go to the bus stop target. You can travel between bus stops by buses only.
 * Return the least number of buses you must take to travel from source to target. Return -1 if it is not possible.
 *  
 * Example 1:
 *
 * Input: routes = [[1,2,7],[3,6,7]], source = 1, target = 6
 * Output: 2
 * Explanation: The best strategy is take the first bus to the bus stop 7, then take the second bus to the bus stop 6.
 *
 * Example 2:
 *
 * Input: routes = [[7,12],[4,5,15],[6],[15,19],[9,12,13]], source = 15, target = 12
 * Output: -1
 *
 *  
 * Constraints:
 *
 * 	1 <= routes.length <= 500.
 * 	1 <= routes[i].length <= 10^5
 * 	All the values of routes[i] are unique.
 * 	sum(routes[i].length) <= 10^5
 * 	0 <= routes[i][j] < 10^6
 * 	0 <= source, target < 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/bus-routes/
// discuss: https://leetcode.com/problems/bus-routes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/bus-routes/discuss/873659/Rust-translated-44ms-100
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        let n = routes.len();
        let mut stop2routes =
            std::collections::HashMap::<i32, std::collections::HashSet<i32>>::new();

        for (i, route) in routes.iter().enumerate() {
            for &j in route {
                stop2routes.entry(j).or_default().insert(i as i32);
            }
        }

        let mut q = std::collections::VecDeque::<(i32, i32)>::new();
        let mut visited = std::collections::HashSet::<i32>::new();
        q.push_back((source, 0));
        visited.insert(source);

        let mut visited_routes = vec![false; n];

        while !q.is_empty() {
            let (stop, bus) = q.pop_front().unwrap();
            if stop == target {
                return bus;
            }
            for &i in stop2routes.get(&stop).unwrap() {
                if visited_routes[i as usize] {
                    continue;
                }
                for &j in &routes[i as usize] {
                    if !visited.contains(&j) {
                        visited.insert(j);
                        q.push_back((j, bus + 1))
                    }
                }
                visited_routes[i as usize] = true;
            }
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0815_example_1() {
        let routes = vec![vec![1, 2, 7], vec![3, 6, 7]];
        let source = 1;
        let target = 6;
        let result = 2;

        assert_eq!(
            Solution::num_buses_to_destination(routes, source, target),
            result
        );
    }

    #[test]
    fn test_0815_example_2() {
        let routes = vec![
            vec![7, 12],
            vec![4, 5, 15],
            vec![6],
            vec![15, 19],
            vec![9, 12, 13],
        ];
        let source = 15;
        let target = 12;
        let result = -1;

        assert_eq!(
            Solution::num_buses_to_destination(routes, source, target),
            result
        );
    }
}
