/**
 * [2606] Find the Substring With Maximum Cost
 *
 * You are given a string s, a string chars of distinct characters and an integer array vals of the same length as chars.
 * The cost of the substring is the sum of the values of each character in the substring. The cost of an empty string is considered 0.
 * The value of the character is defined in the following way:
 *
 * 	If the character is not in the string chars, then its value is its corresponding position (1-indexed) in the alphabet.
 *
 * 		For example, the value of 'a' is 1, the value of 'b' is 2, and so on. The value of 'z' is 26.
 *
 *
 * 	Otherwise, assuming i is the index where the character occurs in the string chars, then its value is vals[i].
 *
 * Return the maximum cost among all substrings of the string s.
 *  
 * Example 1:
 *
 * Input: s = "adaa", chars = "d", vals = [-1000]
 * Output: 2
 * Explanation: The value of the characters "a" and "d" is 1 and -1000 respectively.
 * The substring with the maximum cost is "aa" and its cost is 1 + 1 = 2.
 * It can be proven that 2 is the maximum cost.
 *
 * Example 2:
 *
 * Input: s = "abc", chars = "abc", vals = [-1,-1,-1]
 * Output: 0
 * Explanation: The value of the characters "a", "b" and "c" is -1, -1, and -1 respectively.
 * The substring with the maximum cost is the empty substring "" and its cost is 0.
 * It can be proven that 0 is the maximum cost.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consist of lowercase English letters.
 * 	1 <= chars.length <= 26
 * 	chars consist of distinct lowercase English letters.
 * 	vals.length == chars.length
 * 	-1000 <= vals[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-substring-with-maximum-cost/
// discuss: https://leetcode.com/problems/find-the-substring-with-maximum-cost/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2606_example_1() {
        let s = "adaa".to_string();
        let chars = "d".to_string();
        let vals = vec![-1000];

        let result = 2;

        assert_eq!(Solution::maximum_cost_substring(s, chars, vals), result);
    }

    #[test]
    #[ignore]
    fn test_2606_example_2() {
        let s = "abc".to_string();
        let chars = "abc".to_string();
        let vals = vec![-1, -1, -1];

        let result = 0;

        assert_eq!(Solution::maximum_cost_substring(s, chars, vals), result);
    }
}
