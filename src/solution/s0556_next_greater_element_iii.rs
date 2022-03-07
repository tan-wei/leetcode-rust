/**
 * [0556] Next Greater Element III
 *
 * Given a positive integer n, find the smallest integer which has exactly the same digits existing in the integer n and is greater in value than n. If no such positive integer exists, return -1.
 * Note that the returned integer should fit in 32-bit integer, if there is a valid answer but it does not fit in 32-bit integer, return -1.
 *  
 * Example 1:
 * Input: n = 12
 * Output: 21
 * Example 2:
 * Input: n = 21
 * Output: -1
 *  
 * Constraints:
 *
 * 	1 <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/next-greater-element-iii/
// discuss: https://leetcode.com/problems/next-greater-element-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits: Vec<u8> = n.to_string().bytes().collect();
        if Self::next_permutation(&mut digits) {
            digits
                .into_iter()
                .map(|x| x as char)
                .collect::<String>()
                .parse()
                .unwrap_or(-1)
        } else {
            -1
        }
    }

    fn next_permutation(sequence: &mut [u8]) -> bool {
        if sequence.len() < 2 {
            return false;
        }

        let mut idx = sequence.len() - 2;
        while sequence[idx] >= sequence[idx + 1] {
            if idx == 0 {
                return false;
            }
            idx -= 1;
        }

        let mut greater_idx = sequence.len() - 1;
        while sequence[greater_idx] <= sequence[idx] {
            greater_idx -= 1;
        }
        sequence.swap(idx, greater_idx);
        sequence[idx + 1..].reverse();

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0556_example_1() {
        let n = 12;
        let result = 21;

        assert_eq!(Solution::next_greater_element(n), result);
    }

    #[test]
    fn test_0556_example_2() {
        let n = 21;
        let result = -1;

        assert_eq!(Solution::next_greater_element(n), result);
    }
}
