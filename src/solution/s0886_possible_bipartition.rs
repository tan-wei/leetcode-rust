/**
 * [0886] Possible Bipartition
 *
 * We want to split a group of n people (labeled from 1 to n) into two groups of any size. Each person may dislike some other people, and they should not go into the same group.
 * Given the integer n and the array dislikes where dislikes[i] = [ai, bi] indicates that the person labeled ai does not like the person labeled bi, return true if it is possible to split everyone into two groups in this way.
 *  
 * Example 1:
 *
 * Input: n = 4, dislikes = [[1,2],[1,3],[2,4]]
 * Output: true
 * Explanation: group1 [1,4] and group2 [2,3].
 *
 * Example 2:
 *
 * Input: n = 3, dislikes = [[1,2],[1,3],[2,3]]
 * Output: false
 *
 * Example 3:
 *
 * Input: n = 5, dislikes = [[1,2],[2,3],[3,4],[4,5],[1,5]]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 2000
 * 	0 <= dislikes.length <= 10^4
 * 	dislikes[i].length == 2
 * 	1 <= dislikes[i][j] <= n
 * 	ai < bi
 * 	All the pairs of dislikes are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/possible-bipartition/
// discuss: https://leetcode.com/problems/possible-bipartition/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/possible-bipartition/solutions/654966/rust-bfs-solution/
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut dmap = vec![vec![]; n as usize];
        for d in dislikes.iter() {
            dmap[d[0] as usize - 1].push(d[1] as usize - 1);
            dmap[d[1] as usize - 1].push(d[0] as usize - 1);
        }
        let mut v: Vec<Option<bool>> = vec![None; n as usize];
        for i in 0..n as usize {
            if v[i].is_some() {
                continue;
            }
            let mut vd = std::collections::VecDeque::new();
            vd.push_back((i, true));
            while let Some(last) = vd.pop_front() {
                if let Some(b) = v[last.0] {
                    if b != last.1 {
                        return false;
                    }
                    continue;
                }
                v[last.0] = Some(last.1);
                for j in dmap[last.0].iter() {
                    if v[*j].is_none() {
                        vd.push_back((*j, !last.1));
                    }
                }
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0886_example_1() {
        let n = 4;
        let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 4]];
        let result = true;

        assert_eq!(Solution::possible_bipartition(n, dislikes), result);
    }

    #[test]
    fn test_0886_example_2() {
        let n = 3;
        let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        let result = false;

        assert_eq!(Solution::possible_bipartition(n, dislikes), result);
    }

    #[test]
    fn test_0886_example_3() {
        let n = 5;
        let dislikes = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]];
        let result = false;

        assert_eq!(Solution::possible_bipartition(n, dislikes), result);
    }
}
