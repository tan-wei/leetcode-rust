/**
 * [2055] Plates Between Candles
 *
 * There is a long table with a line of plates and candles arranged on top of it. You are given a 0-indexed string s consisting of characters '*' and '|' only, where a '*' represents a plate and a '|' represents a candle.
 * You are also given a 0-indexed 2D integer array queries where queries[i] = [lefti, righti] denotes the substring s[lefti...righti] (inclusive). For each query, you need to find the number of plates between candles that are in the substring. A plate is considered between candles if there is at least one candle to its left and at least one candle to its right in the substring.
 *
 * 	For example, s = "||**||**|*", and a query [3, 8] denotes the substring "*||<u>**</u>|". The number of plates between candles in this substring is 2, as each of the two plates has at least one candle in the substring to its left and right.
 *
 * Return an integer array answer where answer[i] is the answer to the i^th query.
 *  
 * Example 1:
 * <img alt="ex-1" src="https://assets.leetcode.com/uploads/2021/10/04/ex-1.png" style="width: 400px; height: 134px;" />
 * Input: s = "**|**|***|", queries = [[2,5],[5,9]]
 * Output: [2,3]
 * Explanation:
 * - queries[0] has two plates between candles.
 * - queries[1] has three plates between candles.
 *
 * Example 2:
 * <img alt="ex-2" src="https://assets.leetcode.com/uploads/2021/10/04/ex-2.png" style="width: 600px; height: 193px;" />
 * Input: s = "***|**|*****|**||**|*", queries = [[1,17],[4,5],[14,17],[5,11],[15,16]]
 * Output: [9,0,0,0,0]
 * Explanation:
 * - queries[0] has nine plates between candles.
 * - The other queries have zero plates between candles.
 *
 *  
 * Constraints:
 *
 * 	3 <= s.length <= 10^5
 * 	s consists of '*' and '|' characters.
 * 	1 <= queries.length <= 10^5
 * 	queries[i].length == 2
 * 	0 <= lefti <= righti < s.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/plates-between-candles/
// discuss: https://leetcode.com/problems/plates-between-candles/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2055_example_1() {
        let s = "**|**|***|".to_string();
        let queries = vec![vec![2, 5], vec![5, 9]];

        let result = vec![2, 3];

        assert_eq!(Solution::plates_between_candles(s, queries), result);
    }

    #[test]
    #[ignore]
    fn test_2055_example_2() {
        let s = "***|**|*****|**||**|*".to_string();
        let queries = vec![
            vec![1, 17],
            vec![4, 5],
            vec![14, 17],
            vec![5, 11],
            vec![15, 16],
        ];

        let result = vec![9, 0, 0, 0, 0];

        assert_eq!(Solution::plates_between_candles(s, queries), result);
    }
}
