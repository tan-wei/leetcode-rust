/**
 * [0803] Bricks Falling When Hit
 *
 * You are given an m x n binary grid, where each 1 represents a brick and 0 represents an empty space. A brick is stable if:
 *
 * 	It is directly connected to the top of the grid, or
 * 	At least one other brick in its four adjacent cells is stable.
 *
 * You are also given an array hits, which is a sequence of erasures we want to apply. Each time we want to erase the brick at the location hits[i] = (rowi, coli). The brick on that location (if it exists) will disappear. Some other bricks may no longer be stable because of that erasure and will fall. Once a brick falls, it is immediately erased from the grid (i.e., it does not land on other stable bricks).
 * Return an array result, where each result[i] is the number of bricks that will fall after the i^th erasure is applied.
 * Note that an erasure may refer to a location with no brick, and if it does, no bricks drop.
 *  
 * Example 1:
 *
 * Input: grid = [[1,0,0,0],[1,1,1,0]], hits = [[1,0]]
 * Output: [2]
 * Explanation: Starting with the grid:
 * [[1,0,0,0],
 *  [<u>1</u>,1,1,0]]
 * We erase the underlined brick at (1,0), resulting in the grid:
 * [[1,0,0,0],
 *  [0,<u>1</u>,<u>1</u>,0]]
 * The two underlined bricks are no longer stable as they are no longer connected to the top nor adjacent to another stable brick, so they will fall. The resulting grid is:
 * [[1,0,0,0],
 *  [0,0,0,0]]
 * Hence the result is [2].
 *
 * Example 2:
 *
 * Input: grid = [[1,0,0,0],[1,1,0,0]], hits = [[1,1],[1,0]]
 * Output: [0,0]
 * Explanation: Starting with the grid:
 * [[1,0,0,0],
 *  [1,<u>1</u>,0,0]]
 * We erase the underlined brick at (1,1), resulting in the grid:
 * [[1,0,0,0],
 *  [1,0,0,0]]
 * All remaining bricks are still stable, so no bricks fall. The grid remains the same:
 * [[1,0,0,0],
 *  [<u>1</u>,0,0,0]]
 * Next, we erase the underlined brick at (1,0), resulting in the grid:
 * [[1,0,0,0],
 *  [0,0,0,0]]
 * Once again, all remaining bricks are still stable, so no bricks fall.
 * Hence the result is [0,0].
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 200
 * 	grid[i][j] is 0 or 1.
 * 	1 <= hits.length <= 4 * 10^4
 * 	hits[i].length == 2
 * 	0 <= xi <= m - 1
 * 	0 <= yi <= n - 1
 * 	All (xi, yi) are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/bricks-falling-when-hit/
// discuss: https://leetcode.com/problems/bricks-falling-when-hit/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/bricks-falling-when-hit/discuss/2252748/Rust-Union-Find
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let len_ns: usize = len_rs * len_cs;
        let len_hs: usize = hits.len();
        let mut grid: Vec<Vec<i32>> = {
            let mut grid = grid;
            for hit in &hits {
                let r: usize = hit[0] as usize;
                let c: usize = hit[1] as usize;
                if grid[r][c] == 1 {
                    grid[r][c] = 2;
                }
            }
            grid
        };
        let mut roots: Vec<usize> = (0..len_ns + 1).collect();
        let mut sizes: Vec<i32> = vec![1; len_ns + 1];
        for r in 0..len_rs {
            for c in 0..len_cs {
                if grid[r][c] == 1 {
                    Self::union_around((r, c), &grid, &mut roots, &mut sizes);
                }
            }
        }
        let mut bricks_left = sizes[Self::find(0, &mut roots)];
        let mut bricks_dropped: Vec<i32> = vec![0; len_hs];
        for (idx, hit) in hits.iter().enumerate().rev() {
            let r: usize = hit[0] as usize;
            let c: usize = hit[1] as usize;
            if grid[r][c] == 2 {
                grid[r][c] = 1;
                Self::union_around((r, c), &grid, &mut roots, &mut sizes);
                let cur_bricks_left = sizes[Self::find(0, &mut roots)];
                bricks_dropped[idx] = if cur_bricks_left == bricks_left {
                    0
                } else {
                    cur_bricks_left - bricks_left - 1
                };
                bricks_left = cur_bricks_left;
            }
        }
        bricks_dropped
    }

    fn union_around(
        coord: (usize, usize),
        grid: &Vec<Vec<i32>>,
        roots: &mut Vec<usize>,
        sizes: &mut Vec<i32>,
    ) {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let (r, c) = coord;
        for d in 0..4 {
            let r_nxt: isize = r as isize + Self::DIRS[d];
            let c_nxt: isize = c as isize + Self::DIRS[d + 1];
            if r_nxt < 0
                || c_nxt < 0
                || r_nxt as usize >= len_rs
                || c_nxt as usize >= len_cs
                || grid[r_nxt as usize][c_nxt as usize] != 1
            {
                continue;
            }
            Self::union(
                Self::hash(r, c, len_cs),
                Self::hash(r_nxt as usize, c_nxt as usize, len_cs),
                roots,
                sizes,
            );
        }
        if r == 0 {
            Self::union(0, Self::hash(r, c, len_cs), roots, sizes);
        }
    }

    fn union(x: usize, y: usize, roots: &mut Vec<usize>, sizes: &mut Vec<i32>) {
        let root_x: usize = Self::find(x, roots);
        let root_y: usize = Self::find(y, roots);
        if root_x == root_y {
            return;
        }
        if sizes[root_x] > sizes[root_y] {
            roots[root_y] = root_x;
            sizes[root_x] += sizes[root_y];
            sizes[root_y] = 0;
        } else {
            roots[root_x] = root_y;
            sizes[root_y] += sizes[root_x];
            sizes[root_x] = 0;
        }
    }

    fn find(mut x: usize, roots: &mut Vec<usize>) -> usize {
        while x != roots[x] {
            roots[x] = roots[roots[x]];
            x = roots[x];
        }
        x
    }

    fn hash(r: usize, c: usize, len_cs: usize) -> usize {
        r * len_cs + c + 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0803_example_1() {
        let grid = vec![vec![1, 0, 0, 0], vec![1, 1, 1, 0]];
        let hits = vec![vec![1, 0]];
        let result = vec![2];

        assert_eq!(Solution::hit_bricks(grid, hits), result);
    }

    #[test]
    fn test_0803_example_2() {
        let grid = vec![vec![1, 0, 0, 0], vec![1, 1, 0, 0]];
        let hits = vec![vec![1, 1], vec![1, 0]];
        let result = vec![0, 0];

        assert_eq!(Solution::hit_bricks(grid, hits), result);
    }
}
