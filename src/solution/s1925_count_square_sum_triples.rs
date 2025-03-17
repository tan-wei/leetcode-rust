/**
 * [1925] Count Square Sum Triples
 *
 * A square triple (a,b,c) is a triple where a, b, and c are integers and a^2 + b^2 = c^2.
 * Given an integer n, return the number of square triples such that 1 <= a, b, c <= n.
 *  
 * Example 1:
 *
 * Input: n = 5
 * Output: 2
 * Explanation: The square triples are (3,4,5) and (4,3,5).
 *
 * Example 2:
 *
 * Input: n = 10
 * Output: 4
 * Explanation: The square triples are (3,4,5), (4,3,5), (6,8,10), and (8,6,10).
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 250
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-square-sum-triples/
// discuss: https://leetcode.com/problems/count-square-sum-triples/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut count: i32 = 0;

        for i in 1..=n {
            for j in 1..=n {
                for k in 1..=n {
                    if i.pow(2) + j.pow(2) == k.pow(2) {
                        count += 1
                    }
                }
            }
        }

        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1925_example_1() {
        let n = 5;

        let result = 2;

        assert_eq!(Solution::count_triples(n), result);
    }

    #[test]
    fn test_1925_example_2() {
        let n = 10;

        let result = 4;

        assert_eq!(Solution::count_triples(n), result);
    }
}
