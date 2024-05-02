/**
 * [1584] Min Cost to Connect All Points
 *
 * You are given an array points representing integer coordinates of some points on a 2D-plane, where points[i] = [xi, yi].
 * The cost of connecting two points [xi, yi] and [xj, yj] is the manhattan distance between them: |xi - xj| + |yi - yj|, where |val| denotes the absolute value of val.
 * Return the minimum cost to make all points connected. All points are connected if there is exactly one simple path between any two points.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/26/d.png" style="width: 214px; height: 268px;" />
 * Input: points = [[0,0],[2,2],[3,10],[5,2],[7,0]]
 * Output: 20
 * Explanation:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/26/c.png" style="width: 214px; height: 268px;" />
 * We can connect the points as shown above to get the minimum cost of 20.
 * Notice that there is a unique path between every pair of points.
 *
 * Example 2:
 *
 * Input: points = [[3,12],[-2,5],[-4,1]]
 * Output: 18
 *
 *  
 * Constraints:
 *
 * 	1 <= points.length <= 1000
 * 	-10^6 <= xi, yi <= 10^6
 * 	All pairs (xi, yi) are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/min-cost-to-connect-all-points/
// discuss: https://leetcode.com/problems/min-cost-to-connect-all-points/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() < 2 {
            return 0;
        }

        let mut edges = Vec::new();
        for i in points.iter() {
            for j in points.iter() {
                if i == j {
                    continue;
                }
                let d = (i[0] - j[0]).abs() + (i[1] - j[1]).abs();
                edges.push((i, j, d));
            }
        }
        edges.sort_by_key(|q| q.2);

        let mut cost = 0;
        let mut visited = std::collections::HashSet::new();

        loop {
            for (v1, v2, c) in edges.iter().cycle() {
                let in1 = visited.contains(v1);
                let in2 = visited.contains(v2);
                if in1 && in2 {
                    continue;
                }
                if visited.len() > 0 && !(in1 || in2) {
                    continue;
                }
                visited.insert(v1);
                visited.insert(v2);
                cost += c;
                break;
            }
            if visited.len() >= points.len() {
                break;
            }
        }

        cost
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1584_example_1() {
        let points = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];

        let result = 20;

        assert_eq!(Solution::min_cost_connect_points(points), result);
    }

    #[test]
    fn test_1584_example_2() {
        let points = vec![vec![3, 12], vec![-2, 5], vec![-4, 1]];

        let result = 18;

        assert_eq!(Solution::min_cost_connect_points(points), result);
    }
}
