/**
 * vec![1162] As Far from Land as Possible
 *
 * Given an n x n grid containing only values 0 and 1, where 0 represents water and 1 represents land, find a water cell such that its distance to the nearest land cell is maximized, and return the distance. If no land or water exists in the grid, return -1.
 * The distance used in this problem is the Manhattan distance: the distance between two cells (x0, y0) and (x1, y1) is |x0 - x1| + |y0 - y1|.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/05/03/1336_ex1.JPG" style="width: 185px; height: 87px;" />
 * Input: grid = [[1,0,1],[0,0,0],[1,0,1]]
 * Output: 2
 * Explanation: The cell (1, 1) is as far as possible from all the land with distance 2.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/05/03/1336_ex2.JPG" style="width: 184px; height: 87px;" />
 * Input: grid = [[1,0,0],[0,0,0],[0,0,0]]
 * Output: 4
 * Explanation: The cell (2, 2) is as far as possible from all the land with distance 4.
 *
 *  
 * Constraints:
 *
 * 	n == grid.length
 * 	n == grid[i].length
 * 	1 <= n <= 100
 * 	grid[i][j] is 0 or 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/as-far-from-land-as-possible/
// discuss: https://leetcode.com/problems/as-far-from-land-as-possible/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let n = grid.len();
        let mut front: Vec<(i32, i32)> = Vec::new();
        let mut count = 0;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    count += 1;
                    front.push((i as i32, j as i32));
                }
            }
        }

        let neighbor = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
        for step in 0.. {
            let mut new_front: Vec<(i32, i32)> = Vec::new();
            for pt in front {
                for off in &neighbor {
                    let new_i = pt.0 + off.0;
                    let new_j = pt.1 + off.1;
                    if new_i >= 0
                        && new_i < n as i32
                        && new_j >= 0
                        && new_j < n as i32
                        && grid[new_i as usize][new_j as usize] == 0
                    {
                        new_front.push((new_i, new_j));
                        grid[new_i as usize][new_j as usize] = 1;
                    }
                }
            }
            if new_front.is_empty() {
                return if step == 0 { -1 } else { step };
            }
            front = new_front;
        }

        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1162_example_1() {
        let grid = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        let result = 2;

        assert_eq!(Solution::max_distance(grid), result);
    }

    #[test]
    fn test_1162_example_2() {
        let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        let result = 4;

        assert_eq!(Solution::max_distance(grid), result);
    }
}
