/**
 * [187] Repeated DNA Sequences
 *
 * The DNA sequence is composed of a series of nucleotides abbreviated as 'A', 'C', 'G', and 'T'.
 *
 * 	For example, "ACGAATTCCG" is a DNA sequence.
 *
 * When studying DNA, it is useful to identify repeated sequences within the DNA.
 * Given a string s that represents a DNA sequence, return all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule. You may return the answer in any order.
 *  
 * Example 1:
 * Input: s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
 * Output: ["AAAAACCCCC","CCCCCAAAAA"]
 * Example 2:
 * Input: s = "AAAAAAAAAAAAA"
 * Output: ["AAAAAAAAAA"]
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is either 'A', 'C', 'G', or 'T'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/repeated-dna-sequences/
// discuss: https://leetcode.com/problems/repeated-dna-sequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return vec![];
        }

        let mut hm = std::collections::HashMap::new();
        for i in 0..s.len() - 9 {
            let si = &s[i..i + 10];
            if hm.contains_key(si) {
                hm.insert(si, true);
            } else {
                hm.insert(si, false);
            }
        }

        hm.keys()
            .filter(|&s| hm[s])
            .map(|s| s.to_string())
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0187_example_1() {
        let s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string();
        let result = vec_string!["AAAAACCCCC", "CCCCCAAAAA"];

        assert_eq_sorted!(Solution::find_repeated_dna_sequences(s), result);
    }

    #[test]
    fn test_0187_example_2() {
        let s = "AAAAAAAAAAAAA".to_string();
        let result = vec_string!["AAAAAAAAAA"];

        assert_eq_sorted!(Solution::find_repeated_dna_sequences(s), result);
    }
}
