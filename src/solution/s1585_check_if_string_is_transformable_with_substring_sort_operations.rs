/**
 * [1585] Check If String Is Transformable With Substring Sort Operations
 *
 * Given two strings s and t, transform string s into string t using the following operation any number of times:
 *
 * 	Choose a non-empty substring in s and sort it in place so the characters are in ascending order.
 *
 * 		For example, applying the operation on the underlined substring in "1<u>4234</u>" results in "1<u>2344</u>".
 *
 *
 *
 * Return true if it is possible to transform s into t. Otherwise, return false.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * Example 1:
 *
 * Input: s = "84532", t = "34852"
 * Output: true
 * Explanation: You can transform s into t using the following sort operations:
 * "84<u>53</u>2" (from index 2 to 3) -> "84<u>35</u>2"
 * "<u>843</u>52" (from index 0 to 2) -> "<u>348</u>52"
 *
 * Example 2:
 *
 * Input: s = "34521", t = "23415"
 * Output: true
 * Explanation: You can transform s into t using the following sort operations:
 * "<u>3452</u>1" -> "<u>2345</u>1"
 * "234<u>51</u>" -> "234<u>15</u>"
 *
 * Example 3:
 *
 * Input: s = "12345", t = "12435"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	s.length == t.length
 * 	1 <= s.length <= 10^5
 * 	s and t consist of only digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-string-is-transformable-with-substring-sort-operations/
// discuss: https://leetcode.com/problems/check-if-string-is-transformable-with-substring-sort-operations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/check-if-string-is-transformable-with-substring-sort-operations/solutions/3174804/just-a-runnable-solution/
    pub fn is_transformable(s: String, t: String) -> bool {
        let to_digits = |s: String| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        };

        let s = to_digits(s);
        let t = to_digits(t);

        let mut pos = vec![vec![]; 10];

        for i in 0..s.len() {
            pos[s[i]].push(i);
        }

        for &item_t in &t {
            let c = item_t;
            if pos[c].is_empty() {
                return false;
            }
            let p = pos[c][0];
            for item in pos.iter().take(c) {
                if !item.is_empty() && item[0] < p {
                    return false;
                }
            }
            pos[c].remove(0);
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1585_example_1() {
        let s = "84532".to_string();
        let t = "34852".to_string();

        let result = true;

        assert_eq!(Solution::is_transformable(s, t), result);
    }

    #[test]
    fn test_1585_example_2() {
        let s = "34521".to_string();
        let t = "23415".to_string();

        let result = true;

        assert_eq!(Solution::is_transformable(s, t), result);
    }

    #[test]
    fn test_1585_example_3() {
        let s = "12345".to_string();
        let t = "12435".to_string();

        let result = false;

        assert_eq!(Solution::is_transformable(s, t), result);
    }
}
