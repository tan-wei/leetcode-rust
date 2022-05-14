/**
 * [0676] Implement Magic Dictionary
 *
 * Design a data structure that is initialized with a list of different words. Provided a string, you should determine if you can change exactly one character in this string to match any word in the data structure.
 * Implement the MagicDictionary class:
 *
 * 	MagicDictionary() Initializes the object.
 * 	void buildDict(String[] dictionary) Sets the data structure with an array of distinct strings dictionary.
 * 	bool search(String searchWord) Returns true if you can change exactly one character in searchWord to match any string in the data structure, otherwise returns false.
 *
 *  
 * Example 1:
 *
 * Input
 * ["MagicDictionary", "buildDict", "search", "search", "search", "search"]
 * [[], [["hello", "leetcode"]], ["hello"], ["hhllo"], ["hell"], ["leetcoded"]]
 * Output
 * [null, null, false, true, false, false]
 * Explanation
 * MagicDictionary magicDictionary = new MagicDictionary();
 * magicDictionary.buildDict(["hello", "leetcode"]);
 * magicDictionary.search("hello"); // return False
 * magicDictionary.search("hhllo"); // We can change the second 'h' to 'e' to match "hello" so we return True
 * magicDictionary.search("hell"); // return False
 * magicDictionary.search("leetcoded"); // return False
 *
 *  
 * Constraints:
 *
 * 	1 <= dictionary.length <= 100
 * 	1 <= dictionary[i].length <= 100
 * 	dictionary[i] consists of only lower-case English letters.
 * 	All the strings in dictionary are distinct.
 * 	1 <= searchWord.length <= 100
 * 	searchWord consists of only lower-case English letters.
 * 	buildDict will be called only once before search.
 * 	At most 100 calls will be made to search.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/implement-magic-dictionary/
// discuss: https://leetcode.com/problems/implement-magic-dictionary/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Debug, Clone)]
struct TrieNode {
    is_word: bool,
    children: Vec<Option<TrieNode>>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_word: false,
            children: vec![None; 26],
        }
    }
}

struct MagicDictionary {
    trie: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    fn new() -> Self {
        Self {
            trie: TrieNode::new(),
        }
    }
    fn build_dict(&mut self, dictionary: Vec<String>) {
        for s in &dictionary {
            let mut cur = &mut self.trie;
            for ch in s.as_bytes() {
                if cur.children[(ch - b'a') as usize].is_none() {
                    cur.children[(ch - b'a') as usize] = Some(TrieNode::new());
                }
                cur = cur.children[(ch - b'a') as usize].as_mut().unwrap();
            }
            cur.is_word = true
        }
    }

    fn search(&self, mut search_word: String) -> bool {
        fn helper(v: &[u8], trie: &TrieNode) -> bool {
            let mut cur = trie;
            for ch in v {
                if cur.children[(ch - b'a') as usize].is_none() {
                    return false;
                }
                cur = cur.children[(ch - b'a') as usize].as_ref().unwrap();
            }
            cur.is_word
        }
        let mut v = unsafe { search_word.as_bytes_mut() };
        for i in 0..v.len() {
            for c in b'a'..=b'z' {
                if v[i] == c {
                    continue;
                }
                let orig = v[i];
                v[i] = c;
                if helper(v, &self.trie) {
                    return true;
                }
                v[i] = orig;
            }
        }
        false
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0676_example_1() {
        let mut dict = MagicDictionary::new();
        dict.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
        assert!(!dict.search("hello".to_string()));
        assert!(dict.search("hhllo".to_string()));
        assert!(!dict.search("hell".to_string()));
        assert!(!dict.search("leetcode".to_string()));
    }
}
