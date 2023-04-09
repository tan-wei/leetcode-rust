/**
 * [1029] Two City Scheduling
 *
 * A company is planning to interview 2n people. Given the array costs where costs[i] = [aCosti, bCosti], the cost of flying the i^th person to city a is aCosti, and the cost of flying the i^th person to city b is bCosti.
 * Return the minimum cost to fly every person to a city such that exactly n people arrive in each city.
 *  
 * Example 1:
 *
 * Input: costs = [[10,20],[30,200],[400,50],[30,20]]
 * Output: 110
 * Explanation:
 * The first person goes to city A for a cost of 10.
 * The second person goes to city A for a cost of 30.
 * The third person goes to city B for a cost of 50.
 * The fourth person goes to city B for a cost of 20.
 * The total minimum cost is 10 + 30 + 50 + 20 = 110 to have half the people interviewing in each city.
 *
 * Example 2:
 *
 * Input: costs = [[259,770],[448,54],[926,667],[184,139],[840,118],[577,469]]
 * Output: 1859
 *
 * Example 3:
 *
 * Input: costs = [[515,563],[451,713],[537,709],[343,819],[855,779],[457,60],[650,359],[631,42]]
 * Output: 3086
 *
 *  
 * Constraints:
 *
 * 	2 * n == costs.length
 * 	2 <= costs.length <= 100
 * 	costs.length is even.
 * 	1 <= aCosti, bCosti <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/two-city-scheduling/
// discuss: https://leetcode.com/problems/two-city-scheduling/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut costs = costs;
        costs.sort_unstable_by(|v1, v2| (v1[0] - v1[1]).cmp(&(v2[0] - v2[1])));
        let mut result = 0;
        for i in 0..costs.len() / 2 {
            result += costs[i][0] + costs[i + costs.len() / 2][1];
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1029_example_1() {
        let costs = vec![vec![10, 20], vec![30, 200], vec![400, 50], vec![30, 20]];
        let result = 110;

        assert_eq!(Solution::two_city_sched_cost(costs), result);
    }

    #[test]
    fn test_1029_example_2() {
        let costs = vec![
            vec![259, 770],
            vec![448, 54],
            vec![926, 667],
            vec![184, 139],
            vec![840, 118],
            vec![577, 469],
        ];
        let result = 1859;

        assert_eq!(Solution::two_city_sched_cost(costs), result);
    }

    #[test]
    fn test_1029_example_3() {
        let costs = vec![
            vec![515, 563],
            vec![451, 713],
            vec![537, 709],
            vec![343, 819],
            vec![855, 779],
            vec![457, 60],
            vec![650, 359],
            vec![631, 42],
        ];
        let result = 3086;

        assert_eq!(Solution::two_city_sched_cost(costs), result);
    }
}
