/**
 * [1061] Lexicographically Smallest Equivalent String
 *
 * You are given two strings of the same length s1 and s2 and a string baseStr.
 * We say s1[i] and s2[i] are equivalent characters.
 *
 * 	For example, if s1 = "abc" and s2 = "cde", then we have 'a' == 'c', 'b' == 'd', and 'c' == 'e'.
 *
 * Equivalent characters follow the usual rules of any equivalence relation:
 *
 * 	Reflexivity: 'a' == 'a'.
 * 	Symmetry: 'a' == 'b' implies 'b' == 'a'.
 * 	Transitivity: 'a' == 'b' and 'b' == 'c' implies 'a' == 'c'.
 *
 * For example, given the equivalency information from s1 = "abc" and s2 = "cde", "acd" and "aab" are equivalent strings of baseStr = "eed", and "aab" is the lexicographically smallest equivalent string of baseStr.
 * Return the lexicographically smallest equivalent string of baseStr by using the equivalency information from s1 and s2.
 *  
 * Example 1:
 *
 * Input: s1 = "parker", s2 = "morris", baseStr = "parser"
 * Output: "makkek"
 * Explanation: Based on the equivalency information in s1 and s2, we can group their characters as [m,p], [a,o], [k,r,s], [e,i].
 * The characters in each group are equivalent and sorted in lexicographical order.
 * So the answer is "makkek".
 *
 * Example 2:
 *
 * Input: s1 = "hello", s2 = "world", baseStr = "hold"
 * Output: "hdld"
 * Explanation: Based on the equivalency information in s1 and s2, we can group their characters as [h,w], [d,e,o], [l,r].
 * So only the second letter 'o' in baseStr is changed to 'd', the answer is "hdld".
 *
 * Example 3:
 *
 * Input: s1 = "leetcode", s2 = "programs", baseStr = "sourcecode"
 * Output: "aauaaaaada"
 * Explanation: We group the equivalent characters in s1 and s2 as [a,o,e,r,s,c], [l,p], [g,t] and [d,m], thus all letters in baseStr except 'u' and 'd' are transformed to 'a', the answer is "aauaaaaada".
 *
 *  
 * Constraints:
 *
 * 	1 <= s1.length, s2.length, baseStr <= 1000
 * 	s1.length == s2.length
 * 	s1, s2, and baseStr consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/lexicographically-smallest-equivalent-string/
// discuss: https://leetcode.com/problems/lexicographically-smallest-equivalent-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/lexicographically-smallest-equivalent-string/solutions/3048320/rust-union-find-and-graph-solutions-beat-100-simple-copy-paste/
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let g = s1.bytes().zip(s2.bytes()).fold(
            std::collections::HashMap::with_capacity(26),
            |mut g, (mut c1, mut c2)| {
                if c1 != c2 {
                    c1 -= 0x61;
                    c2 -= 0x61;
                    g.entry(c1).or_insert(vec![]).push(c2);
                    g.entry(c2).or_insert(vec![]).push(c1);
                }
                g
            },
        );
        let mut minc = vec![0; 26];

        (0u8..26).for_each(|i| {
            minc[i as usize] = i;
            let mut visited = vec![false; 26];
            Self::dfs_helper(i, i, &mut minc[i as usize], &g, &mut visited);
        });
        base_str
            .chars()
            .map(|c| (0x61 + minc[c as usize - 0x61]) as char)
            .collect()
    }

    fn dfs_helper(
        curr: u8,
        parent: u8,
        min: &mut u8,
        graph: &std::collections::HashMap<u8, Vec<u8>>,
        v: &mut Vec<bool>,
    ) {
        if v[curr as usize] {
            return;
        }
        v[curr as usize] = true;
        *min = u8::min(*min, curr);
        // println!("{}:{}:{}", curr, parent, min);
        graph.get(&curr).map(|children: &Vec<u8>| {
            children
                .iter()
                .filter(|&&neigh| neigh != parent)
                .for_each(|&child| {
                    Self::dfs_helper(child, curr, min, graph, v);
                })
        });
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1061_example_1() {
        let s1 = "parker".to_string();
        let s2 = "morris".to_string();
        let base_str = "parser".to_string();
        let result = "makkek".to_string();

        assert_eq!(
            Solution::smallest_equivalent_string(s1, s2, base_str),
            result
        );
    }

    #[test]
    fn test_1061_example_2() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let base_str = "hold".to_string();
        let result = "hdld".to_string();

        assert_eq!(
            Solution::smallest_equivalent_string(s1, s2, base_str),
            result
        );
    }

    #[test]
    fn test_1061_example_3() {
        let s1 = "leetcode".to_string();
        let s2 = "programs".to_string();
        let base_str = "sourcecode".to_string();
        let result = "aauaaaaada".to_string();

        assert_eq!(
            Solution::smallest_equivalent_string(s1, s2, base_str),
            result
        );
    }
}
