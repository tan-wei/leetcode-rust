/**
 * [0765] Couples Holding Hands
 *
 * There are n couples sitting in 2n seats arranged in a row and want to hold hands.
 * The people and seats are represented by an integer array row where row[i] is the ID of the person sitting in the i^th seat. The couples are numbered in order, the first couple being (0, 1), the second couple being (2, 3), and so on with the last couple being (2n - 2, 2n - 1).
 * Return the minimum number of swaps so that every couple is sitting side by side. A swap consists of choosing any two people, then they stand up and switch seats.
 *  
 * Example 1:
 *
 * Input: row = [0,2,1,3]
 * Output: 1
 * Explanation: We only need to swap the second (row[1]) and third (row[2]) person.
 *
 * Example 2:
 *
 * Input: row = [3,2,0,1]
 * Output: 0
 * Explanation: All couples are already seated side by side.
 *
 *  
 * Constraints:
 *
 * 	2n == row.length
 * 	2 <= n <= 30
 * 	n is even.
 * 	0 <= row[i] < 2n
 * 	All the elements of row are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/couples-holding-hands/
// discuss: https://leetcode.com/problems/couples-holding-hands/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        Self::dfs_helper(row.clone())
    }

    fn dfs_helper(row: Vec<i32>) -> i32 {
        let mut row = row;
        let n = row.len();
        if n <= 2 {
            return 0;
        }

        let index_n_1 = row.iter().position(|&x| (x as usize) == n - 1).unwrap();

        let neighbour = if index_n_1 % 2 != 0 {
            index_n_1 - 1
        } else {
            index_n_1 + 1
        };
        let mut result = 0;

        if (row[index_n_1] - row[neighbour]).abs() == 1 {
            // do not need swap
        } else {
            result += 1;
            let to_swap_index = row.iter().position(|&x| (x as usize) == n - 2).unwrap();
            row.swap(neighbour, to_swap_index);
        }

        let left = std::cmp::min(index_n_1, neighbour);
        let right = std::cmp::max(index_n_1, neighbour);
        let mut newrow = row[..left].to_vec();
        newrow.extend_from_slice(&row[right + 1..]);
        result += Self::dfs_helper(newrow.clone());
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0765_example_1() {
        let row = vec![0, 2, 1, 3];
        let result = 1;

        assert_eq!(Solution::min_swaps_couples(row), result);
    }

    #[test]
    fn test_0765_example_2() {
        let row = vec![3, 2, 0, 1];
        let result = 0;

        assert_eq!(Solution::min_swaps_couples(row), result);
    }
}
