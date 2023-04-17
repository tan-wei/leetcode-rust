/**
 * [1037] Valid Boomerang
 *
 * Given an array points where points[i] = [xi, yi] represents a point on the X-Y plane, return true if these points are a boomerang.
 * A boomerang is a set of three points that are all distinct and not in a straight line.
 *  
 * Example 1:
 * Input: points = [[1,1],[2,3],[3,2]]
 * Output: true
 * Example 2:
 * Input: points = [[1,1],[2,2],[3,3]]
 * Output: false
 *  
 * Constraints:
 *
 * 	points.length == 3
 * 	points[i].length == 2
 * 	0 <= xi, yi <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-boomerang/
// discuss: https://leetcode.com/problems/valid-boomerang/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        points[0][0] * (points[1][1] - points[2][1])
            + points[1][0] * (points[2][1] - points[0][1])
            + points[2][0] * (points[0][1] - points[1][1])
            != 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1037_example_1() {
        let points = vec![vec![1, 1], vec![2, 3], vec![3, 2]];
        let result = true;

        assert_eq!(Solution::is_boomerang(points), result);
    }

    #[test]
    fn test_1037_example_2() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        let result = false;

        assert_eq!(Solution::is_boomerang(points), result);
    }
}
