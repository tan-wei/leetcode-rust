/**
 * [0554] Brick Wall
 *
 * There is a rectangular brick wall in front of you with n rows of bricks. The i^th row has some number of bricks each of the same height (i.e., one unit) but they can be of different widths. The total width of each row is the same.
 * Draw a vertical line from the top to the bottom and cross the least bricks. If your line goes through the edge of a brick, then the brick is not considered as crossed. You cannot draw a line just along one of the two vertical edges of the wall, in which case the line will obviously cross no bricks.
 * Given the 2D array wall that contains the information about the wall, return the minimum number of crossed bricks after drawing such a vertical line.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/cutwall-grid.jpg" style="width: 493px; height: 577px;" />
 * Input: wall = [[1,2,2,1],[3,1,2],[1,3,2],[2,4],[3,1,2],[1,3,1,1]]
 * Output: 2
 *
 * Example 2:
 *
 * Input: wall = [[1],[1],[1]]
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	n == wall.length
 * 	1 <= n <= 10^4
 * 	1 <= wall[i].length <= 10^4
 * 	1 <= sum(wall[i].length) <= 2 * 10^4
 * 	sum(wall[i]) is the same for each row i.
 * 	1 <= wall[i][j] <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/brick-wall/
// discuss: https://leetcode.com/problems/brick-wall/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        (wall.len()
            - wall
                .into_iter()
                .fold(
                    (std::collections::HashMap::new(), 0, 0),
                    |(mut map, mut x, mut sum), row| {
                        for i in 0..row.len() - 1 {
                            sum += row[i];
                            (*map.entry(sum).or_insert(0)) += 1;
                            x = x.max(*map.get(&sum).unwrap());
                        }
                        (map, x, 0)
                    },
                )
                .1) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0554_example_1() {
        let wall = vec![
            vec![1, 2, 2, 1],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 4],
            vec![3, 1, 2],
            vec![1, 3, 1, 1],
        ];
        let result = 2;

        assert_eq!(Solution::least_bricks(wall), result);
    }

    #[test]
    fn test_0554_example_2() {
        let wall = vec![vec![1], vec![1], vec![1]];
        let result = 3;

        assert_eq!(Solution::least_bricks(wall), result);
    }
}
