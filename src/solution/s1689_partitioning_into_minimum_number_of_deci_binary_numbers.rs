/**
 * [1689] Partitioning Into Minimum Number Of Deci-Binary Numbers
 *
 * A decimal number is called deci-binary if each of its digits is either 0 or 1 without any leading zeros. For example, 101 and 1100 are deci-binary, while 112 and 3001 are not.
 * Given a string n that represents a positive decimal integer, return the minimum number of positive deci-binary numbers needed so that they sum up to n.
 *  
 * Example 1:
 *
 * Input: n = "32"
 * Output: 3
 * Explanation: 10 + 11 + 11 = 32
 *
 * Example 2:
 *
 * Input: n = "82734"
 * Output: 8
 *
 * Example 3:
 *
 * Input: n = "27346209830709182346"
 * Output: 9
 *
 *  
 * Constraints:
 *
 * 	1 <= n.length <= 10^5
 * 	n consists of only digits.
 * 	n does not contain any leading zeros and represents a positive integer.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/
// discuss: https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .max()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1689_example_1() {
        let n = "32".to_string();

        let result = 3;

        assert_eq!(Solution::min_partitions(n), result);
    }

    #[test]
    fn test_1689_example_2() {
        let n = "82734".to_string();

        let result = 8;

        assert_eq!(Solution::min_partitions(n), result);
    }

    #[test]
    fn test_1689_example_3() {
        let n = "27346209830709182346".to_string();

        let result = 9;

        assert_eq!(Solution::min_partitions(n), result);
    }
}
