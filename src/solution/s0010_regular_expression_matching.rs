/**
 * [10] Regular Expression Matching
 *
 * Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*' where:
 *
 * 	'.' Matches any single character.​​​​
 * 	'*' Matches zero or more of the preceding element.
 *
 * The matching should cover the entire input string (not partial).
 *  
 * Example 1:
 *
 * Input: s = "aa", p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 *
 * Example 2:
 *
 * Input: s = "aa", p = "a*"
 * Output: true
 * Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
 *
 * Example 3:
 *
 * Input: s = "ab", p = ".*"
 * Output: true
 * Explanation: ".*" means "zero or more (*) of any character (.)".
 *
 * Example 4:
 *
 * Input: s = "aab", p = "c*a*b"
 * Output: true
 * Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore, it matches "aab".
 *
 * Example 5:
 *
 * Input: s = "mississippi", p = "mis*is*p*."
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 20
 * 	0 <= p.length <= 30
 * 	s contains only lowercase English letters.
 * 	p contains only lowercase English letters, '.', and '*'.
 * 	It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/regular-expression-matching/
// discuss: https://leetcode.com/problems/regular-expression-matching/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
enum CharMatcher {
    Any,
    Literal(char),
}

impl CharMatcher {
    pub fn does_match(&self, ch: char) -> bool {
        match self {
            CharMatcher::Any => true,
            CharMatcher::Literal(expected) => *expected == ch,
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Pattern {
    ZeroOrMore(CharMatcher),
    Char(CharMatcher),
}

impl Pattern {
    pub fn can_match_zero_chars(&self) -> bool {
        match self {
            Pattern::Char(_) => false,
            Pattern::ZeroOrMore(_) => true,
        }
    }
}

fn parse_pattern(p: &str) -> Result<Vec<Pattern>, &'static str> {
    fn char_to_matcher(ch: char) -> Result<CharMatcher, &'static str> {
        match ch {
            '.' => Ok(CharMatcher::Any),
            '*' => Err("Literal * not allowed"),
            c => Ok(CharMatcher::Literal(c)),
        }
    }

    let mut patterns = vec![];
    let mut chars = p.chars();
    while let Some(ch) = chars.next() {
        let next_ch_opt = chars.as_str().chars().next();
        if let Some(_) = next_ch_opt.filter(|next_ch| *next_ch == '*') {
            // Skip the '*' char.
            chars.next().unwrap();
            patterns.push(Pattern::ZeroOrMore(char_to_matcher(ch)?));
        } else {
            patterns.push(Pattern::Char(char_to_matcher(ch)?));
        }
    }

    Ok(patterns)
}

fn _is_match<'a>(
    s: &'a str,
    patterns: &'a [Pattern],
    cache: &mut HashMap<(&'a str, &'a [Pattern]), bool>,
) -> bool {
    if s.is_empty() || patterns.is_empty() {
        return s.is_empty() && patterns.iter().all(|p| p.can_match_zero_chars());
    }
    if let Some(res) = cache.get(&(s, patterns)) {
        return *res;
    }
    let first_ch = s.chars().next().unwrap();

    let res = match &patterns[0] {
        Pattern::Char(matcher) => {
            matcher.does_match(first_ch) && _is_match(&s[1..], &patterns[1..], cache)
        }
        Pattern::ZeroOrMore(matcher) => {
            let mut chars = s.chars();
            // Check if one characters leads to a match.
            while let Some(_) = chars.next().filter(|ch| matcher.does_match(*ch)) {
                if _is_match(chars.as_str(), &patterns[1..], cache) {
                    return true;
                }
            }
            // Check if zero characters leads to a match.
            _is_match(s, &patterns[1..], cache)
        }
    };

    cache.insert((s, patterns), res);
    res
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let patterns = parse_pattern(p.as_str()).expect("pattern parsing error");
        _is_match(s.as_str(), patterns.as_slice(), &mut HashMap::new())
    }

    pub fn is_match_v2(s: String, p: String) -> bool {
        Self::is_match_slice(s.as_bytes(), p.as_bytes())
    }

    fn is_match_slice(s: &[u8], p: &[u8]) -> bool {
        match (p, s) {
            ([x, b'*', subp @ ..], [y, subs @ ..]) if *x == b'.' || x == y => {
                Self::is_match_slice(subs, p)
            }
            ([_, b'*', subp @ ..], _) => Self::is_match_slice(s, subp),
            ([x, subp @ ..], [y, subs @ ..]) if *x == b'.' || x == y => {
                Self::is_match_slice(subs, subp)
            }
            ([], s) => s.is_empty(),
            _ => false,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0010_example_1() {
        let s = String::from("aa");
        let p = String::from("a");

        assert_eq!(Solution::is_match(s, p), false);

        let s = String::from("aa");
        let p = String::from("a");

        assert_eq!(Solution::is_match_v2(s, p), false);
    }

    #[test]
    fn test_0010_example_2() {
        let s = String::from("aa");
        let p = String::from("a*");

        assert_eq!(Solution::is_match(s, p), true);

        let s = String::from("aa");
        let p = String::from("a*");

        assert_eq!(Solution::is_match_v2(s, p), true);
    }

    #[test]
    fn test_0010_example_3() {
        let s = String::from("ab");
        let p = String::from(".*");

        assert_eq!(Solution::is_match(s, p), true);

        let s = String::from("ab");
        let p = String::from(".*");

        assert_eq!(Solution::is_match_v2(s, p), true);
    }

    #[test]
    fn test_0010_example_4() {
        let s = String::from("aab");
        let p = String::from("c*a*b");

        assert_eq!(Solution::is_match(s, p), true);

        let s = String::from("aab");
        let p = String::from("c*a*b");

        assert_eq!(Solution::is_match_v2(s, p), true);
    }

    #[test]
    fn test_0010_example_5() {
        let s = String::from("mississippi");
        let p = String::from("mis*is*p*.");

        assert_eq!(Solution::is_match(s, p), false);

        let s = String::from("mississippi");
        let p = String::from("mis*is*p*.");

        assert_eq!(Solution::is_match_v2(s, p), false);
    }
}
