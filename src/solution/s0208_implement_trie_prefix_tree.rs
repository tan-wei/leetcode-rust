/**
 * [208] Implement Trie (Prefix Tree)
 *
 * A <a href="https://en.wikipedia.org/wiki/Trie" target="_blank">trie</a> (pronounced as "try") or prefix tree is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.
 * Implement the Trie class:
 *
 * 	Trie() Initializes the trie object.
 * 	void insert(String word) Inserts the string word into the trie.
 * 	boolean search(String word) Returns true if the string word is in the trie (i.e., was inserted before), and false otherwise.
 * 	boolean startsWith(String prefix) Returns true if there is a previously inserted string word that has the prefix prefix, and false otherwise.
 *
 *  
 * Example 1:
 *
 * Input
 * ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
 * [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
 * Output
 * [null, null, true, false, true, null, true]
 * Explanation
 * Trie trie = new Trie();
 * trie.insert("apple");
 * trie.search("apple");   // return True
 * trie.search("app");     // return False
 * trie.startsWith("app"); // return True
 * trie.insert("app");
 * trie.search("app");     // return True
 *
 *  
 * Constraints:
 *
 * 	1 <= word.length, prefix.length <= 2000
 * 	word and prefix consist only of lowercase English letters.
 * 	At most 3 * 10^4 calls in total will be made to insert, search, and startsWith.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/implement-trie-prefix-tree/
// discuss: https://leetcode.com/problems/implement-trie-prefix-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Default)]
struct Trie {
    is_word: bool,
    children: [Option<Box<Trie>>; 26],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            match &cur.children[c as usize - 'a' as usize] {
                None => cur.children[c as usize - 'a' as usize] = Some(Box::new(Trie::new())),
                Some(n) => (),
            }
            cur = cur.children[c as usize - 'a' as usize].as_mut().unwrap();
        }
        cur.is_word = true;
    }
    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for c in word.chars() {
            match cur.children[c as usize - 'a' as usize].as_ref() {
                None => return false,
                Some(n) => cur = n.as_ref(),
            }
        }
        return cur.is_word;
    }
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;
        for c in prefix.chars() {
            match cur.children[c as usize - 'a' as usize].as_ref() {
                None => return false,
                Some(n) => cur = n.as_ref(),
            }
        }
        return true;
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0208_example_1() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple".to_string()), true); // return True
        assert_eq!(trie.search("app".to_string()), false); // return False
        assert_eq!(trie.starts_with("app".to_string()), true); // return True
        trie.insert("app".to_string());
        assert_eq!(trie.search("app".to_string()), true); // return True
    }
}
