/**
 * [0461] Hamming Distance
 *
 * The <a href="https://en.wikipedia.org/wiki/Hamming_distance" target="_blank">Hamming distance</a> between two integers is the number of positions at which the corresponding bits are different.
 * Given two integers x and y, return the Hamming distance between them.
 *  
 * Example 1:
 *
 * Input: x = 1, y = 4
 * Output: 2
 * Explanation:
 * 1   (0 0 0 1)
 * 4   (0 1 0 0)
 *        &uarr;   &uarr;
 * The above arrows point to positions where the corresponding bits are different.
 *
 * Example 2:
 *
 * Input: x = 3, y = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	0 <= x, y <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/hamming-distance/
// discuss: https://leetcode.com/problems/hamming-distance/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        format!("{:b}", x ^ y).matches('1').count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0461_example_1() {
        let x = 1;
        let y = 4;

        let result = 2;

        assert_eq!(Solution::hamming_distance(x, y), result);
    }

    #[test]
    fn test_0461_example_2() {
        let x = 3;
        let y = 1;

        let result = 1;

        assert_eq!(Solution::hamming_distance(x, y), result);
    }
}
