/**
 * [0407] Trapping Rain Water II
 *
 * Given an m x n integer matrix heightMap representing the height of each unit cell in a 2D elevation map, return the volume of water it can trap after raining.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/trap1-3d.jpg" style="width: 361px; height: 321px;" />
 * Input: heightMap = [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]
 * Output: 4
 * Explanation: After the rain, water is trapped between the blocks.
 * We have two small pounds 1 and 3 units trapped.
 * The total volume of water trapped is 4.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/trap2-3d.jpg" style="width: 401px; height: 321px;" />
 * Input: heightMap = [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]
 * Output: 10
 *
 *  
 * Constraints:
 *
 * 	m == heightMap.length
 * 	n == heightMap[i].length
 * 	1 <= m, n <= 200
 * 	0 <= heightMap[i][j] <= 2 * 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/trapping-rain-water-ii/
// discuss: https://leetcode.com/problems/trapping-rain-water-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/trapping-rain-water-ii/discuss/834255/Rust-translated-4ms-100
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        if m == 0 {
            return 0;
        }
        let n = height_map[0].len();
        let mut heap = std::collections::BinaryHeap::new();
        let mut visited = vec![vec![false; n]; m];
        let mut result = 0;
        for c in 0..n {
            heap.push((std::cmp::Reverse(height_map[0][c]), 0, c as i32));
            heap.push((
                std::cmp::Reverse(height_map[m - 1][c]),
                m as i32 - 1,
                c as i32,
            ));
            visited[0][c] = true;
            visited[m - 1][c] = true;
        }
        for r in 1..m {
            heap.push((std::cmp::Reverse(height_map[r][0]), r as i32, 0));
            heap.push((
                std::cmp::Reverse(height_map[r][n - 1]),
                r as i32,
                n as i32 - 1,
            ));
            visited[r][0] = true;
            visited[r][n - 1] = true;
        }
        const DIRS: [i32; 5] = [0, -1, 0, 1, 0];
        while !heap.is_empty() {
            let min = heap.pop().unwrap();
            for i in 0..4 {
                let r = min.1 + DIRS[i];
                let c = min.2 + DIRS[i + 1];
                if r > 0
                    && c > 0
                    && r < m as i32
                    && c < n as i32
                    && !visited[r as usize][c as usize]
                {
                    heap.push((
                        std::cmp::Reverse(std::cmp::max(
                            (min.0).0,
                            height_map[r as usize][c as usize],
                        )),
                        r,
                        c,
                    ));
                    visited[r as usize][c as usize] = true;
                    result += std::cmp::max(0, (min.0).0 - height_map[r as usize][c as usize]);
                }
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
    fn test_0407_example_1() {
        let height_map = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ];
        let result = 4;

        assert_eq!(Solution::trap_rain_water(height_map), result);
    }

    #[test]
    fn test_0407_example_2() {
        let height_map = vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 1, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 3, 3, 3],
        ];
        let result = 10;

        assert_eq!(Solution::trap_rain_water(height_map), result);
    }
}
