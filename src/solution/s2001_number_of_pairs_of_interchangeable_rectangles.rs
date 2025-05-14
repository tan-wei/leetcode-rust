/**
 * [2001] Number of Pairs of Interchangeable Rectangles
 *
 * You are given n rectangles represented by a 0-indexed 2D integer array rectangles, where rectangles[i] = [widthi, heighti] denotes the width and height of the i^th rectangle.
 * Two rectangles i and j (i < j) are considered interchangeable if they have the same width-to-height ratio. More formally, two rectangles are interchangeable if widthi/heighti == widthj/heightj (using decimal division, not integer division).
 * Return the number of pairs of interchangeable rectangles in rectangles.
 *  
 * Example 1:
 *
 * Input: rectangles = [[4,8],[3,6],[10,20],[15,30]]
 * Output: 6
 * Explanation: The following are the interchangeable pairs of rectangles by index (0-indexed):
 * - Rectangle 0 with rectangle 1: 4/8 == 3/6.
 * - Rectangle 0 with rectangle 2: 4/8 == 10/20.
 * - Rectangle 0 with rectangle 3: 4/8 == 15/30.
 * - Rectangle 1 with rectangle 2: 3/6 == 10/20.
 * - Rectangle 1 with rectangle 3: 3/6 == 15/30.
 * - Rectangle 2 with rectangle 3: 10/20 == 15/30.
 *
 * Example 2:
 *
 * Input: rectangles = [[4,5],[7,8]]
 * Output: 0
 * Explanation: There are no interchangeable pairs of rectangles.
 *
 *  
 * Constraints:
 *
 * 	n == rectangles.length
 * 	1 <= n <= 10^5
 * 	rectangles[i].length == 2
 * 	1 <= widthi, heighti <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-pairs-of-interchangeable-rectangles/
// discuss: https://leetcode.com/problems/number-of-pairs-of-interchangeable-rectangles/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut ratios = std::collections::HashMap::new();

        rectangles.iter().for_each(|rect| {
            let pos = rect[0] as f64 / rect[1] as f64;

            *ratios.entry(pos.to_ne_bytes()).or_insert(0) += 1
        });

        ratios.values().map(|c| c * (c - 1) / 2).sum::<i64>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2001_example_1() {
        let rectangles = vec![vec![4, 8], vec![3, 6], vec![10, 20], vec![15, 30]];

        let result = 6;

        assert_eq!(Solution::interchangeable_rectangles(rectangles), result);
    }

    #[test]
    fn test_2001_example_2() {
        let rectangles = vec![vec![4, 5], vec![7, 8]];

        let result = 0;

        assert_eq!(Solution::interchangeable_rectangles(rectangles), result);
    }
}
