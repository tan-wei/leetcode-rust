/**
 * [1032] Stream of Characters
 *
 * Design an algorithm that accepts a stream of characters and checks if a suffix of these characters is a string of a given array of strings words.
 * For example, if words = ["abc", "xyz"] and the stream added the four characters (one by one) 'a', 'x', 'y', and 'z', your algorithm should detect that the suffix "xyz" of the characters "axyz" matches "xyz" from words.
 * Implement the StreamChecker class:
 *
 * 	StreamChecker(String[] words) Initializes the object with the strings array words.
 * 	boolean query(char letter) Accepts a new character from the stream and returns true if any non-empty suffix from the stream forms a word that is in words.
 *
 *  
 * Example 1:
 *
 * Input
 * ["StreamChecker", "query", "query", "query", "query", "query", "query", "query", "query", "query", "query", "query", "query"]
 * [[["cd", "f", "kl"]], ["a"], ["b"], ["c"], ["d"], ["e"], ["f"], ["g"], ["h"], ["i"], ["j"], ["k"], ["l"]]
 * Output
 * [null, false, false, false, true, false, true, false, false, false, false, false, true]
 * Explanation
 * StreamChecker streamChecker = new StreamChecker(["cd", "f", "kl"]);
 * streamChecker.query("a"); // return False
 * streamChecker.query("b"); // return False
 * streamChecker.query("c"); // return False
 * streamChecker.query("d"); // return True, because 'cd' is in the wordlist
 * streamChecker.query("e"); // return False
 * streamChecker.query("f"); // return True, because 'f' is in the wordlist
 * streamChecker.query("g"); // return False
 * streamChecker.query("h"); // return False
 * streamChecker.query("i"); // return False
 * streamChecker.query("j"); // return False
 * streamChecker.query("k"); // return False
 * streamChecker.query("l"); // return True, because 'kl' is in the wordlist
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 2000
 * 	1 <= words[i].length <= 200
 * 	words[i] consists of lowercase English letters.
 * 	letter is a lowercase English letter.
 * 	At most 4 * 10^4 calls will be made to query.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/stream-of-characters/
// discuss: https://leetcode.com/problems/stream-of-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/stream-of-characters/solutions/1610905/rust-sorted-array-binary-search-solution/

struct StreamChecker {
    dict: Vec<Vec<char>>,
    query: Vec<char>,
}

fn partition_point<F>(words: &[Vec<char>], mut f: F) -> usize
where
    F: FnMut(&Vec<char>) -> bool,
{
    let mut lo = 0;
    let mut hi = words.len();
    while lo < hi {
        let mi = lo + (hi - lo) / 2;
        if f(&words[mi]) {
            lo = mi + 1;
        } else {
            hi = mi;
        }
    }
    lo
}

fn prefix_exists(words: &[Vec<char>], char_i: usize, query: &[char]) -> bool {
    let last_c = query[query.len() - 1 - char_i];
    let lo = partition_point(words, |x| x[char_i] < last_c);
    let hi = partition_point(words, |x| x[char_i] <= last_c);
    assert!(lo <= hi);
    if lo == hi {
        false
    } else if words[lo..hi].iter().any(|w| w.len() <= char_i + 1) {
        true
    } else if query.len() <= char_i + 1 {
        false
    } else {
        prefix_exists(&words[lo..hi], char_i + 1, query)
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut dict = words
            .iter()
            .map(|s| s.chars().rev().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        dict.sort();
        StreamChecker {
            dict,
            query: vec![],
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.query.push(letter);
        prefix_exists(&self.dict, 0, &self.query)
    }
}

/**
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1032_example_1() {
        let mut stream_checker = StreamChecker::new(vec_string!["cd", "f", "kl"]);
        assert_eq!(stream_checker.query('a'), false); // return False
        assert_eq!(stream_checker.query('b'), false); // return False
        assert_eq!(stream_checker.query('c'), false); // return False
        assert_eq!(stream_checker.query('d'), true); // return True, because 'cd' is in the wordlist
        assert_eq!(stream_checker.query('e'), false); // return False
        assert_eq!(stream_checker.query('f'), true); // return True, because 'f' is in the wordlist
        assert_eq!(stream_checker.query('g'), false); // return False
        assert_eq!(stream_checker.query('h'), false); // return False
        assert_eq!(stream_checker.query('i'), false); // return False
        assert_eq!(stream_checker.query('j'), false); // return False
        assert_eq!(stream_checker.query('k'), false); // return False
        assert_eq!(stream_checker.query('l'), true); // return True, because 'kl' is in the wordlist
    }
}
