/**
 * [306] Additive Number
 *
 * Additive number is a string whose digits can form additive sequence.
 * A valid additive sequence should contain at least three numbers. Except for the first two numbers, each subsequent number in the sequence must be the sum of the preceding two.
 * Given a string containing only digits '0'-'9', write a function to determine if it's an additive number.
 * Note: Numbers in the additive sequence cannot have leading zeros, so sequence 1, 2, 03 or 1, 02, 3 is invalid.
 *  
 * Example 1:
 *
 * Input: "112358"
 * Output: true
 * Explanation: The digits can form an additive sequence: 1, 1, 2, 3, 5, 8.
 *              1 + 1 = 2, 1 + 2 = 3, 2 + 3 = 5, 3 + 5 = 8
 *
 * Example 2:
 *
 * Input: "199100199"
 * Output: true
 * Explanation: The additive sequence is: 1, 99, 100, 199.
 *              1 + 99 = 100, 99 + 100 = 199
 *
 *  
 * Constraints:
 *
 * 	<font face="monospace">num </font>consists only of digits '0'-'9'.
 * 	1 <= num.length <= 35
 *
 * Follow up:<br />
 * How would you handle overflow for very large input integers?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/additive-number/
// discuss: https://leetcode.com/problems/additive-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        for i in 1..num.len() - 1 {
            if i > 1 && &num[0..1] == "0" {
                continue;
            }
            for j in i + 1..num.len() {
                if j - i > 1 && &num[i..i + 1] == "0" {
                    continue;
                }
                let mut n1 = (&num[0..i]).parse::<u64>().unwrap();
                let mut n2 = (&num[i..j]).parse::<u64>().unwrap();
                let mut s = (&num[0..j]).to_string();
                loop {
                    let n3 = n1 + n2;
                    n1 = n2;
                    n2 = n3;
                    s += n3.to_string().as_str();
                    if s == num {
                        return true;
                    }
                    if !num.starts_with(&s) {
                        break;
                    }
                }
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0306_example_1() {
        let num = "112358".to_string();
        let result = true;

        assert_eq!(Solution::is_additive_number(num), result);
    }

    #[test]
    fn test_0306_example_2() {
        let num = "199100199".to_string();
        let result = true;

        assert_eq!(Solution::is_additive_number(num), result);
    }
}
