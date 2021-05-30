/**
 * [149] Max Points on a Line
 *
 * Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane, return the maximum number of points that lie on the same straight line.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/25/plane1.jpg" style="width: 300px; height: 294px;" />
 * Input: points = [[1,1],[2,2],[3,3]]
 * Output: 3
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/25/plane2.jpg" style="width: 300px; height: 294px;" />
 * Input: points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= points.length <= 300
 * 	points[i].length == 2
 * 	-10^4 <= xi, yi <= 10^4
 * 	All the points are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-points-on-a-line/
// discuss: https://leetcode.com/problems/max-points-on-a-line/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/max-points-on-a-line/discuss/866915/Rust-Time-O(n2)-Space-O(N)-4ms-2.2mb
    // Time O(n^2) Space O(N)
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        use std::cmp::max;
        use std::collections::HashMap;
        use std::i32::MAX;

        fn max_points_on_a_line_containing_point_i(
            points: &Vec<Vec<i32>>,
            i: usize,
            n: usize,
        ) -> i32 {
            fn slope_coprime(x1: i32, y1: i32, x2: i32, y2: i32) -> (i32, i32) {
                let (mut delta_x, mut delta_y) = (x1 - x2, y1 - y2);
                if delta_x == 0 {
                    return (0, 0);
                } else if delta_y == 0 {
                    return (MAX, MAX);
                } else if delta_x < 0 {
                    delta_x = -delta_x;
                    delta_y = -delta_y;
                }
                let gcd = Solution::gcd(delta_x, delta_y);
                (delta_x / gcd, delta_y / gcd)
            }

            fn add_line(
                lines_map: &mut HashMap<(i32, i32), i32>,
                points: &Vec<Vec<i32>>,
                i: usize,
                j: usize,
                mut count: i32,
                mut duplicates: i32,
                horizontal_lines: &mut i32,
            ) -> (i32, i32) {
                let (x1, y1, x2, y2) = (points[i][0], points[i][1], points[j][0], points[j][1]);

                if x1 == x2 && y1 == y2 {
                    duplicates += 1;
                } else if y1 == y2 {
                    *horizontal_lines += 1;
                    count = max(*horizontal_lines, count);
                } else {
                    let slope = slope_coprime(x1, y1, x2, y2);
                    let t = *lines_map.entry(slope).or_insert(1);
                    lines_map.insert(slope, t + 1);
                    count = max(*lines_map.get(&slope).unwrap(), count);
                }

                (count, duplicates)
            }
            //  init lines passing through point i
            let mut horizontal_lines = 1;

            let mut map = HashMap::new();
            // One starts with just one point on a line : point i.
            let mut counts = (1, 0);
            // Compute lines passing through point i (fixed)
            // and point j (interation).
            // Update in a loop the number of points on a line
            //  and the number of duplicates of point i.
            for j in (i + 1)..n {
                counts = add_line(
                    &mut map,
                    points,
                    i,
                    j,
                    counts.0,
                    counts.1,
                    &mut horizontal_lines,
                );
            }
            counts.0 + counts.1
        }

        let n = points.len();
        if n < 3 {
            return n as i32;
        }
        let mut max_count = 1;
        // Compute in a loop a max number of points
        // on a line containing point i.
        for i in 0..(n - 1) {
            max_count = max(
                max_points_on_a_line_containing_point_i(&points, i, n),
                max_count,
            )
        }
        max_count
    }

    fn gcd(u: i32, v: i32) -> i32 {
        if v == 0 {
            u
        } else {
            Self::gcd(v, u % v)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0149_example_1() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        let result = 3;

        assert_eq!(Solution::max_points(points), result);
    }

    #[test]
    fn test_0149_example_2() {
        let points = vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4],
        ];
        let result = 4;

        assert_eq!(Solution::max_points(points), result);
    }
}
