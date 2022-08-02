/**
 * [0778] Swim in Rising Water
 *
 * You are given an n x n integer matrix grid where each value grid[i][j] represents the elevation at that point (i, j).
 * The rain starts to fall. At time t, the depth of the water everywhere is t. You can swim from a square to another 4-directionally adjacent square if and only if the elevation of both squares individually are at most t. You can swim infinite distances in zero time. Of course, you must stay within the boundaries of the grid during your swim.
 * Return the least time until you can reach the bottom right square (n - 1, n - 1) if you start at the top left square (0, 0).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/swim1-grid.jpg" style="width: 164px; height: 165px;" />
 * Input: grid = [[0,2],[1,3]]
 * Output: 3
 * Explanation:
 * At time 0, you are in grid location (0, 0).
 * You cannot go anywhere else because 4-directionally adjacent neighbors have a higher elevation than t = 0.
 * You cannot reach point (1, 1) until time 3.
 * When the depth of water is 3, we can swim anywhere inside the grid.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/swim2-grid-1.jpg" style="width: 404px; height: 405px;" />
 * Input: grid = [[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]]
 * Output: 16
 * Explanation: The final route is shown.
 * We need to wait until time 16 so that (0, 0) and (4, 4) are connected.
 *
 *  
 * Constraints:
 *
 * 	n == grid.length
 * 	n == grid[i].length
 * 	1 <= n <= 50
 * 	0 <= grid[i][j] < n^2
 * 	Each value grid[i][j] is unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/swim-in-rising-water/
// discuss: https://leetcode.com/problems/swim-in-rising-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let reachable = |t: i32| {
            let mut visited = vec![vec![false; n]; n];
            let mut vd = std::collections::VecDeque::new();
            if grid[0][0] <= t {
                visited[0][0] = true;
                vd.push_back((0, 0));
            }
            while let Some((i, j)) = vd.pop_front() {
                if i == n - 1 && j == n - 1 {
                    return true;
                }
                for d in [0, 1, 0, !0, 0].windows(2) {
                    let i = i.wrapping_add(d[0]);
                    let j = j.wrapping_add(d[1]);
                    if (0..n).contains(&i)
                        && (0..n).contains(&j)
                        && !visited[i][j]
                        && grid[i][j] <= t
                    {
                        visited[i][j] = true;
                        vd.push_back((i, j));
                    }
                }
            }
            false
        };

        let (mut low, mut high) = (0, (n * n) as i32);
        while low < high {
            let m = (low + high) / 2;
            if reachable(m) {
                high = m;
            } else {
                low = m + 1;
            }
        }

        low
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0778_example_1() {
        let grid = vec![vec![0, 2], vec![1, 3]];
        let result = 3;

        assert_eq!(Solution::swim_in_water(grid), result);
    }

    #[test]
    fn test_0778_example_2() {
        let grid = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6],
        ];
        let result = 16;

        assert_eq!(Solution::swim_in_water(grid), result);
    }
}
