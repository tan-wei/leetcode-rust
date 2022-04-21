/**
 * [0648] Replace Words
 *
 * In English, we have a concept called root, which can be followed by some other word to form another longer word - let's call this word successor. For example, when the root "an" is followed by the successor word "other", we can form a new word "another".
 * Given a dictionary consisting of many roots and a sentence consisting of words separated by spaces, replace all the successors in the sentence with the root forming it. If a successor can be replaced by more than one root, replace it with the root that has the shortest length.
 * Return the sentence after the replacement.
 *  
 * Example 1:
 *
 * Input: dictionary = ["cat","bat","rat"], sentence = "the cattle was rattled by the battery"
 * Output: "the cat was rat by the bat"
 *
 * Example 2:
 *
 * Input: dictionary = ["a","b","c"], sentence = "aadsfasf absbs bbab cadsfafs"
 * Output: "a a b c"
 *
 *  
 * Constraints:
 *
 * 	1 <= dictionary.length <= 1000
 * 	1 <= dictionary[i].length <= 100
 * 	dictionary[i] consists of only lower-case letters.
 * 	1 <= sentence.length <= 10^6
 * 	sentence consists of only lower-case letters and spaces.
 * 	The number of words in sentence is in the range [1, 1000]
 * 	The length of each word in sentence is in the range [1, 1000]
 * 	Every two consecutive words in sentence will be separated by exactly one space.
 * 	sentence does not have leading or trailing spaces.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/replace-words/
// discuss: https://leetcode.com/problems/replace-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/replace-words/discuss/942708/Rust-4ms

#[derive(Default)]
pub struct Trie {
    pub children: [Option<Box<Trie>>; 26],
    pub is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            let idx = (c as i8 - 'a' as i8) as usize;
            cur = cur.children[idx].get_or_insert_with(|| Box::new(Trie::new()));
        }
        cur.is_word = true;
    }

    pub fn get_root(&self, word: &str) -> Option<String> {
        let mut cur = self;
        let mut chars: Vec<char> = vec![];
        for c in word.chars() {
            let idx = (c as i8 - 'a' as i8) as usize;
            match cur.children[idx].as_ref() {
                Some(a) => cur = a,
                None => return None,
            }
            chars.push(c);
            if cur.is_word {
                return Some(chars.into_iter().collect());
            }
        }
        return None;
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        for word in dictionary {
            trie.insert(word)
        }
        sentence
            .split(' ')
            .map(|w| trie.get_root(w).unwrap_or(w.to_string()))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0648_example_1() {
        let dictionary = vec_string!["cat", "bat", "rat"];
        let sentence = "the cattle was rattled by the battery".to_string();
        let result = "the cat was rat by the bat".to_string();

        assert_eq!(Solution::replace_words(dictionary, sentence), result);
    }

    #[test]
    fn test_0648_example_2() {
        let dictionary = vec_string!["a", "b", "c"];
        let sentence = "aadsfasf absbs bbab cadsfafs".to_string();
        let result = "a a b c".to_string();

        assert_eq!(Solution::replace_words(dictionary, sentence), result);
    }
}
