/**
 * [0843] Guess the Word
 *
 * You are given an array of unique strings words where words[i] is six letters long. One word of words was chosen as a secret word.
 * You are also given the helper object Master. You may call Master.guess(word) where word is a six-letter-long string, and it must be from words. Master.guess(word) returns:
 *
 * 	-1 if word is not from words, or
 * 	an integer representing the number of exact matches (value and position) of your guess to the secret word.
 *
 * There is a parameter allowedGuesses for each test case where allowedGuesses is the maximum number of times you can call Master.guess(word).
 * For each test case, you should call Master.guess with the secret word without exceeding the maximum number of allowed guesses. You will get:
 *
 * 	"Either you took too many guesses, or you did not find the secret word." if you called Master.guess more than allowedGuesses times or if you did not call Master.guess with the secret word, or
 * 	"You guessed the secret word correctly." if you called Master.guess with the secret word with the number of calls to Master.guess less than or equal to allowedGuesses.
 *
 * The test cases are generated such that you can guess the secret word with a reasonable strategy (other than using the bruteforce method).
 *  
 * Example 1:
 *
 * Input: secret = "acckzz", words = ["acckzz","ccbazz","eiowzz","abcczz"], allowedGuesses = 10
 * Output: You guessed the secret word correctly.
 * Explanation:
 * master.guess("aaaaaa") returns -1, because "aaaaaa" is not in wordlist.
 * master.guess("acckzz") returns 6, because "acckzz" is secret and has all 6 matches.
 * master.guess("ccbazz") returns 3, because "ccbazz" has 3 matches.
 * master.guess("eiowzz") returns 2, because "eiowzz" has 2 matches.
 * master.guess("abcczz") returns 4, because "abcczz" has 4 matches.
 * We made 5 calls to master.guess, and one of them was the secret, so we pass the test case.
 *
 * Example 2:
 *
 * Input: secret = "hamada", words = ["hamada","khaled"], allowedGuesses = 10
 * Output: You guessed the secret word correctly.
 * Explanation: Since there are two words, you can guess both.
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 100
 * 	words[i].length == 6
 * 	words[i] consist of lowercase English letters.
 * 	All the strings of wordlist are unique.
 * 	secret exists in words.
 * 	10 <= allowedGuesses <= 30
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/guess-the-word/
// discuss: https://leetcode.com/problems/guess-the-word/discuss/?currentPage=1&orderBy=most_votes&query=

#[derive(Clone, Copy)]
pub struct Master;

impl Master {
    pub fn guess(self, word: String) -> i32 {
        0
    }
}

// submission codes start here

/**
 * // This is the Master's API interface.
 * // You should not implement it, or speculate about its implementation
 * struct Master;
 * impl Master {
 *     fn guess(word:String)->int;
 * };
 */

impl Solution {
    // Credit: https://leetcode.com/problems/guess-the-word/solutions/1081646/rust-translated/
    pub fn find_secret_word(words: Vec<String>, master: &Master) {
        let mut h = vec![vec![0; words.len()]; words.len()];
        for i in 0..words.len() - 1 {
            for j in i + 1..words.len() {
                h[i][j] = words[i]
                    .chars()
                    .zip(words[j].chars())
                    .filter(|&cs| cs.0 == cs.1)
                    .count();
                h[j][i] = h[i][j];
            }
        }
        let mut possible = (0..words.len()).collect::<Vec<_>>();
        let mut path = std::collections::HashSet::with_capacity(words.len());
        let solve = |possible: &[usize], path: &std::collections::HashSet<usize>| -> usize {
            let mut ret = 0;
            let mut minsize = possible.len();
            for i in 0..words.len() {
                if !path.contains(&i) {
                    let mut groups = vec![Vec::new(); 6];
                    for &j in possible.iter() {
                        if i != j {
                            groups[h[i][j]].push(j);
                        }
                    }
                    if let Some(maxsize) = groups.iter().map(|g| g.len()).max() {
                        if maxsize < minsize {
                            minsize = maxsize;
                            ret = i;
                        }
                    }
                }
            }
            ret
        };
        while !possible.is_empty() {
            let guess = solve(&possible, &path);
            let m = master.guess(words[guess].clone()) as usize;
            if m == 6 {
                return;
            }
            possible.retain(|&i| h[guess][i] == m);
            path.insert(guess);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_0843_example_1() {
        let secret = "acckzz".to_string();
        let words = vec_string!["acckzz", "ccbazz", "eiowzz", "abcczz"];
    }

    #[ignore]
    #[test]
    fn test_0843_example_2() {
        let secret = "hamada".to_string();
        let words = vec_string!["hamada", "khaled"];
    }
}
