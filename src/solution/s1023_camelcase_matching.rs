/**
 * [1023] Camelcase Matching
 *
 * Given an array of strings queries and a string pattern, return a boolean array answer where answer[i] is true if queries[i] matches pattern, and false otherwise.
 * A query word queries[i] matches pattern if you can insert lowercase English letters pattern so that it equals the query. You may insert each character at any position and you may not insert any characters.
 *  
 * Example 1:
 *
 * Input: queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FB"
 * Output: [true,false,true,true,false]
 * Explanation: "FooBar" can be generated like this "F" + "oo" + "B" + "ar".
 * "FootBall" can be generated like this "F" + "oot" + "B" + "all".
 * "FrameBuffer" can be generated like this "F" + "rame" + "B" + "uffer".
 *
 * Example 2:
 *
 * Input: queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FoBa"
 * Output: [true,false,true,false,false]
 * Explanation: "FooBar" can be generated like this "Fo" + "o" + "Ba" + "r".
 * "FootBall" can be generated like this "Fo" + "ot" + "Ba" + "ll".
 *
 * Example 3:
 *
 * Input: queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FoBaT"
 * Output: [false,true,false,false,false]
 * Explanation: "FooBarTest" can be generated like this "Fo" + "o" + "Ba" + "r" + "T" + "est".
 *
 *  
 * Constraints:
 *
 * 	1 <= pattern.length, queries.length <= 100
 * 	1 <= queries[i].length <= 100
 * 	queries[i] and pattern consist of English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/camelcase-matching/
// discuss: https://leetcode.com/problems/camelcase-matching/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut result = Vec::new();
        for query in queries {
            let mut count = 0;
            let mut flag = 0;
            for i in 0..query.len() {
                if count < pattern.len() {
                    if query.chars().nth(i).unwrap() == pattern.chars().nth(count).unwrap() {
                        count += 1
                    } else if query.chars().nth(i).unwrap()
                        == query.to_uppercase().chars().nth(i).unwrap()
                    {
                        flag = 1;
                        break;
                    }
                } else if query[i..] != query[i..].to_lowercase() {
                    flag = 1;
                    break;
                } else {
                    break;
                }
            }
            if flag == 0 && count == pattern.len() {
                result.push(true);
            } else {
                result.push(false);
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1023_example_1() {
        let queries = vec_string![
            "FooBar",
            "FooBarTest",
            "FootBall",
            "FrameBuffer",
            "ForceFeedBack",
        ];
        let pattern = "FB".to_string();
        let result = vec![true, false, true, true, false];

        assert_eq!(Solution::camel_match(queries, pattern), result);
    }

    #[test]
    fn test_1023_example_2() {
        let queries = vec_string![
            "FooBar",
            "FooBarTest",
            "FootBall",
            "FrameBuffer",
            "ForceFeedBack",
        ];
        let pattern = "FoBa".to_string();
        let result = vec![true, false, true, false, false];

        assert_eq!(Solution::camel_match(queries, pattern), result);
    }

    #[test]
    fn test_1023_example_3() {
        let queries = vec_string![
            "FooBar",
            "FooBarTest",
            "FootBall",
            "FrameBuffer",
            "ForceFeedBack",
        ];
        let pattern = "FoBaT".to_string();
        let result = vec![false, true, false, false, false];

        assert_eq!(Solution::camel_match(queries, pattern), result);
    }
}
