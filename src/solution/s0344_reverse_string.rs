/**
 * [344] Reverse String
 *
 * Write a function that reverses a string. The input string is given as an array of characters s.
 *  
 * Example 1:
 * Input: s = ["h","e","l","l","o"]
 * Output: ["o","l","l","e","h"]
 * Example 2:
 * Input: s = ["H","a","n","n","a","h"]
 * Output: ["h","a","n","n","a","H"]
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is a <a href="https://en.wikipedia.org/wiki/ASCII#Printable_characters" target="_blank">printable ascii character</a>.
 *
 *  
 * Follow up: Do not allocate extra space for another array. You must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-string/
// discuss: https://leetcode.com/problems/reverse-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        let m = n / 2;
        for i in 0..m {
            s.swap(i, n - i - 1);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0344_example_1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        let result = vec!['o', 'l', 'l', 'e', 'h'];

        Solution::reverse_string(&mut s);

        assert_eq!(s, result);
    }

    #[test]
    fn test_0344_example_2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        let result = vec!['h', 'a', 'n', 'n', 'a', 'H'];

        Solution::reverse_string(&mut s);

        assert_eq!(s, result);
    }
}
