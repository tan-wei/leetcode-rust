/**
 * [212] Word Search II
 *
 * Given an m x n board of characters and a list of strings words, return all words on the board.
 * Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/07/search1.jpg" style="width: 322px; height: 322px;" />
 * Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]
 * Output: ["eat","oath"]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/07/search2.jpg" style="width: 162px; height: 162px;" />
 * Input: board = [["a","b"],["c","d"]], words = ["abcb"]
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	m == board.length
 * 	n == board[i].length
 * 	1 <= m, n <= 12
 * 	board[i][j] is a lowercase English letter.
 * 	1 <= words.length <= 3 * 10^4
 * 	1 <= words[i].length <= 10
 * 	words[i] consists of lowercase English letters.
 * 	All the strings of words are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-search-ii/
// discuss: https://leetcode.com/problems/word-search-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    word: Option<String>,
}

impl Solution {
    // Credit: https://leetcode.com/problems/word-search-ii/discuss/712977/Rust-Trie-DFS-solution
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie: Trie = Default::default();
        for word in words.iter() {
            let mut node = &mut trie;
            for c in word.as_bytes() {
                node =
                    node.children[(c - b'a') as usize].get_or_insert(Box::new(Default::default()));
            }
            node.word = Some(word.clone());
        }
        let mut result = std::collections::HashSet::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let mut visited: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
                Self::dfs_helper(&board, (i, j), &trie, &mut visited, &mut result);
            }
        }
        result.into_iter().collect()
    }
    fn dfs_helper(
        board: &[Vec<char>],
        pos: (usize, usize),
        trie: &Trie,
        visited: &mut Vec<Vec<bool>>,
        answer: &mut std::collections::HashSet<String>,
    ) {
        if visited[pos.0][pos.1] {
            return;
        }
        visited[pos.0][pos.1] = true;
        let c = board[pos.0][pos.1];
        if let Some(node) = &trie.children[(c as u8 - b'a') as usize] {
            if let Some(word) = &node.word {
                answer.insert(word.clone());
            }
            if pos.0 > 0 {
                Self::dfs_helper(board, (pos.0 - 1, pos.1), node.as_ref(), visited, answer);
            }
            if pos.1 > 0 {
                Self::dfs_helper(board, (pos.0, pos.1 - 1), node.as_ref(), visited, answer);
            }
            if pos.0 < board.len() - 1 {
                Self::dfs_helper(board, (pos.0 + 1, pos.1), node.as_ref(), visited, answer);
            }
            if pos.1 < board[0].len() - 1 {
                Self::dfs_helper(board, (pos.0, pos.1 + 1), node.as_ref(), visited, answer);
            }
        }
        visited[pos.0][pos.1] = false;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0212_example_1() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = vec_string!["oath", "pea", "eat", "rain"];
        let result = vec_string!["eat", "oath"];

        assert_eq_sorted!(Solution::find_words(board, words), result);
    }

    #[test]
    fn test_0212_example_2() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let words = vec_string!["abcb"];
        let result: Vec<String> = vec_string![];

        assert_eq_sorted!(Solution::find_words(board, words), result);
    }
}
