/**
 * [0748] Shortest Completing Word
 *
 * Given a string licensePlate and an array of strings words, find the shortest completing word in words.
 * A completing word is a word that contains all the letters in licensePlate. Ignore numbers and spaces in licensePlate, and treat letters as case insensitive. If a letter appears more than once in licensePlate, then it must appear in the word the same number of times or more.
 * For example, if licensePlate = "aBc 12c", then it contains letters 'a', 'b' (ignoring case), and 'c' twice. Possible completing words are "abccdef", "caaacab", and "cbca".
 * Return the shortest completing word in words. It is guaranteed an answer exists. If there are multiple shortest completing words, return the first one that occurs in words.
 *  
 * Example 1:
 *
 * Input: licensePlate = "1s3 PSt", words = ["step","steps","stripe","stepple"]
 * Output: "steps"
 * Explanation: licensePlate contains letters 's', 'p', 's' (ignoring case), and 't'.
 * "step" contains 't' and 'p', but only contains 1 's'.
 * "steps" contains 't', 'p', and both 's' characters.
 * "stripe" is missing an 's'.
 * "stepple" is missing an 's'.
 * Since "steps" is the only word containing all the letters, that is the answer.
 *
 * Example 2:
 *
 * Input: licensePlate = "1s3 456", words = ["looks","pest","stew","show"]
 * Output: "pest"
 * Explanation: licensePlate only contains the letter 's'. All the words contain 's', but among these "pest", "stew", and "show" are shortest. The answer is "pest" because it is the word that appears earliest of the 3.
 *
 *  
 * Constraints:
 *
 * 	1 <= licensePlate.length <= 7
 * 	licensePlate contains digits, letters (uppercase or lowercase), or space ' '.
 * 	1 <= words.length <= 1000
 * 	1 <= words[i].length <= 15
 * 	words[i] consists of lower case English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-completing-word/
// discuss: https://leetcode.com/problems/shortest-completing-word/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut counter_lic = [0; 26];
        license_plate
            .chars()
            .filter_map(|c| {
                if c.is_alphabetic() {
                    Some(c.to_ascii_lowercase())
                } else {
                    None
                }
            })
            .for_each(|c| counter_lic[((c as u8) - b'a') as usize] += 1);

        let mut short_word = String::new();
        for word in words.into_iter() {
            let mut counter_word = [0; 26];
            word.as_bytes()
                .iter()
                .for_each(|&b| counter_word[(b - b'a') as usize] += 1);

            if counter_lic
                .iter()
                .zip(counter_word.iter())
                .all(|(l, w)| *w >= *l)
                && (short_word.is_empty() || short_word.len() > word.len())
            {
                short_word = word;
            }
        }
        short_word
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0748_example_1() {
        let license_plate = "1s3 PSt".to_string();
        let words = vec_string!["step", "steps", "stripe", "stepple"];
        let result = "steps".to_string();

        assert_eq!(
            Solution::shortest_completing_word(license_plate, words),
            result
        );
    }

    #[test]
    fn test_0748_example_2() {
        let license_plate = "1s3 456".to_string();
        let words = vec_string!["looks", "pest", "stew", "show"];
        let result = "pest".to_string();

        assert_eq!(
            Solution::shortest_completing_word(license_plate, words),
            result
        );
    }
}
