/**
 * [1001] Grid Illumination
 *
 * There is a 2D grid of size n x n where each cell of this grid has a lamp that is initially turned off.
 * You are given a 2D array of lamp positions lamps, where lamps[i] = [rowi, coli] indicates that the lamp at grid[rowi][coli] is turned on. Even if the same lamp is listed more than once, it is turned on.
 * When a lamp is turned on, it illuminates its cell and all other cells in the same row, column, or diagonal.
 * You are also given another 2D array queries, where queries[j] = [rowj, colj]. For the j^th query, determine whether grid[rowj][colj] is illuminated or not. After answering the j^th query, turn off the lamp at grid[rowj][colj] and its 8 adjacent lamps if they exist. A lamp is adjacent if its cell shares either a side or corner with grid[rowj][colj].
 * Return an array of integers ans, where ans[j] should be 1 if the cell in the j^th query was illuminated, or 0 if the lamp was not.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/illu_1.jpg" style="width: 750px; height: 209px;" />
 * Input: n = 5, lamps = [[0,0],[4,4]], queries = [[1,1],[1,0]]
 * Output: [1,0]
 * Explanation: We have the initial grid with all lamps turned off. In the above picture we see the grid after turning on the lamp at grid[0][0] then turning on the lamp at grid[4][4].
 * The 0^th query asks if the lamp at grid[1][1] is illuminated or not (the blue square). It is illuminated, so set ans[0] = 1. Then, we turn off all lamps in the red square.
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/illu_step1.jpg" style="width: 500px; height: 218px;" />
 * The 1^st query asks if the lamp at grid[1][0] is illuminated or not (the blue square). It is not illuminated, so set ans[1] = 0. Then, we turn off all lamps in the red rectangle.
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/illu_step2.jpg" style="width: 500px; height: 219px;" />
 *
 * Example 2:
 *
 * Input: n = 5, lamps = [[0,0],[4,4]], queries = [[1,1],[1,1]]
 * Output: [1,1]
 *
 * Example 3:
 *
 * Input: n = 5, lamps = [[0,0],[0,4]], queries = [[0,4],[0,1],[1,4]]
 * Output: [1,1,0]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^9
 * 	0 <= lamps.length <= 20000
 * 	0 <= queries.length <= 20000
 * 	lamps[i].length == 2
 * 	0 <= rowi, coli < n
 * 	queries[j].length == 2
 * 	0 <= rowj, colj < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/grid-illumination/
// discuss: https://leetcode.com/problems/grid-illumination/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct LampGrid {
    horizontal: std::collections::HashMap<i64, i32>,
    vertical: std::collections::HashMap<i64, i32>,
    rdiagonals: std::collections::HashMap<i64, i32>,
    ldiagonals: std::collections::HashMap<i64, i32>,
    lamps: std::collections::HashSet<(usize, usize)>,
    n: usize,
}

impl LampGrid {
    pub fn new(n: usize, lamps: Vec<Vec<i32>>) -> Self {
        // we keep track of which lines (horizontal, vertical, ldiagonal, rdiagonal) contain a lamp
        let mut horizontal = std::collections::HashMap::new();
        let mut vertical = std::collections::HashMap::new();
        let mut rdiagonals = std::collections::HashMap::new();
        let mut ldiagonals = std::collections::HashMap::new();

        for lamp in lamps.iter() {
            let (x, y) = (lamp[0] as i64, lamp[1] as i64);

            *horizontal.entry(y).or_insert(0) += 1;
            *vertical.entry(x).or_insert(0) += 1;
            *ldiagonals.entry(y - x).or_insert(0) += 1;
            *rdiagonals.entry(x + y).or_insert(0) += 1;
        }

        LampGrid {
            horizontal,
            vertical,
            ldiagonals,
            rdiagonals,
            lamps: lamps
                .into_iter()
                .map(|v| (v[0] as usize, v[1] as usize))
                .collect(),
            n,
        }
    }

    fn is_illuminated(&self, x: i64, y: i64) -> bool {
        // does this position lie on a line containing a lamp?
        (self.horizontal.contains_key(&y) && self.horizontal[&y] > 0)
            || (self.vertical.contains_key(&x) && self.vertical[&x] > 0)
            || (self.rdiagonals.contains_key(&(x + y)) && self.rdiagonals[&(x + y)] > 0)
            || (self.ldiagonals.contains_key(&(y - x)) && self.ldiagonals[&(y - x)] > 0)
    }

    fn dim(&mut self, x: i64, y: i64) {
        if self.lamps.remove(&(x as usize, y as usize)) {
            // decrease the lamp count on all of the lines this lamp is on
            *(self.horizontal.get_mut(&y).unwrap()) -= 1;
            *(self.vertical.get_mut(&x).unwrap()) -= 1;
            *(self.rdiagonals.get_mut(&(x + y)).unwrap()) -= 1;
            *(self.ldiagonals.get_mut(&(y - x)).unwrap()) -= 1;
        }
    }

    pub fn query(&mut self, x: i64, y: i64) -> i32 {
        let answer = if self.is_illuminated(x, y) { 1 } else { 0 };

        // dim all lights that are at or adjacent 8-directionally to (x,y)
        for xdelta in -1..=1 {
            for ydelta in -1..=1 {
                self.dim(x + xdelta, y + ydelta);
            }
        }

        answer
    }
}

impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut lg = LampGrid::new(n as usize, lamps);

        queries
            .into_iter()
            .map(|v| lg.query(v[0] as i64, v[1] as i64))
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1001_example_1() {
        let n = 5;
        let lamps = vec![vec![0, 0], vec![4, 4]];
        let queries = vec![vec![1, 1], vec![1, 0]];
        let result = vec![1, 0];

        assert_eq!(Solution::grid_illumination(n, lamps, queries), result);
    }

    #[test]
    fn test_1001_example_2() {
        let n = 5;
        let lamps = vec![vec![0, 0], vec![4, 4]];
        let queries = vec![vec![1, 1], vec![1, 1]];
        let result = vec![1, 1];

        assert_eq!(Solution::grid_illumination(n, lamps, queries), result);
    }

    #[test]
    fn test_1001_example_3() {
        let n = 5;
        let lamps = vec![vec![0, 0], vec![0, 4]];
        let queries = vec![vec![0, 4], vec![0, 1], vec![1, 4]];
        let result = vec![1, 1, 0];

        assert_eq!(Solution::grid_illumination(n, lamps, queries), result);
    }

    #[test]
    #[ignore]
    fn test_1001_additional_1() {
        let n = 6;
        let lamps = vec![
            vec![2, 5],
            vec![4, 2],
            vec![0, 3],
            vec![0, 5],
            vec![1, 4],
            vec![4, 2],
            vec![3, 3],
            vec![1, 0],
        ];

        let queries = vec![
            vec![4, 3],
            vec![3, 1],
            vec![5, 3],
            vec![0, 5],
            vec![4, 4],
            vec![3, 3],
        ];
        let result = vec![1, 0, 1, 1, 0, 1];

        assert_eq!(Solution::grid_illumination(n, lamps, queries), result);
    }
}
