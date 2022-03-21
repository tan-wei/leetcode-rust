/**
 * [0587] Erect the Fence
 *
 * You are given an array trees where trees[i] = [xi, yi] represents the location of a tree in the garden.
 * You are asked to fence the entire garden using the minimum length of rope as it is expensive. The garden is well fenced only if all the trees are enclosed.
 * Return the coordinates of trees that are exactly located on the fence perimeter.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/erect2-plane.jpg" style="width: 509px; height: 500px;" />
 * Input: points = [[1,1],[2,2],[2,0],[2,4],[3,3],[4,2]]
 * Output: [[1,1],[2,0],[3,3],[2,4],[4,2]]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/erect1-plane.jpg" style="width: 509px; height: 500px;" />
 * Input: points = [[1,2],[2,2],[4,2]]
 * Output: [[4,2],[2,2],[1,2]]
 *
 *  
 * Constraints:
 *
 * 	1 <= points.length <= 3000
 * 	points[i].length == 2
 * 	0 <= xi, yi <= 100
 * 	All the given points are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/erect-the-fence/
// discuss: https://leetcode.com/problems/erect-the-fence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/erect-the-fence/discuss/792325/Rust-translated-8ms-2.2M-100
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn orientation(p: &[i32], q: &[i32], r: &[i32]) -> i32 {
            (q[1] - p[1]) * (r[0] - q[0]) - (q[0] - p[0]) * (r[1] - q[1])
        }

        let mut trees = trees;

        if trees.len() <= 1 {
            return trees;
        }

        trees.sort_by(|p, q| p[0].cmp(&q[0]).then(p[1].cmp(&q[1])));

        let mut hull = Vec::<Vec<i32>>::new();
        let n = trees.len();

        for i in 0..n {
            while hull.len() >= 2
                && orientation(
                    hull.get(hull.len() - 2).unwrap(),
                    hull.get(hull.len() - 1).unwrap(),
                    &trees[i],
                ) > 0
            {
                hull.pop();
            }
            hull.push(trees[i].clone());
        }

        for i in (0..n).rev() {
            while hull.len() >= 2
                && orientation(
                    hull.get(hull.len() - 2).unwrap(),
                    hull.get(hull.len() - 1).unwrap(),
                    &trees[i],
                ) > 0
            {
                hull.pop();
            }
            hull.push(trees[i].clone());
        }

        let mut set = std::collections::HashSet::<Vec<i32>>::new();

        for v in hull {
            set.insert(v);
        }

        let mut result = vec![];

        for v in set {
            result.push(v);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0587_example_1() {
        let trees = vec![
            vec![1, 1],
            vec![2, 2],
            vec![2, 0],
            vec![2, 4],
            vec![3, 3],
            vec![4, 2],
        ];
        let result = vec![vec![1, 1], vec![2, 0], vec![3, 3], vec![2, 4], vec![4, 2]];

        assert_eq_sorted!(Solution::outer_trees(trees), result);
    }

    #[test]
    fn test_0587_example_2() {
        let trees = vec![vec![1, 2], vec![2, 2], vec![4, 2]];
        let result = vec![vec![4, 2], vec![2, 2], vec![1, 2]];

        assert_eq_sorted!(Solution::outer_trees(trees), result);
    }
}
