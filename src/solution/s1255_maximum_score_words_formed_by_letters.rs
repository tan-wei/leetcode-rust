/**
 * [1255] Maximum Score Words Formed by Letters
 *
 * Given a list of words, list of  single letters (might be repeating) and score of every character.
 * Return the maximum score of any valid set of words formed by using the given letters (words[i] cannot be used two or more times).
 * It is not necessary to use all characters in letters and each letter can only be used once. Score of letters 'a', 'b', 'c', ... ,'z' is given by score[0], score[1], ... , score[25] respectively.
 *  
 * Example 1:
 *
 * Input: words = ["dog","cat","dad","good"], letters = ["a","a","c","d","d","d","g","o","o"], score = [1,0,9,5,0,0,3,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0]
 * Output: 23
 * Explanation:
 * Score  a=1, c=9, d=5, g=3, o=2
 * Given letters, we can form the words "dad" (5+1+5) and "good" (3+2+2+5) with a score of 23.
 * Words "dad" and "dog" only get a score of 21.
 * Example 2:
 *
 * Input: words = ["xxxz","ax","bx","cx"], letters = ["z","a","b","c","x","x","x"], score = [4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,10]
 * Output: 27
 * Explanation:
 * Score  a=4, b=4, c=4, x=5, z=10
 * Given letters, we can form the words "ax" (4+5), "bx" (4+5) and "cx" (4+5) with a score of 27.
 * Word "xxxz" only get a score of 25.
 * Example 3:
 *
 * Input: words = ["leetcode"], letters = ["l","e","t","c","o","d"], score = [0,0,1,1,1,0,0,0,0,0,0,1,0,0,1,0,0,0,0,1,0,0,0,0,0,0]
 * Output: 0
 * Explanation:
 * Letter "e" can only be used once.
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 14
 * 	1 <= words[i].length <= 15
 * 	1 <= letters.length <= 100
 * 	letters[i].length == 1
 * 	score.length == 26
 * 	0 <= score[i] <= 10
 * 	words[i], letters[i] contains only lower case English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-score-words-formed-by-letters/
// discuss: https://leetcode.com/problems/maximum-score-words-formed-by-letters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-score-words-formed-by-letters/solutions/3443996/rust-knapsack-problem-dfs/
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let num_words = words.len();

        // Convert each word in words into [u8; 26] array of letter counts
        let word_arrs: Vec<[u8; 26]> = words
            .iter()
            .map(|word| {
                word.as_bytes().iter().fold([0; 26], |mut arr, &byte| {
                    arr[(byte - b'a') as usize] += 1;
                    arr
                })
            })
            .collect();

        // Take inventory of the letters we have, convert from char to byte
        let inventory = {
            // Used for encoding as UTF-8
            let mut buf: [u8; 4] = [0, 0, 0, 0];
            let mut inventory = [0; 26];

            for letter in letters.into_iter() {
                inventory
                    [(letter.encode_utf8(&mut buf).as_bytes().get(0).unwrap() - b'a') as usize] +=
                    1;
            }

            // Return from block, make immutable
            inventory
        };

        // Do a depth-first search of possible words
        // Use a hashmap to keep track of what word combinations we've already visited
        let mut visited = std::collections::HashMap::<u16, i32>::new();
        visited.insert(0, 0);

        // Start our search at "no words", full letter inventory, no score
        let mut stack: Vec<(u16, [u8; 26], i32)> = Vec::new();
        stack.push((0, inventory, 0));

        // Do the search until completed
        while let Some((used, available, curr_score)) = stack.pop() {
            // Try adding each word
            for word in 0..num_words {
                if (used >> word) & 1 == 1 {
                    // Word is already being used, can't add twice
                    continue;
                }

                let new_used = used | (1 << word);
                if visited.contains_key(&new_used) {
                    // Already visited this combination of words
                    continue;
                }

                // Check to see if this new word combination is possible and reduce inventory accordingly
                let mut possible = true;
                let mut new_available = available.clone();
                let mut new_score = curr_score;

                for letter in 0..26 {
                    if word_arrs[word][letter] > available[letter] {
                        // Don't have enough letters left to add this word
                        possible = false;
                        break;
                    } else {
                        // Subtract the word's letter costs from available inventory
                        new_available[letter] -= word_arrs[word][letter];
                        // Update score to reflect added letter
                        new_score += (word_arrs[word][letter] as i32) * score[letter];
                    }
                }

                if !possible {
                    continue;
                }

                // Add the new word combination to the stack
                visited.insert(new_used, new_score);
                stack.push((new_used, new_available, new_score));
            }
        }

        // Take the best score out of all visited combinations
        // If no word combinations are possible, score is 0
        *visited.values().max().unwrap_or(&0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1255_example_1() {
        let words = vec_string!["dog", "cat", "dad", "good"];
        let letters = vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'];
        let score = vec![
            1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let result = 23;

        assert_eq!(Solution::max_score_words(words, letters, score), result);
    }

    #[test]
    fn test_1255_example_2() {
        let words = vec_string!["xxxz", "ax", "bx", "cx"];
        let letters = vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'];
        let score = vec![
            4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10,
        ];
        let result = 27;

        assert_eq!(Solution::max_score_words(words, letters, score), result);
    }

    #[test]
    fn test_1255_example_3() {
        let words = vec_string!["leetcode"];
        let letters = vec!['l', 'e', 't', 'c', 'o', 'd'];
        let score = vec![
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        ];
        let result = 0;

        assert_eq!(Solution::max_score_words(words, letters, score), result);
    }
}
