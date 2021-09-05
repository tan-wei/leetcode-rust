/**
 * [316] Remove Duplicate Letters
 *
 * Given a string s, remove duplicate letters so that every letter appears once and only once. You must make sure your result is the smallest in lexicographical order among all possible results.
 *  
 * Example 1:
 *
 * Input: s = "bcabc"
 * Output: "abc"
 *
 * Example 2:
 *
 * Input: s = "cbacdcbc"
 * Output: "acdb"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^4
 * 	s consists of lowercase English letters.
 *
 *  
 * Note: This question is the same as 1081: <a href="https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/" target="_blank">https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/</a>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-duplicate-letters/
// discuss: https://leetcode.com/problems/remove-duplicate-letters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/remove-duplicate-letters/discuss/1131952/Rust-cheapest-and-best
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut map = s
            .chars()
            .enumerate()
            .map(|(i, c)| (c, (i, false)))
            .collect::<std::collections::HashMap<char, (usize, bool)>>();

        s.chars()
            .enumerate()
            .fold(vec![' '], |mut stack, (i, a)| {
                if !map.get(&a).unwrap().1 {
                    let mut b = stack.pop().unwrap();
                    while a < b && i < map.get(&b).unwrap().0 {
                        map.get_mut(&b).unwrap().1 = false;
                        b = stack.pop().unwrap();
                    }
                    stack.push(b);
                    stack.push(a);
                    map.get_mut(&a).unwrap().1 = true;
                }
                stack
            })
            .into_iter()
            .skip(1)
            .collect()
    }

    // Credit: https://leetcode.com/problems/remove-duplicate-letters/discuss/889778/Rust-stack-solution
    pub fn remove_duplicate_letters_v2(s: String) -> String {
        let mut counts: [usize; 26] = [0; 26];
        for &b in s.as_bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        let mut v: Vec<char> = Vec::with_capacity(26);
        let mut exists: [bool; 26] = [false; 26];
        for &b in s.as_bytes() {
            let i = (b - b'a') as usize;
            counts[i] -= 1;
            if exists[i] {
                continue;
            }
            while let Some(&last) = v.last() {
                let j = (last as u8 - b'a') as usize;
                if b < last as u8 && counts[j] > 0 {
                    exists[j] = false;
                    v.pop();
                } else {
                    break;
                }
            }
            v.push(b as char);
            exists[i] = true;
        }
        v.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0316_example_1() {
        let s = "bcabc".to_string();
        let result = "abc";

        assert_eq!(Solution::remove_duplicate_letters(s), result);

        let s = "bcabc".to_string();
        let result = "abc";

        assert_eq!(Solution::remove_duplicate_letters_v2(s), result);
    }

    #[test]
    fn test_0316_example_2() {
        let s = "cbacdcbc".to_string();
        let result = "acdb";

        assert_eq!(Solution::remove_duplicate_letters(s), result);

        let s = "cbacdcbc".to_string();
        let result = "acdb";

        assert_eq!(Solution::remove_duplicate_letters_v2(s), result);
    }
}
