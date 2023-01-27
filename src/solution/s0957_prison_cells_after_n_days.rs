/**
 * [0957] Prison Cells After N Days
 *
 * There are 8 prison cells in a row and each cell is either occupied or vacant.
 * Each day, whether the cell is occupied or vacant changes according to the following rules:
 *
 * 	If a cell has two adjacent neighbors that are both occupied or both vacant, then the cell becomes occupied.
 * 	Otherwise, it becomes vacant.
 *
 * Note that because the prison is a row, the first and the last cells in the row can't have two adjacent neighbors.
 * You are given an integer array cells where cells[i] == 1 if the i^th cell is occupied and cells[i] == 0 if the i^th cell is vacant, and you are given an integer n.
 * Return the state of the prison after n days (i.e., n such changes described above).
 *  
 * Example 1:
 *
 * Input: cells = [0,1,0,1,1,0,0,1], n = 7
 * Output: [0,0,1,1,0,0,0,0]
 * Explanation: The following table summarizes the state of the prison on each day:
 * Day 0: [0, 1, 0, 1, 1, 0, 0, 1]
 * Day 1: [0, 1, 1, 0, 0, 0, 0, 0]
 * Day 2: [0, 0, 0, 0, 1, 1, 1, 0]
 * Day 3: [0, 1, 1, 0, 0, 1, 0, 0]
 * Day 4: [0, 0, 0, 0, 0, 1, 0, 0]
 * Day 5: [0, 1, 1, 1, 0, 1, 0, 0]
 * Day 6: [0, 0, 1, 0, 1, 1, 0, 0]
 * Day 7: [0, 0, 1, 1, 0, 0, 0, 0]
 *
 * Example 2:
 *
 * Input: cells = [1,0,0,1,0,0,1,0], n = 1000000000
 * Output: [0,0,1,1,1,1,1,0]
 *
 *  
 * Constraints:
 *
 * 	cells.length == 8
 * 	cells[i] is either 0 or 1.
 * 	1 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/prison-cells-after-n-days/
// discuss: https://leetcode.com/problems/prison-cells-after-n-days/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let mut cells = cells;
        for _ in 0..(n - 1) % 14 + 1 {
            cells = (0..cells.len())
                .map(|i| {
                    if i > 0 && i < cells.len() - 1 && cells[i - 1] == cells[i + 1] {
                        1
                    } else {
                        0
                    }
                })
                .collect();
        }
        cells
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0957_example_1() {
        let cells = vec![0, 1, 0, 1, 1, 0, 0, 1];
        let n = 7;
        let result = vec![0, 0, 1, 1, 0, 0, 0, 0];

        assert_eq!(Solution::prison_after_n_days(cells, n), result);
    }

    #[test]
    fn test_0957_example_2() {
        let cells = vec![1, 0, 0, 1, 0, 0, 1, 0];
        let n = 1000000000;
        let result = vec![0, 0, 1, 1, 1, 1, 1, 0];

        assert_eq!(Solution::prison_after_n_days(cells, n), result);
    }
}
