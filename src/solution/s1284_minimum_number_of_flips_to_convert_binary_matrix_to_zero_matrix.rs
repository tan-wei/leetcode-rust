/**
 * [1284] Minimum Number of Flips to Convert Binary Matrix to Zero Matrix
 *
 * Given a m x n binary matrix mat. In one step, you can choose one cell and flip it and all the four neighbors of it if they exist (Flip is changing 1 to 0 and 0 to 1). A pair of cells are called neighbors if they share one edge.
 * Return the minimum number of steps required to convert mat to a zero matrix or -1 if you cannot.
 * A binary matrix is a matrix with all cells equal to 0 or 1 only.
 * A zero matrix is a matrix with all cells equal to 0.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/28/matrix.png" style="width: 409px; height: 86px;" />
 * Input: mat = [[0,0],[0,1]]
 * Output: 3
 * Explanation: One possible solution is to flip (1, 0) then (0, 1) and finally (1, 1) as shown.
 *
 * Example 2:
 *
 * Input: mat = [[0]]
 * Output: 0
 * Explanation: Given matrix is a zero matrix. We do not need to change it.
 *
 * Example 3:
 *
 * Input: mat = [[1,0,0],[1,0,0]]
 * Output: -1
 * Explanation: Given matrix cannot be a zero matrix.
 *
 *
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 3
 * 	mat[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix/
// discuss: https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix/solutions/876567/rust-translated-0ms-100/
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        const NEIGHBORS: [i32; 6] = [0, 0, 1, 0, -1, 0];
        let mut start = 0;

        let m = mat.len();
        let n = mat[0].len();
        for i in 0..m {
            for j in 0..n {
                start |= mat[i][j] << (i * n + j);
            }
        }
        let mut q = std::collections::VecDeque::<i32>::new();
        let mut visited = std::collections::HashSet::<i32>::new();
        q.push_back(start);
        visited.insert(start);
        let mut step = 0;
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let cur = q.pop_front().unwrap();
                if cur == 0 {
                    return step;
                }
                for i in 0..m {
                    for j in 0..n {
                        let mut next = cur;
                        for k in 0..5 {
                            let r = i as i32 + NEIGHBORS[k];
                            let c = j as i32 + NEIGHBORS[k + 1];
                            if r >= 0 && r < m as i32 && c >= 0 && c < n as i32 {
                                next ^= 1 << (r * n as i32 + c);
                            }
                        }
                        if visited.insert(next) {
                            q.push_back(next);
                        }
                    }
                }
            }
            step += 1;
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1284_example_1() {
        let mat = vec![vec![0, 0], vec![0, 1]];
        let result = 3;

        assert_eq!(Solution::min_flips(mat), result);
    }

    #[test]
    fn test_1284_example_2() {
        let mat = vec![vec![0]];
        let result = 0;

        assert_eq!(Solution::min_flips(mat), result);
    }

    #[test]
    fn test_1284_example_3() {
        let mat = vec![vec![1, 0, 0], vec![1, 0, 0]];
        let result = -1;

        assert_eq!(Solution::min_flips(mat), result);
    }
}
