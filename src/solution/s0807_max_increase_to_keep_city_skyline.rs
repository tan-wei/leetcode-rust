/**
 * [0807] Max Increase to Keep City Skyline
 *
 * There is a city composed of n x n blocks, where each block contains a single building shaped like a vertical square prism. You are given a 0-indexed n x n integer matrix grid where grid[r][c] represents the height of the building located in the block at row r and column c.
 * A city's skyline is the the outer contour formed by all the building when viewing the side of the city from a distance. The skyline from each cardinal direction north, east, south, and west may be different.
 * We are allowed to increase the height of any number of buildings by any amount (the amount can be different per building). The height of a 0-height building can also be increased. However, increasing the height of a building should not affect the city's skyline from any cardinal direction.
 * Return the maximum total sum that the height of the buildings can be increased by without changing the city's skyline from any cardinal direction.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/21/807-ex1.png" style="width: 700px; height: 603px;" />
 * Input: grid = [[3,0,8,4],[2,4,5,7],[9,2,6,3],[0,3,1,0]]
 * Output: 35
 * Explanation: The building heights are shown in the center of the above image.
 * The skylines when viewed from each cardinal direction are drawn in red.
 * The grid after increasing the height of buildings without affecting skylines is:
 * gridNew = [ [8, 4, 8, 7],
 *             [7, 4, 7, 7],
 *             [9, 4, 8, 7],
 *             [3, 3, 3, 3] ]
 *
 * Example 2:
 *
 * Input: grid = [[0,0,0],[0,0,0],[0,0,0]]
 * Output: 0
 * Explanation: Increasing the height of any building will result in the skyline changing.
 *
 *  
 * Constraints:
 *
 * 	n == grid.length
 * 	n == grid[r].length
 * 	2 <= n <= 50
 * 	0 <= grid[r][c] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-increase-to-keep-city-skyline/
// discuss: https://leetcode.com/problems/max-increase-to-keep-city-skyline/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/max-increase-to-keep-city-skyline/discuss/2408899/Rust-cheapest-and-best
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let v = (0..grid.len())
            .flat_map(|i| std::iter::repeat(i).zip(0..grid.len()))
            .fold(vec![(0, 0); grid.len()], |mut v, (i, j)| {
                v[i].0 = v[i].0.max(grid[i][j]);
                v[j].1 = v[j].1.max(grid[i][j]);
                v
            });

        grid.into_iter()
            .enumerate()
            .flat_map(|(i, row)| std::iter::repeat(i).zip(row.into_iter().enumerate()))
            .map(|(i, (j, x))| v[i].0.min(v[j].1) - x)
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0807_example_1() {
        let grid = vec![
            vec![3, 0, 8, 4],
            vec![2, 4, 5, 7],
            vec![9, 2, 6, 3],
            vec![0, 3, 1, 0],
        ];

        let result = 35;

        assert_eq!(Solution::max_increase_keeping_skyline(grid), result);
    }

    #[test]
    fn test_0807_example_2() {
        let grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];

        let result = 0;

        assert_eq!(Solution::max_increase_keeping_skyline(grid), result);
    }
}
