/**
 * [0983] Minimum Cost For Tickets
 *
 * You have planned some train traveling one year in advance. The days of the year in which you will travel are given as an integer array days. Each day is an integer from 1 to 365.
 * Train tickets are sold in three different ways:
 *
 * 	a 1-day pass is sold for costs[0] dollars,
 * 	a 7-day pass is sold for costs[1] dollars, and
 * 	a 30-day pass is sold for costs[2] dollars.
 *
 * The passes allow that many days of consecutive travel.
 *
 * 	For example, if we get a 7-day pass on day 2, then we can travel for 7 days: 2, 3, 4, 5, 6, 7, and 8.
 *
 * Return the minimum number of dollars you need to travel every day in the given list of days.
 *  
 * Example 1:
 *
 * Input: days = [1,4,6,7,8,20], costs = [2,7,15]
 * Output: 11
 * Explanation: For example, here is one way to buy passes that lets you travel your travel plan:
 * On day 1, you bought a 1-day pass for costs[0] = $2, which covered day 1.
 * On day 3, you bought a 7-day pass for costs[1] = $7, which covered days 3, 4, ..., 9.
 * On day 20, you bought a 1-day pass for costs[0] = $2, which covered day 20.
 * In total, you spent $11 and covered all the days of your travel.
 *
 * Example 2:
 *
 * Input: days = [1,2,3,4,5,6,7,8,9,10,30,31], costs = [2,7,15]
 * Output: 17
 * Explanation: For example, here is one way to buy passes that lets you travel your travel plan:
 * On day 1, you bought a 30-day pass for costs[2] = $15 which covered days 1, 2, ..., 30.
 * On day 31, you bought a 1-day pass for costs[0] = $2 which covered day 31.
 * In total, you spent $17 and covered all the days of your travel.
 *
 *  
 * Constraints:
 *
 * 	1 <= days.length <= 365
 * 	1 <= days[i] <= 365
 * 	days is in strictly increasing order.
 * 	costs.length == 3
 * 	1 <= costs[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-for-tickets/
// discuss: https://leetcode.com/problems/minimum-cost-for-tickets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-cost-for-tickets/solutions/810974/rust-translated-0ms/
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let last = days[days.len() - 1] as usize;
        let mut min_cost = [0; 366];
        let mut is_travel = [false; 366];
        for &day in &days {
            is_travel[day as usize] = true;
        }
        for i in 1..=last as i32 {
            if !is_travel[i as usize] {
                min_cost[i as usize] = min_cost[i as usize - 1];
                continue;
            }
            let mut min = min_cost[i as usize - 1] + costs[0];
            min = std::cmp::min(min, min_cost[std::cmp::max(i - 7, 0) as usize] + costs[1]);
            min = std::cmp::min(min, min_cost[std::cmp::max(i - 30, 0) as usize] + costs[2]);
            min_cost[i as usize] = min;
        }
        min_cost[last]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0983_example_1() {
        let days = vec![1, 4, 6, 7, 8, 20];
        let costs = vec![2, 7, 15];
        let result = 11;

        assert_eq!(Solution::mincost_tickets(days, costs), result);
    }

    #[test]
    fn test_0983_example_2() {
        let days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31];
        let costs = vec![2, 7, 15];
        let result = 17;

        assert_eq!(Solution::mincost_tickets(days, costs), result);
    }
}
