/**
 * [1128] Number of Equivalent Domino Pairs
 *
 * Given a list of dominoes, dominoes[i] = [a, b] is equivalent to dominoes[j] = [c, d] if and only if either (a == c and b == d), or (a == d and b == c) - that is, one domino can be rotated to be equal to another domino.
 * Return the number of pairs (i, j) for which 0 <= i < j < dominoes.length, and dominoes[i] is equivalent to dominoes[j].
 *  
 * Example 1:
 *
 * Input: dominoes = [[1,2],[2,1],[3,4],[5,6]]
 * Output: 1
 *
 * Example 2:
 *
 * Input: dominoes = [[1,2],[1,2],[1,1],[1,2],[2,2]]
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	1 <= dominoes.length <= 4 * 10^4
 * 	dominoes[i].length == 2
 * 	1 <= dominoes[i][j] <= 9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-equivalent-domino-pairs/
// discuss: https://leetcode.com/problems/number-of-equivalent-domino-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut count = std::collections::HashMap::new();
        for domino in dominoes {
            let d = (domino[0].min(domino[1]), domino[0].max(domino[1]));
            *count.entry(d).or_insert(0) += 1;
        }
        count.values().map(|c| (c * (c - 1)) / 2).sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1128_example_1() {
        let dominoes = vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]];
        let result = 1;

        assert_eq!(Solution::num_equiv_domino_pairs(dominoes), result);
    }

    #[test]
    fn test_1128_example_2() {
        let dominoes = vec![vec![1, 2], vec![1, 2], vec![1, 1], vec![1, 2], vec![2, 2]];
        let result = 3;

        assert_eq!(Solution::num_equiv_domino_pairs(dominoes), result);
    }
}
