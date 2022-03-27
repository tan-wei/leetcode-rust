/**
 * [0599] Minimum Index Sum of Two Lists
 *
 * Suppose Andy and Doris want to choose a restaurant for dinner, and they both have a list of favorite restaurants represented by strings.
 * You need to help them find out their common interest with the least list index sum. If there is a choice tie between answers, output all of them with no order requirement. You could assume there always exists an answer.
 *  
 * Example 1:
 *
 * Input: list1 = ["Shogun","Tapioca Express","Burger King","KFC"], list2 = ["Piatti","The Grill at Torrey Pines","Hungry Hunter Steakhouse","Shogun"]
 * Output: ["Shogun"]
 * Explanation: The only restaurant they both like is "Shogun".
 *
 * Example 2:
 *
 * Input: list1 = ["Shogun","Tapioca Express","Burger King","KFC"], list2 = ["KFC","Shogun","Burger King"]
 * Output: ["Shogun"]
 * Explanation: The restaurant they both like and have the least index sum is "Shogun" with index sum 1 (0+1).
 *
 *  
 * Constraints:
 *
 * 	1 <= list1.length, list2.length <= 1000
 * 	1 <= list1[i].length, list2[i].length <= 30
 * 	list1[i] and list2[i] consist of spaces ' ' and English letters.
 * 	All the stings of list1 are unique.
 * 	All the stings of list2 are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-index-sum-of-two-lists/
// discuss: https://leetcode.com/problems/minimum-index-sum-of-two-lists/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let m1 = list1
            .iter()
            .enumerate()
            .map(|(x, y)| (y, x))
            .collect::<std::collections::HashMap<&String, usize>>();

        let result = list2
            .iter()
            .enumerate()
            .flat_map(|(l2_pos, rest)| {
                m1.get(rest)
                    .map(|l1_pos| (l1_pos + l2_pos, rest))
                    .into_iter()
            })
            .collect::<Vec<(usize, &String)>>();

        match result.iter().map(|(pos, _)| pos).min() {
            Some(&min) => result
                .iter()
                .filter(|&&(pos, _)| pos == min)
                .map(|&(_, rest)| String::from(rest))
                .collect(),
            None => vec![],
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0599_example_1() {
        let list1 = vec_string!["Shogun", "Tapioca Express", "Burger King", "KFC"];
        let list2 = vec_string![
            "Piatti",
            "The Grill at Torrey Pines",
            "Hungry Hunter Steakhouse",
            "Shogun"
        ];
        let result = vec_string!["Shogun"];

        assert_eq!(Solution::find_restaurant(list1, list2), result);
    }

    #[test]
    fn test_0599_example_2() {
        let list1 = vec_string!["Shogun", "Tapioca Express", "Burger King", "KFC"];
        let list2 = vec_string!["KFC", "Shogun", "Burger King"];
        let result = vec_string!["Shogun"];

        assert_eq!(Solution::find_restaurant(list1, list2), result);
    }
}
