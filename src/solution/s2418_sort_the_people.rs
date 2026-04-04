/**
 * [2418] Sort the People
 *
 * You are given an array of strings names, and an array heights that consists of distinct positive integers. Both arrays are of length n.
 * For each index i, names[i] and heights[i] denote the name and height of the i^th person.
 * Return names sorted in descending order by the people's heights.
 *  
 * Example 1:
 *
 * Input: names = ["Mary","John","Emma"], heights = [180,165,170]
 * Output: ["Mary","Emma","John"]
 * Explanation: Mary is the tallest, followed by Emma and John.
 *
 * Example 2:
 *
 * Input: names = ["Alice","Bob","Bob"], heights = [155,185,150]
 * Output: ["Bob","Alice","Bob"]
 * Explanation: The first Bob is the tallest, followed by Alice and the second Bob.
 *
 *  
 * Constraints:
 *
 * 	n == names.length == heights.length
 * 	1 <= n <= 10^3
 * 	1 <= names[i].length <= 20
 * 	1 <= heights[i] <= 10^5
 * 	names[i] consists of lower and upper case English letters.
 * 	All the values of heights are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-the-people/
// discuss: https://leetcode.com/problems/sort-the-people/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut order = Vec::from_iter(0..names.len());
        let mut names = names;
        order.sort_unstable_by_key(|i| std::cmp::Reverse(heights[*i]));
        order
            .into_iter()
            .map(|i| std::mem::take(&mut names[i]))
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2418_example_1() {
        let names = vec_string!["Mary", "John", "Emma"];
        let heights = vec![180, 165, 170];

        let result = vec_string!["Mary", "Emma", "John"];

        assert_eq!(Solution::sort_people(names, heights), result);
    }

    #[test]
    fn test_2418_example_2() {
        let names = vec_string!["Alice", "Bob", "Bob"];
        let heights = vec![155, 185, 150];

        let result = vec_string!["Bob", "Alice", "Bob"];

        assert_eq!(Solution::sort_people(names, heights), result);
    }
}
