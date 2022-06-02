/**
 * [0699] Falling Squares
 *
 * There are several squares being dropped onto the X-axis of a 2D plane.
 * You are given a 2D integer array positions where positions[i] = [lefti, sideLengthi] represents the i^th square with a side length of sideLengthi that is dropped with its left edge aligned with X-coordinate lefti.
 * Each square is dropped one at a time from a height above any landed squares. It then falls downward (negative Y direction) until it either lands on the top side of another square or on the X-axis. A square brushing the left/right side of another square does not count as landing on it. Once it lands, it freezes in place and cannot be moved.
 * After each square is dropped, you must record the height of the current tallest stack of squares.
 * Return an integer array ans where ans[i] represents the height described above after dropping the i^th square.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/28/fallingsq1-plane.jpg" style="width: 500px; height: 505px;" />
 * Input: positions = [[1,2],[2,3],[6,1]]
 * Output: [2,5,5]
 * Explanation:
 * After the first drop, the tallest stack is square 1 with a height of 2.
 * After the second drop, the tallest stack is squares 1 and 2 with a height of 5.
 * After the third drop, the tallest stack is still squares 1 and 2 with a height of 5.
 * Thus, we return an answer of [2, 5, 5].
 *
 * Example 2:
 *
 * Input: positions = [[100,100],[200,100]]
 * Output: [100,100]
 * Explanation:
 * After the first drop, the tallest stack is square 1 with a height of 100.
 * After the second drop, the tallest stack is either square 1 or square 2, both with heights of 100.
 * Thus, we return an answer of [100, 100].
 * Note that square 2 only brushes the right side of square 1, which does not count as landing on it.
 *
 *  
 * Constraints:
 *
 * 	1 <= positions.length <= 1000
 * 	1 <= lefti <= 10^8
 * 	1 <= sideLengthi <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/falling-squares/
// discuss: https://leetcode.com/problems/falling-squares/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/falling-squares/discuss/845821/Rust-translated-8ms-100
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = std::collections::BTreeMap::<i32, i32>::new();
        map.insert(0, 0);
        let mut max = 0;
        let mut result = vec![];
        for pos in &positions {
            let start = pos[0];
            let end = pos[0] + pos[1];
            let mut h = 0;
            let mut k = 0;
            if let Some((&key, &height)) = map.range(..=start).last() {
                h = height;
                k = key;
            }
            let mut iter = map.range(k..).skip(1);
            while let Some((&key, &height)) = iter.next() {
                if key >= end {
                    break;
                }
                h = std::cmp::max(h, height);
            }
            h += pos[1];
            max = std::cmp::max(max, h);
            result.push(max);

            if let Some((&tail, &height)) = map.range(..end).last() {
                map.insert(start, h);
                map.insert(end, height);
            }
            let mut iter = map.range(start..).skip(1);
            let mut list = vec![];
            while let Some((&key, &height)) = iter.next() {
                if key < end {
                    list.push(key)
                } else {
                    break;
                }
            }
            for key in list {
                map.remove(&key);
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0699_example_1() {
        let positions = vec![vec![1, 2], vec![2, 3], vec![6, 1]];
        let result = vec![2, 5, 5];

        assert_eq!(Solution::falling_squares(positions), result);
    }

    #[test]
    fn test_0699_example_2() {
        let positions = vec![vec![100, 100], vec![200, 100]];
        let result = vec![100, 100];

        assert_eq!(Solution::falling_squares(positions), result);
    }
}
