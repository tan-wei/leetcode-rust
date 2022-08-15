/**
 * [0791] Custom Sort String
 *
 * You are given two strings order and s. All the characters of order are unique and were sorted in some custom order previously.
 * Permute the characters of s so that they match the order that order was sorted. More specifically, if a character x occurs before a character y in order, then x should occur before y in the permuted string.
 * Return any permutation of s that satisfies this property.
 *  
 * Example 1:
 *
 * Input: order = "cba", s = "abcd"
 * Output: "cbad"
 * Explanation:
 * "a", "b", "c" appear in order, so the order of "a", "b", "c" should be "c", "b", and "a".
 * Since "d" does not appear in order, it can be at any position in the returned string. "dcba", "cdba", "cbda" are also valid outputs.
 *
 * Example 2:
 *
 * Input: order = "cbafg", s = "abcd"
 * Output: "cbad"
 *
 *  
 * Constraints:
 *
 * 	1 <= order.length <= 26
 * 	1 <= s.length <= 200
 * 	order and s consist of lowercase English letters.
 * 	All the characters of order are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/custom-sort-string/
// discuss: https://leetcode.com/problems/custom-sort-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let order_map = order.chars().enumerate().fold([0; 26], |mut acc, (i, c)| {
            acc[c as usize - 'a' as usize] = i;
            acc
        });
        let mut s = s.into_bytes();
        s.sort_unstable_by_key(|&c| order_map[c as usize - 'a' as usize]);
        String::from_utf8(s).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0791_example_1() {
        let order = "cba".to_string();
        let s = "abcd".to_string();
        let result = "cbad".to_string();

        assert_eq!(Solution::custom_sort_string(order, s), result);
    }

    #[test]
    #[ignore]
    fn test_0791_example_2() {
        let order = "cbafg".to_string();
        let s = "abcd".to_string();
        let result = "cbad".to_string();

        assert_eq!(Solution::custom_sort_string(order, s), result);
    }
}
