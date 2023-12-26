/**
 * [1436] Destination City
 *
 * You are given the array paths, where paths[i] = [cityAi, cityBi] means there exists a direct path going from cityAi to cityBi. Return the destination city, that is, the city without any path outgoing to another city.
 * It is guaranteed that the graph of paths forms a line without any loop, therefore, there will be exactly one destination city.
 *  
 * Example 1:
 *
 * Input: paths = [["London","New York"],["New York","Lima"],["Lima","Sao Paulo"]]
 * Output: "Sao Paulo"
 * Explanation: Starting at "London" city you will reach "Sao Paulo" city which is the destination city. Your trip consist of: "London" -> "New York" -> "Lima" -> "Sao Paulo".
 *
 * Example 2:
 *
 * Input: paths = [["B","C"],["D","B"],["C","A"]]
 * Output: "A"
 * Explanation: All possible trips are:
 * "D" -> "B" -> "C" -> "A".
 * "B" -> "C" -> "A".
 * "C" -> "A".
 * "A".
 * Clearly the destination city is "A".
 *
 * Example 3:
 *
 * Input: paths = [["A","Z"]]
 * Output: "Z"
 *
 *  
 * Constraints:
 *
 * 	1 <= paths.length <= 100
 * 	paths[i].length == 2
 * 	1 <= cityAi.length, cityBi.length <= 10
 * 	cityAi != cityBi
 * 	All strings consist of lowercase and uppercase English letters and the space character.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/destination-city/
// discuss: https://leetcode.com/problems/destination-city/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let sources: std::collections::HashSet<String> =
            paths.iter().map(|r| r[0].clone()).collect::<_>();
        paths
            .iter()
            .map(|r| r[1].clone())
            .find(|d| !sources.contains(d))
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1436_example_1() {
        let paths = vec![
            vec_string!["London", "New York"],
            vec_string!["New York", "Lima"],
            vec_string!["Lima", "Sao Paulo"],
        ];

        let result = "Sao Paulo".to_string();

        assert_eq!(Solution::dest_city(paths), result);
    }

    #[test]
    fn test_1436_example_2() {
        let paths = vec![
            vec_string!["B", "C"],
            vec_string!["D", "B"],
            vec_string!["C", "A"],
        ];

        let result = "A".to_string();

        assert_eq!(Solution::dest_city(paths), result);
    }

    #[test]
    fn test_1436_example_3() {
        let paths = vec![vec_string!["A", "Z"]];

        let result = "Z".to_string();

        assert_eq!(Solution::dest_city(paths), result);
    }
}
