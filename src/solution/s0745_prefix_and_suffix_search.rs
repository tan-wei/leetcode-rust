/**
 * [0745] Prefix and Suffix Search
 *
 * Design a special dictionary that searches the words in it by a prefix and a suffix.
 * Implement the WordFilter class:
 *
 * 	WordFilter(string[] words) Initializes the object with the words in the dictionary.
 * 	f(string pref, string suff) Returns the index of the word in the dictionary, which has the prefix pref and the suffix suff. If there is more than one valid index, return the largest of them. If there is no such word in the dictionary, return -1.
 *
 *  
 * Example 1:
 *
 * Input
 * ["WordFilter", "f"]
 * [[["apple"]], ["a", "e"]]
 * Output
 * [null, 0]
 * Explanation
 * WordFilter wordFilter = new WordFilter(["apple"]);
 * wordFilter.f("a", "e"); // return 0, because the word at index 0 has prefix = "a" and suffix = "e".
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 10^4
 * 	1 <= words[i].length <= 7
 * 	1 <= pref.length, suff.length <= 7
 * 	words[i], pref and suff consist of lowercase English letters only.
 * 	At most 10^4 calls will be made to the function f.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/prefix-and-suffix-search/
// discuss: https://leetcode.com/problems/prefix-and-suffix-search/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/prefix-and-suffix-search/discuss/1185417/Rust-Trie-solution

#[derive(Default)]
struct Trie {
    index: i32,
    childlen: [Option<Box<Trie>>; 27],
}

struct WordFilter {
    trie: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::default();
        for (i, word) in words.iter().enumerate() {
            let s = String::new() + &word + "{" + &word;
            for j in 0..word.len() {
                let mut node = &mut trie;
                for &b in &s.as_bytes()[j..] {
                    node = node.childlen[(b - b'a') as usize].get_or_insert_with(Default::default);
                    node.index = i as i32;
                }
            }
        }
        Self { trie }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        let mut node = &self.trie;
        let s = String::new() + &suff + "{" + &pref;
        for &b in s.as_bytes() {
            if let Some(n) = &node.childlen[(b - b'a') as usize] {
                node = n.as_ref();
            } else {
                return -1;
            }
        }
        node.index
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(pref, suff);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0745_example_1() {
        let mut word_filter = WordFilter::new(vec_string!["apple"]);
        assert_eq!(word_filter.f("a".to_string(), "e".to_string()), 0); // return 0, because the word at index 0 has prefix = "a" and suffix = "e".
    }
}
