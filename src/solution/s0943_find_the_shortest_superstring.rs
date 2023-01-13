/**
 * [0943] Find the Shortest Superstring
 *
 * Given an array of strings words, return the smallest string that contains each string in words as a substring. If there are multiple valid strings of the smallest length, return any of them.
 * You may assume that no string in words is a substring of another string in words.
 *  
 * Example 1:
 *
 * Input: words = ["alex","loves","leetcode"]
 * Output: "alexlovesleetcode"
 * Explanation: All permutations of "alex","loves","leetcode" would also be accepted.
 *
 * Example 2:
 *
 * Input: words = ["catg","ctaagt","gcta","ttca","atgcatc"]
 * Output: "gctaagttcatgcatc"
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 12
 * 	1 <= words[i].length <= 20
 * 	words[i] consists of lowercase English letters.
 * 	All the strings of words are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-shortest-superstring/
// discuss: https://leetcode.com/problems/find-the-shortest-superstring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/find-the-shortest-superstring/solutions/1226857/rust-translated-dp-solution/
    pub fn shortest_superstring(words: Vec<String>) -> String {
        let n = words.len();
        let mut graph = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let mut k = words[j].len();
                    while !words[i].ends_with(&words[j][..k]) {
                        k -= 1;
                    }
                    graph[i][j] = k;
                }
            }
        }
        let mut dp = vec![vec![0; n]; 1 << n];
        let mut parent = vec![vec![None; n]; 1 << n];
        for mask in 0..1 << n {
            for b in 0..n {
                if mask >> b & 1 == 0 {
                    continue;
                }
                let prev = mask ^ 1 << b;
                for (i, row) in graph.iter().enumerate() {
                    if prev >> i & 1 == 0 {
                        continue;
                    }
                    let val = dp[prev][i] + row[b];
                    if val > dp[mask][b] {
                        dp[mask][b] = val;
                        parent[mask][b] = Some(i);
                    }
                }
            }
        }
        let mut mask = (1 << n) - 1;
        let mut perm = Vec::with_capacity(n);
        let mut seen = vec![false; n];
        if let Some(mut p) = (0..n).max_by_key(|&i| dp[(1 << n) - 1][i]) {
            loop {
                perm.push(p);
                seen[p] = true;
                if let Some(t) = parent[mask][p] {
                    mask ^= 1 << p;
                    p = t;
                } else {
                    break;
                }
            }
        }
        perm.extend((0..n).filter(|&i| !seen[i]));
        (1..perm.len())
            .rev()
            .fold(words[perm[n - 1]].clone(), |acc, i| {
                let overlap = graph[perm[i]][perm[i - 1]];
                acc + &words[perm[i - 1]][overlap..]
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0943_example_1() {
        let words = vec_string!["alex", "loves", "leetcode"];
        let result = "alexlovesleetcode".to_string();

        assert_eq!(Solution::shortest_superstring(words), result);
    }

    #[test]
    #[ignore]
    fn test_0943_example_2() {
        let words = vec_string!["catg", "ctaagt", "gcta", "ttca", "atgcatc"];
        let result = "gctaagttcatgcatc".to_string();

        assert_eq!(Solution::shortest_superstring(words), result);
    }
}
