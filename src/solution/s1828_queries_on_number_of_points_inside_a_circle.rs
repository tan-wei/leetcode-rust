/**
 * [1828] Queries on Number of Points Inside a Circle
 *
 * You are given an array points where points[i] = [xi, yi] is the coordinates of the i^th point on a 2D plane. Multiple points can have the same coordinates.
 * You are also given an array queries where queries[j] = [xj, yj, rj] describes a circle centered at (xj, yj) with a radius of rj.
 * For each query queries[j], compute the number of points inside the j^th circle. Points on the border of the circle are considered inside.
 * Return an array answer, where answer[j] is the answer to the j^th query.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/25/chrome_2021-03-25_22-34-16.png" style="width: 500px; height: 418px;" />
 * Input: points = [[1,3],[3,3],[5,3],[2,2]], queries = [[2,3,1],[4,3,1],[1,1,2]]
 * Output: [3,2,2]
 * Explanation: The points and circles are shown above.
 * queries[0] is the green circle, queries[1] is the red circle, and queries[2] is the blue circle.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/25/chrome_2021-03-25_22-42-07.png" style="width: 500px; height: 390px;" />
 * Input: points = [[1,1],[2,2],[3,3],[4,4],[5,5]], queries = [[1,2,2],[2,2,2],[4,3,2],[4,3,3]]
 * Output: [2,3,2,4]
 * Explanation: The points and circles are shown above.
 * queries[0] is green, queries[1] is red, queries[2] is blue, and queries[3] is purple.
 *
 *  
 * Constraints:
 *
 * 	1 <= points.length <= 500
 * 	points[i].length == 2
 * 	0 <= x​​​​​​i, y​​​​​​i <= 500
 * 	1 <= queries.length <= 500
 * 	queries[j].length == 3
 * 	0 <= xj, yj <= 500
 * 	1 <= rj <= 500
 * 	All coordinates are integers.
 *
 *  
 * Follow up: Could you find the answer for each query in better complexity than O(n)?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/
// discuss: https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries
            .iter()
            .map(|q| {
                points
                    .iter()
                    .filter(|p| q[2].pow(2) >= (p[0] - q[0]).pow(2) + (p[1] - q[1]).pow(2))
                    .count() as _
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1828_example_1() {
        let points = vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]];
        let queries = vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]];

        let result = vec![3, 2, 2];

        assert_eq!(Solution::count_points(points, queries), result);
    }

    #[test]
    fn test_1828_example_2() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        let queries = vec![vec![1, 2, 2], vec![2, 2, 2], vec![4, 3, 2], vec![4, 3, 3]];

        let result = vec![2, 3, 2, 4];

        assert_eq!(Solution::count_points(points, queries), result);
    }
}
