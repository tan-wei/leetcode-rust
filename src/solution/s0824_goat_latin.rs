/**
 * [0824] Goat Latin
 *
 * You are given a string sentence that consist of words separated by spaces. Each word consists of lowercase and uppercase letters only.
 * We would like to convert the sentence to "Goat Latin" (a made-up language similar to Pig Latin.) The rules of Goat Latin are as follows:
 *
 * 	If a word begins with a vowel ('a', 'e', 'i', 'o', or 'u'), append "ma" to the end of the word.
 *
 * 		For example, the word "apple" becomes "applema".
 *
 *
 * 	If a word begins with a consonant (i.e., not a vowel), remove the first letter and append it to the end, then add "ma".
 *
 * 		For example, the word "goat" becomes "oatgma".
 *
 *
 * 	Add one letter 'a' to the end of each word per its word index in the sentence, starting with 1.
 *
 * 		For example, the first word gets "a" added to the end, the second word gets "aa" added to the end, and so on.
 *
 *
 *
 * Return the final sentence representing the conversion from sentence to Goat Latin.
 *  
 * Example 1:
 * Input: sentence = "I speak Goat Latin"
 * Output: "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
 * Example 2:
 * Input: sentence = "The quick brown fox jumped over the lazy dog"
 * Output: "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"
 *  
 * Constraints:
 *
 * 	1 <= sentence.length <= 150
 * 	sentence consists of English letters and spaces.
 * 	sentence has no leading or trailing spaces.
 * 	All the words in sentence are separated by a single space.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/goat-latin/
// discuss: https://leetcode.com/problems/goat-latin/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut v = Vec::new();
        for (n, word) in sentence.split_ascii_whitespace().enumerate() {
            let trans_word = match word.starts_with(&vowels[..]) {
                true => word.to_string(),
                false => word[1..].to_string() + &word[..1],
            } + "ma";
            v.push(trans_word + &"a".repeat(n + 1))
        }
        v.join(" ")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0824_example_1() {
        let sentence = "I speak Goat Latin".to_string();
        let result = "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string();

        assert_eq!(Solution::to_goat_latin(sentence), result);
    }

    #[test]
    fn test_0824_example_2() {
        let sentence = "The quick brown fox jumped over the lazy dog".to_string();
        let result = "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa".to_string();

        assert_eq!(Solution::to_goat_latin(sentence), result);
    }
}
