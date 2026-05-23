/**
 * [2477] Minimum Fuel Cost to Report to the Capital
 *
 * There is a tree (i.e., a connected, undirected graph with no cycles) structure country network consisting of n cities numbered from 0 to n - 1 and exactly n - 1 roads. The capital city is city 0. You are given a 2D integer array roads where roads[i] = [ai, bi] denotes that there exists a bidirectional road connecting cities ai and bi.
 * There is a meeting for the representatives of each city. The meeting is in the capital city.
 * There is a car in each city. You are given an integer seats that indicates the number of seats in each car.
 * A representative can use the car in their city to travel or change the car and ride with another representative. The cost of traveling between two cities is one liter of fuel.
 * Return the minimum number of liters of fuel to reach the capital city.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/09/22/a4c380025e3ff0c379525e96a7d63a3.png" style="width: 303px; height: 332px;" />
 * Input: roads = [[0,1],[0,2],[0,3]], seats = 5
 * Output: 3
 * Explanation:
 * - Representative1 goes directly to the capital with 1 liter of fuel.
 * - Representative2 goes directly to the capital with 1 liter of fuel.
 * - Representative3 goes directly to the capital with 1 liter of fuel.
 * It costs 3 liters of fuel at minimum.
 * It can be proven that 3 is the minimum number of liters of fuel needed.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/11/16/2.png" style="width: 274px; height: 340px;" />
 * Input: roads = [[3,1],[3,2],[1,0],[0,4],[0,5],[4,6]], seats = 2
 * Output: 7
 * Explanation:
 * - Representative2 goes directly to city 3 with 1 liter of fuel.
 * - Representative2 and representative3 go together to city 1 with 1 liter of fuel.
 * - Representative2 and representative3 go together to the capital with 1 liter of fuel.
 * - Representative1 goes directly to the capital with 1 liter of fuel.
 * - Representative5 goes directly to the capital with 1 liter of fuel.
 * - Representative6 goes directly to city 4 with 1 liter of fuel.
 * - Representative4 and representative6 go together to the capital with 1 liter of fuel.
 * It costs 7 liters of fuel at minimum.
 * It can be proven that 7 is the minimum number of liters of fuel needed.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/09/27/efcf7f7be6830b8763639cfd01b690a.png" style="width: 108px; height: 86px;" />
 * Input: roads = [], seats = 1
 * Output: 0
 * Explanation: No representatives need to travel to the capital city.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 * 	roads.length == n - 1
 * 	roads[i].length == 2
 * 	0 <= ai, bi < n
 * 	ai != bi
 * 	roads represents a valid tree.
 * 	1 <= seats <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-fuel-cost-to-report-to-the-capital/
// discuss: https://leetcode.com/problems/minimum-fuel-cost-to-report-to-the-capital/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2477_example_1() {
        let roads = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let seats = 5;

        let result = 3;

        assert_eq!(Solution::minimum_fuel_cost(roads, seats), result);
    }

    #[test]
    #[ignore]
    fn test_2477_example_2() {
        let roads = vec![
            vec![3, 1],
            vec![3, 2],
            vec![1, 0],
            vec![0, 4],
            vec![0, 5],
            vec![4, 6],
        ];
        let seats = 2;

        let result = 7;

        assert_eq!(Solution::minimum_fuel_cost(roads, seats), result);
    }

    #[test]
    #[ignore]
    fn test_2477_example_3() {
        let roads: Vec<Vec<i32>> = vec![];
        let seats = 1;

        let result = 0;

        assert_eq!(Solution::minimum_fuel_cost(roads, seats), result);
    }
}
