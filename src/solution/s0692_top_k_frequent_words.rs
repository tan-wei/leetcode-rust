/**
 * [0692] Top K Frequent Words
 *
 * Given an array of strings words and an integer k, return the k most frequent strings.
 * Return the answer sorted by the frequency from highest to lowest. Sort the words with the same frequency by their lexicographical order.
 *  
 * Example 1:
 *
 * Input: words = ["i","love","leetcode","i","love","coding"], k = 2
 * Output: ["i","love"]
 * Explanation: "i" and "love" are the two most frequent words.
 * Note that "i" comes before "love" due to a lower alphabetical order.
 *
 * Example 2:
 *
 * Input: words = ["the","day","is","sunny","the","the","the","sunny","is","is"], k = 4
 * Output: ["the","is","sunny","day"]
 * Explanation: "the", "is", "sunny" and "day" are the four most frequent words, with the number of occurrence being 4, 3, 2 and 1 respectively.
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 500
 * 	1 <= words[i] <= 10
 * 	words[i] consists of lowercase English letters.
 * 	k is in the range [1, The number of unique words[i]]
 *
 *  
 * Follow-up: Could you solve it in O(n log(k)) time and O(n) extra space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/top-k-frequent-words/
// discuss: https://leetcode.com/problems/top-k-frequent-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut hm = std::collections::HashMap::new();
        for w in words {
            *hm.entry(w.clone()).or_insert(0) += 1;
        }

        let mut to_sort = vec![];

        for (k, v) in hm {
            to_sort.push((v, k));
        }

        to_sort.sort_by(|l, r| {
            if l.0 != r.0 {
                r.0.cmp(&l.0)
            } else {
                l.1.cmp(&r.1)
            }
        });

        let mut result = Vec::new();

        for i in 0..k {
            result.push(to_sort[i as usize].1.clone())
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0692_example_1() {
        let words = vec_string!["i", "love", "leetcode", "i", "love", "coding"];
        let k = 2;
        let result = vec_string!["i", "love"];

        assert_eq!(Solution::top_k_frequent(words, k), result);
    }

    #[test]
    fn test_0692_example_2() {
        let words =
            vec_string!["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"];
        let k = 4;
        let result = vec_string!["the", "is", "sunny", "day"];

        assert_eq!(Solution::top_k_frequent(words, k), result);
    }
}
