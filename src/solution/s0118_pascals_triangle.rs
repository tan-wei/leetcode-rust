/**
 * [118] Pascal's Triangle
 *
 * Given an integer numRows, return the first numRows of Pascal's triangle.
 * In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" style="height:240px; width:260px" />
 *  
 * Example 1:
 * Input: numRows = 5
 * Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
 * Example 2:
 * Input: numRows = 1
 * Output: [[1]]
 *  
 * Constraints:
 *
 * 	1 <= numRows <= 30
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pascals-triangle/
// discuss: https://leetcode.com/problems/pascals-triangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        match num_rows {
            1 => vec![vec![1]],
            n if n > 1 => {
                let mut row_n = vec![1; n as usize];
                let mut result = Self::generate(n - 1);
                let row_n_1 = result.iter().last().unwrap();

                for i in 1usize..(n - 1) as usize {
                    row_n[i] = row_n_1[i - 1] + row_n_1[i];
                }

                result.push(row_n);
                result
            }
            _ => panic!("Wrong input!"),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0118_example_1() {
        let num_rows = 5;
        let result = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];

        assert_eq!(Solution::generate(num_rows), result);
    }

    #[test]
    fn test_0118_example_2() {
        let num_rows = 1;
        let result = vec![vec![1]];

        assert_eq!(Solution::generate(num_rows), result);
    }
}
