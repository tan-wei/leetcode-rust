/**
 * [1632] Rank Transform of a Matrix
 *
 * Given an m x n matrix, return a new matrix answer where answer[row][col] is the rank of matrix[row][col].
 * The rank is an integer that represents how large an element is compared to other elements. It is calculated using the following rules:
 *
 * 	The rank is an integer starting from 1.
 * 	If two elements p and q are in the same row or column, then:
 *
 * 		If p < q then rank(p) < rank(q)
 * 		If p == q then rank(p) == rank(q)
 * 		If p > q then rank(p) > rank(q)
 *
 *
 * 	The rank should be as small as possible.
 *
 * The test cases are generated so that answer is unique under the given rules.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/18/rank1.jpg" style="width: 442px; height: 162px;" />
 * Input: matrix = [[1,2],[3,4]]
 * Output: [[1,2],[2,3]]
 * Explanation:
 * The rank of matrix[0][0] is 1 because it is the smallest integer in its row and column.
 * The rank of matrix[0][1] is 2 because matrix[0][1] > matrix[0][0] and matrix[0][0] is rank 1.
 * The rank of matrix[1][0] is 2 because matrix[1][0] > matrix[0][0] and matrix[0][0] is rank 1.
 * The rank of matrix[1][1] is 3 because matrix[1][1] > matrix[0][1], matrix[1][1] > matrix[1][0], and both matrix[0][1] and matrix[1][0] are rank 2.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/18/rank2.jpg" style="width: 442px; height: 162px;" />
 * Input: matrix = [[7,7],[7,7]]
 * Output: [[1,1],[1,1]]
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/18/rank3.jpg" style="width: 601px; height: 322px;" />
 * Input: matrix = [[20,-21,14],[-19,4,19],[22,-47,24],[-19,4,19]]
 * Output: [[4,2,3],[1,3,4],[5,1,6],[1,3,4]]
 *
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 500
 * 	-10^9 <= matrix[row][col] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rank-transform-of-a-matrix/
// discuss: https://leetcode.com/problems/rank-transform-of-a-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/rank-transform-of-a-matrix/solutions/3183268/unionfind-solution/
struct UnionFind {
    parent: std::collections::HashMap<i32, i32>,
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            parent: std::collections::HashMap::new(),
        }
    }

    fn find(&mut self, u: i32) -> i32 {
        let pu = *self.parent.entry(u).or_insert(u);
        if pu == u {
            return u;
        }
        let v = self.find(pu);
        self.parent.insert(u, v);
        v
    }

    fn union(&mut self, u: i32, v: i32) {
        self.parent.entry(u).or_insert(u);
        self.parent.entry(v).or_insert(v);
        let pu = self.find(u);
        let pv = self.find(v);
        if pu != pv {
            self.parent.insert(pu, pv);
        }
    }

    fn get_groups(&mut self) -> std::collections::HashMap<i32, Vec<i32>> {
        let mut groups = std::collections::HashMap::new();
        let parent = self.parent.clone();
        for (u, _) in parent {
            let pu = self.find(u);
            groups.entry(pu).or_insert(vec![]).push(u);
        }
        groups
    }
}

impl Solution {
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut matrix = matrix;
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut group_by_value = std::collections::BTreeMap::new();
        for (r, item_r) in matrix.iter().enumerate().take(m) {
            for (c, &item_c) in item_r.iter().enumerate().take(n) {
                group_by_value
                    .entry(item_c)
                    .or_insert(vec![])
                    .push((r as i32, c as i32));
            }
        }
        let mut rank = vec![0; m + n];
        for (_, cells) in group_by_value {
            let mut uf = UnionFind::new();
            for &(r, c) in cells.iter() {
                uf.union(r, c + m as i32);
            }
            for (_, group) in uf.get_groups() {
                let mut max_rank = 0;
                for &i in group.iter() {
                    max_rank = max_rank.max(rank[i as usize]);
                }
                for &i in group.iter() {
                    rank[i as usize] = max_rank + 1;
                }
            }
            for &(r, c) in cells.iter() {
                matrix[r as usize][c as usize] = rank[r as usize];
            }
        }

        matrix
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1632_example_1() {
        let matrix = vec![vec![1, 2], vec![3, 4]];

        let result = vec![vec![1, 2], vec![2, 3]];

        assert_eq!(Solution::matrix_rank_transform(matrix), result);
    }

    #[test]
    fn test_1632_example_2() {
        let matrix = vec![vec![7, 7], vec![7, 7]];

        let result = vec![vec![1, 1], vec![1, 1]];

        assert_eq!(Solution::matrix_rank_transform(matrix), result);
    }

    #[test]
    fn test_1632_example_3() {
        let matrix = vec![
            vec![20, -21, 14],
            vec![-19, 4, 19],
            vec![22, -47, 24],
            vec![-19, 4, 19],
        ];

        let result = vec![vec![4, 2, 3], vec![1, 3, 4], vec![5, 1, 6], vec![1, 3, 4]];

        assert_eq!(Solution::matrix_rank_transform(matrix), result);
    }
}
