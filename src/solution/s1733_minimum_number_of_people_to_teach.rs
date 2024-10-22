/**
 * [1733] Minimum Number of People to Teach
 *
 * On a social network consisting of m users and some friendships between users, two users can communicate with each other if they know a common language.
 * You are given an integer n, an array languages, and an array friendships where:
 *
 * 	There are n languages numbered 1 through n,
 * 	languages[i] is the set of languages the i^​​​​​​th​​​​ user knows, and
 * 	friendships[i] = [u​​​​​​i​​​, v​​​​​​i] denotes a friendship between the users u^​​​​​​​​​​​i​​​​​ and vi.
 *
 * You can choose one language and teach it to some users so that all friends can communicate with each other. Return <i data-stringify-type="italic">the minimum <i data-stringify-type="italic">number of users you need to teach.
 * Note that friendships are not transitive, meaning if x is a friend of y and y is a friend of z, this doesn't guarantee that x is a friend of z.
 *  
 * Example 1:
 *
 * Input: n = 2, languages = [[1],[2],[1,2]], friendships = [[1,2],[1,3],[2,3]]
 * Output: 1
 * Explanation: You can either teach user 1 the second language or user 2 the first language.
 *
 * Example 2:
 *
 * Input: n = 3, languages = [[2],[1,3],[1,2],[3]], friendships = [[1,4],[1,2],[3,4],[2,3]]
 * Output: 2
 * Explanation: Teach the third language to users 1 and 3, yielding two users to teach.
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 500
 * 	languages.length == m
 * 	1 <= m <= 500
 * 	1 <= languages[i].length <= n
 * 	1 <= languages[i][j] <= n
 * 	1 <= u​​​​​​i < v​​​​​​i <= languages.length
 * 	1 <= friendships.length <= 500
 * 	All tuples (u​​​​​i, v​​​​​​i) are unique
 * 	languages[i] contains only unique values
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-people-to-teach/
// discuss: https://leetcode.com/problems/minimum-number-of-people-to-teach/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let mut languages = languages;
        let mut memo = vec![vec![0; languages.len() + 1]; languages.len() + 1];
        let mut result = languages.len() as i32;

        for language in languages.iter_mut() {
            language.sort();
        }

        for lang in 1..=n {
            let mut count = 0;
            let mut teach = vec![false; languages.len()];
            for friendship in friendships.iter() {
                let u = friendship[0] as usize - 1;
                let v = friendship[1] as usize - 1;

                if Self::check(&languages, u, v, &mut memo) {
                    continue;
                }

                if languages[u].binary_search(&lang).is_err() {
                    if !teach[u] {
                        count += 1;
                    }
                    teach[u] = true;
                }

                if languages[v].binary_search(&lang).is_err() {
                    if !teach[v] {
                        count += 1;
                    }
                    teach[v] = true;
                }
            }

            result = result.min(count);
        }

        result
    }

    fn check(languages: &[Vec<i32>], u: usize, v: usize, memo: &mut [Vec<i32>]) -> bool {
        if memo[u][v] != 0 {
            return memo[u][v] == 1;
        }

        for i in 0..languages[v].len() {
            if languages[u].binary_search(&languages[v][i]).is_ok() {
                memo[v][u] = 1;
                memo[u][v] = 1;
                return true;
            }
        }

        memo[u][v] = -1;
        memo[v][u] = -1;

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1733_example_1() {
        let n = 2;
        let languages = vec![vec![1], vec![2], vec![1, 2]];
        let friendships = vec![vec![1, 2], vec![1, 3], vec![2, 3]];

        let result = 1;

        assert_eq!(
            Solution::minimum_teachings(n, languages, friendships),
            result
        );
    }

    #[test]
    fn test_1733_example_2() {
        let n = 3;
        let languages = vec![vec![2], vec![1, 3], vec![1, 2], vec![3]];
        let friendships = vec![vec![1, 4], vec![1, 2], vec![3, 4], vec![2, 3]];

        let result = 2;

        assert_eq!(
            Solution::minimum_teachings(n, languages, friendships),
            result
        );
    }
}
