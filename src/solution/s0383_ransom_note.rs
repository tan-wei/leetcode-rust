/**
 * [383] Ransom Note
 *
 * Given two stings ransomNote and magazine, return true if ransomNote can be constructed from magazine and false otherwise.
 * Each letter in magazine can only be used once in ransomNote.
 *  
 * Example 1:
 * Input: ransomNote = "a", magazine = "b"
 * Output: false
 * Example 2:
 * Input: ransomNote = "aa", magazine = "ab"
 * Output: false
 * Example 3:
 * Input: ransomNote = "aa", magazine = "aab"
 * Output: true
 *  
 * Constraints:
 *
 * 	1 <= ransomNote.length, magazine.length <= 10^5
 * 	ransomNote and magazine consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ransom-note/
// discuss: https://leetcode.com/problems/ransom-note/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        fn find_letter_occurences(s: &str) -> [i32; 26] {
            s.bytes().fold([0; 26], |mut acc, chr| {
                let current = chr - b'a';
                acc[current as usize] += 1;
                acc
            })
        }

        let needed = find_letter_occurences(&ransom_note);
        let available = find_letter_occurences(&magazine);

        needed.iter().zip(available.iter()).all(|(a, b)| a <= b)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0383_example_1() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();
        let result = false;

        assert_eq!(Solution::can_construct(ransom_note, magazine), result);
    }

    #[test]
    fn test_0383_example_2() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();
        let result = false;

        assert_eq!(Solution::can_construct(ransom_note, magazine), result);
    }

    #[test]
    fn test_0383_example_3() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();
        let result = true;

        assert_eq!(Solution::can_construct(ransom_note, magazine), result);
    }
}
