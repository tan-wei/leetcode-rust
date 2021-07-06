/**
 * [211] Design Add and Search Words Data Structure
 *
 * Design a data structure that supports adding new words and finding if a string matches any previously added string.
 * Implement the WordDictionary class:
 *
 * 	WordDictionary() Initializes the object.
 * 	void addWord(word) Adds word to the data structure, it can be matched later.
 * 	bool search(word) Returns true if there is any string in the data structure that matches word or false otherwise. word may contain dots '.' where dots can be matched with any letter.
 *
 *  
 * Example:
 *
 * Input
 * ["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
 * [[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
 * Output
 * [null,null,null,null,false,true,true,true]
 * Explanation
 * WordDictionary wordDictionary = new WordDictionary();
 * wordDictionary.addWord("bad");
 * wordDictionary.addWord("dad");
 * wordDictionary.addWord("mad");
 * wordDictionary.search("pad"); // return False
 * wordDictionary.search("bad"); // return True
 * wordDictionary.search(".ad"); // return True
 * wordDictionary.search("b.."); // return True
 *
 *  
 * Constraints:
 *
 * 	1 <= word.length <= 500
 * 	word in addWord consists lower-case English letters.
 * 	word in search consist of  '.' or lower-case English letters.
 * 	At most 50000 calls will be made to addWord and search.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-add-and-search-words-data-structure/
// discuss: https://leetcode.com/problems/design-add-and-search-words-data-structure/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    end: bool,
}

#[derive(Default)]
struct WordDictionary {
    trie: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    fn add_word(&mut self, word: String) {
        let mut node = &mut self.trie;
        for &c in word.as_bytes() {
            node = node.children[(c - b'a') as usize].get_or_insert(Box::new(Default::default()));
        }
        node.end = true;
    }

    fn search(&self, word: String) -> bool {
        WordDictionary::search_trie(&self.trie, &word.as_bytes())
    }

    fn search_trie(trie: &Trie, word: &[u8]) -> bool {
        if let Some(&c) = word.first() {
            if c == b'.' {
                for child in &trie.children {
                    if let Some(node) = child {
                        if WordDictionary::search_trie(&node, &word[1..]) {
                            return true;
                        }
                    }
                }
            } else if let Some(node) = &trie.children[(c - b'a') as usize] {
                return WordDictionary::search_trie(&node, &word[1..]);
            }
            false
        } else {
            trie.end
        }
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0211_example_1() {
        let mut word_dictionary = WordDictionary::new();
        word_dictionary.add_word("bad".to_string());
        word_dictionary.add_word("dad".to_string());
        word_dictionary.add_word("mad".to_string());
        assert_eq!(word_dictionary.search("pad".to_string()), false); // return False
        assert_eq!(word_dictionary.search("bad".to_string()), true); // return True
        assert_eq!(word_dictionary.search(".ad".to_string()), true); // return True
        assert_eq!(word_dictionary.search("b..".to_string()), true); // return True
    }
}
