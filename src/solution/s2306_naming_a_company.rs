/**
 * [2306] Naming a Company
 *
 * You are given an array of strings ideas that represents a list of names to be used in the process of naming a company. The process of naming a company is as follows:
 * <ol>
 * 	Choose 2 distinct names from ideas, call them ideaA and ideaB.
 * 	Swap the first letters of ideaA and ideaB with each other.
 * 	If both of the new names are not found in the original ideas, then the name ideaA ideaB (the concatenation of ideaA and ideaB, separated by a space) is a valid company name.
 * 	Otherwise, it is not a valid name.
 * </ol>
 * Return the number of distinct valid names for the company.
 *  
 * Example 1:
 *
 * Input: ideas = ["coffee","donuts","time","toffee"]
 * Output: 6
 * Explanation: The following selections are valid:
 * - ("coffee", "donuts"): The company name created is "doffee conuts".
 * - ("donuts", "coffee"): The company name created is "conuts doffee".
 * - ("donuts", "time"): The company name created is "tonuts dime".
 * - ("donuts", "toffee"): The company name created is "tonuts doffee".
 * - ("time", "donuts"): The company name created is "dime tonuts".
 * - ("toffee", "donuts"): The company name created is "doffee tonuts".
 * Therefore, there are a total of 6 distinct company names.
 * The following are some examples of invalid selections:
 * - ("coffee", "time"): The name "toffee" formed after swapping already exists in the original array.
 * - ("time", "toffee"): Both names are still the same after swapping and exist in the original array.
 * - ("coffee", "toffee"): Both names formed after swapping already exist in the original array.
 *
 * Example 2:
 *
 * Input: ideas = ["lack","back"]
 * Output: 0
 * Explanation: There are no valid selections. Therefore, 0 is returned.
 *
 *  
 * Constraints:
 *
 * 	2 <= ideas.length <= 5 * 10^4
 * 	1 <= ideas[i].length <= 10
 * 	ideas[i] consists of lowercase English letters.
 * 	All the strings in ideas are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/naming-a-company/
// discuss: https://leetcode.com/problems/naming-a-company/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2306_example_1() {
        let ideas = vec_string!["coffee", "donuts", "time", "toffee"];

        let result = 6;

        assert_eq!(Solution::distinct_names(ideas), result);
    }

    #[test]
    #[ignore]
    fn test_2306_example_2() {
        let ideas = vec_string!["lack", "back"];

        let result = 0;

        assert_eq!(Solution::distinct_names(ideas), result);
    }
}
