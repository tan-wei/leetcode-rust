/**
 * [385] Mini Parser
 *
 * Given a string s represents the serialization of a nested list, implement a parser to deserialize it and return the deserialized NestedInteger.
 * Each element is either an integer or a list whose elements may also be integers or other lists.
 *  
 * Example 1:
 *
 * Input: s = "324"
 * Output: 324
 * Explanation: You should return a NestedInteger object which contains a single integer 324.
 *
 * Example 2:
 *
 * Input: s = "[123,[456,[789]]]"
 * Output: [123,[456,[789]]]
 * Explanation: Return a NestedInteger object containing a nested list with 2 elements:
 * 1. An integer containing value 123.
 * 2. A nested list containing two elements:
 *     i.  An integer containing value 456.
 *     ii. A nested list with one element:
 *          a. An integer containing value 789
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 5 * 10^4
 * 	s consists of digits, square brackets "[]", negative sign '-', and commas ','.
 * 	s is the serialization of valid NestedInteger.
 * 	All the values in the input are in the range [-10^6, 10^6].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/mini-parser/
// discuss: https://leetcode.com/problems/mini-parser/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

impl Solution {
    // Credit: https://leetcode.com/problems/mini-parser/discuss/829813/Rust(Safe)-Iterative-Time-O(N)-Space-O(N)-0ms-2.1mb
    pub fn deserialize(s: String) -> NestedInteger {
        if !&s.starts_with("[") {
            return NestedInteger::Int(s.parse::<i32>().unwrap());
        }

        let mut stack: Vec<NestedInteger> = vec![];
        let mut digit_str: String = String::new();
        for c in s.chars() {
            if c == '[' {
                stack.push(NestedInteger::List(vec![]));
            } else if c == '-' || c.is_digit(10) {
                digit_str.push(c);
            } else if c == ',' {
                if !digit_str.is_empty() {
                    if let Some(v) = stack.last_mut() {
                        if let NestedInteger::List(n) = v {
                            n.push(NestedInteger::Int(digit_str.parse::<i32>().unwrap()));
                        }
                    }
                    digit_str.truncate(0);
                }
            } else {
                if !digit_str.is_empty() {
                    if let Some(v) = stack.last_mut() {
                        if let NestedInteger::List(n) = v {
                            n.push(NestedInteger::Int(digit_str.parse::<i32>().unwrap()));
                        }
                    }
                    digit_str.truncate(0);
                }
                let n = stack.pop().unwrap();
                if stack.is_empty() {
                    return n;
                } else if let Some(v) = stack.last_mut() {
                    if let NestedInteger::List(nst) = v {
                        nst.push(n);
                    }
                }
            }
        }

        NestedInteger::Int(-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0385_example_1() {
        let s = "324".to_string();
        let result = NestedInteger::Int(324);

        assert_eq!(Solution::deserialize(s), result);
    }

    #[test]
    fn test_0385_example_2() {
        let s = "[123,[456,[789]]]".to_string();
        let result = NestedInteger::List(vec![
            NestedInteger::Int(123),
            NestedInteger::List(vec![
                NestedInteger::Int(456),
                NestedInteger::List(vec![NestedInteger::Int(789)]),
            ]),
        ]);

        assert_eq!(Solution::deserialize(s), result);
    }
}
