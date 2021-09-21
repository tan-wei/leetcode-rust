/**
 * [336] Palindrome Pairs
 *
 * Given a list of unique words, return all the pairs of the distinct indices (i, j) in the given list, so that the concatenation of the two words words[i] + words[j] is a palindrome.
 *  
 * Example 1:
 *
 * Input: words = ["abcd","dcba","lls","s","sssll"]
 * Output: [[0,1],[1,0],[3,2],[2,4]]
 * Explanation: The palindromes are ["dcbaabcd","abcddcba","slls","llssssll"]
 *
 * Example 2:
 *
 * Input: words = ["bat","tab","cat"]
 * Output: [[0,1],[1,0]]
 * Explanation: The palindromes are ["battab","tabbat"]
 *
 * Example 3:
 *
 * Input: words = ["a",""]
 * Output: [[0,1],[1,0]]
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 5000
 * 	0 <= words[i].length <= 300
 * 	words[i] consists of lower-case English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-pairs/
// discuss: https://leetcode.com/problems/palindrome-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/palindrome-pairs/discuss/1269909/Rust-HashMap-solution-(48ms)
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let hm = words
            .iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i))
            .collect::<std::collections::HashMap<_, _>>();
        let mut result = Vec::new();
        for (i, word) in words.iter().enumerate() {
            if let Some(&j) = hm.get(&word.chars().rev().collect::<String>()) {
                if i != j {
                    result.push([i as i32, j as i32].to_vec());
                }
            }
            {
                let w = word.chars().rev().collect::<Vec<_>>();
                for k in 0..w.len() {
                    if (0..=k / 2).all(|l| w[l] == w[k - l]) {
                        if let Some(&j) = hm.get(&w[k + 1..].iter().collect::<String>()) {
                            result.push([i as i32, j as i32].to_vec());
                        }
                    }
                }
            }
            {
                let w = word.chars().collect::<Vec<_>>();
                for k in 0..w.len() {
                    if (0..=k / 2).all(|l| w[l] == w[k - l]) {
                        if let Some(&j) = hm.get(&w[k + 1..].iter().rev().collect::<String>()) {
                            result.push([j as i32, i as i32].to_vec());
                        }
                    }
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0336_exapmle_1() {
        let words = vec_string!["abcd", "dcba", "lls", "s", "sssll"];
        let result = vec![vec![0, 1], vec![1, 0], vec![3, 2], vec![2, 4]];

        assert_eq!(Solution::palindrome_pairs(words), result);
    }

    #[test]
    fn test_0336_exapmle_2() {
        let words = vec_string!["bat", "tab", "cat"];
        let result = vec![vec![0, 1], vec![1, 0]];

        assert_eq!(Solution::palindrome_pairs(words), result);
    }

    #[test]
    fn test_0336_exapmle_3() {
        let words = vec_string!["a", ""];
        let result = vec![vec![0, 1], vec![1, 0]];

        assert_eq!(Solution::palindrome_pairs(words), result);
    }
}
