/**
 * [332] Reconstruct Itinerary
 *
 * You are given a list of airline tickets where tickets[i] = [fromi, toi] represent the departure and the arrival airports of one flight. Reconstruct the itinerary in order and return it.
 * All of the tickets belong to a man who departs from "JFK", thus, the itinerary must begin with "JFK". If there are multiple valid itineraries, you should return the itinerary that has the smallest lexical order when read as a single string.
 *
 * 	For example, the itinerary ["JFK", "LGA"] has a smaller lexical order than ["JFK", "LGB"].
 *
 * You may assume all tickets form at least one valid itinerary. You must use all the tickets once and only once.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/itinerary1-graph.jpg" style="width: 382px; height: 222px;" />
 * Input: tickets = [["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]
 * Output: ["JFK","MUC","LHR","SFO","SJC"]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/itinerary2-graph.jpg" style="width: 222px; height: 230px;" />
 * Input: tickets = [["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]]
 * Output: ["JFK","ATL","JFK","SFO","ATL","SFO"]
 * Explanation: Another possible reconstruction is ["JFK","SFO","ATL","JFK","ATL","SFO"] but it is larger in lexical order.
 *
 *  
 * Constraints:
 *
 * 	1 <= tickets.length <= 300
 * 	tickets[i].length == 2
 * 	fromi.length == 3
 * 	toi.length == 3
 * 	fromi and toi consist of uppercase English letters.
 * 	fromi != toi
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reconstruct-itinerary/
// discuss: https://leetcode.com/problems/reconstruct-itinerary/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/reconstruct-itinerary/discuss/711550/Rust-solution
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph: std::collections::HashMap<
            &str,
            std::collections::BinaryHeap<std::cmp::Reverse<&str>>,
        > = std::collections::HashMap::new();
        for ticket in tickets.iter() {
            graph
                .entry(&ticket[0])
                .or_insert_with(std::collections::BinaryHeap::new)
                .push(std::cmp::Reverse(&ticket[1]));
        }
        let mut result: Vec<String> = Vec::with_capacity(tickets.len() + 1);
        let mut stack: Vec<&str> = vec!["JFK"];
        while let Some(src) = stack.last() {
            if let Some(dsts) = graph.get_mut(src) {
                if !dsts.is_empty() {
                    if let Some(dst) = dsts.pop() {
                        stack.push(dst.0);
                    }
                    continue;
                }
            }
            if let Some(last) = stack.pop() {
                result.push(last.to_string());
            }
        }
        result.reverse();
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0332_example_1() {
        let tickets = vec![
            vec_string!["MUC", "LHR"],
            vec_string!["JFK", "MUC"],
            vec_string!["SFO", "SJC"],
            vec_string!["LHR", "SFO"],
        ];
        let result = vec_string!["JFK", "MUC", "LHR", "SFO", "SJC"];

        assert_eq!(Solution::find_itinerary(tickets), result);
    }

    #[test]
    fn test_0332_example_2() {
        let tickets = vec![
            vec_string!["JFK", "SFO"],
            vec_string!["JFK", "ATL"],
            vec_string!["SFO", "ATL"],
            vec_string!["ATL", "JFK"],
            vec_string!["ATL", "SFO"],
        ];
        let result = vec_string!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"];

        assert_eq!(Solution::find_itinerary(tickets), result);
    }
}
