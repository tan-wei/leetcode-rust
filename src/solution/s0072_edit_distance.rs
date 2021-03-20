/**
 * [72] Edit Distance
 *
 * Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.
 * You have the following three operations permitted on a word:
 *
 * 	Insert a character
 * 	Delete a character
 * 	Replace a character
 *
 *  
 * Example 1:
 *
 * Input: word1 = "horse", word2 = "ros"
 * Output: 3
 * Explanation:
 * horse -> rorse (replace 'h' with 'r')
 * rorse -> rose (remove 'r')
 * rose -> ros (remove 'e')
 *
 * Example 2:
 *
 * Input: word1 = "intention", word2 = "execution"
 * Output: 5
 * Explanation:
 * intention -> inention (remove 't')
 * inention -> enention (replace 'i' with 'e')
 * enention -> exention (replace 'n' with 'x')
 * exention -> exection (replace 'n' with 'c')
 * exection -> execution (insert 'u')
 *
 *  
 * Constraints:
 *
 * 	0 <= word1.length, word2.length <= 500
 * 	word1 and word2 consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/edit-distance/
// discuss: https://leetcode.com/problems/edit-distance/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // Treat characters as raw bytes, as it allows us to directly access the underlying arrays:
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());

        // Allocate memory in one-go, as it is typically faster:
        let mut dist = Vec::with_capacity(word2.len() + 1);

        // Base case: we need to delete j characters in word2 in order to match the empty string word1:
        for j in 0..=word2.len() {
            dist.push(j)
        }

        // Use a second vector to store distances for i - 1.
        // This uses less memory than having a matrix of size (m, n),
        // and we always just use the previous row in the matrix anyway:
        let mut prev_dist = dist.clone();

        for i in 1..=word1.len() {
            for j in 0..=word2.len() {
                if j == 0 {
                    dist[j] += 1; // Base case: we need to insert a character in order to match word1.
                } else if word1[i - 1] == word2[j - 1] {
                    // No difference, don't increment the edit distance:
                    dist[j] = prev_dist[j - 1];
                } else {
                    // Either insert, delete or replace a character: increment the edit distance by one:
                    dist[j] = dist[j].min(dist[j - 1]).min(prev_dist[j - 1]) + 1;
                }
            }
            prev_dist.copy_from_slice(&dist); // Backup the distances for this row using memcpy.
        }
        dist[word2.len()] as i32
    }

    pub fn min_distance_v2(word1: String, word2: String) -> i32 {
        // Credit: https://leetcode.com/problems/edit-distance/discuss/686022/Rust-0ms-2.1Mb-10-line-solution
        let mut m = Vec::with_capacity(word2.len());
        m.extend(word2.chars().zip(1..));
        for (i, c) in word1.chars().enumerate() {
            let mut x = i;
            let mut y = i + 1;
            for (d, v) in m.iter_mut() {
                y = std::cmp::min(x + (c != *d) as usize, std::cmp::min(y, *v) + 1);
                x = *v;
                *v = y;
            }
        }
        m.pop().map_or(word1.len(), |x| x.1) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0072_example_1() {
        let word1 = "horse".to_string();
        let word2 = "ros".to_string();
        let result = 3;

        assert_eq!(Solution::min_distance(word1, word2), result);

        let word1 = "horse".to_string();
        let word2 = "ros".to_string();
        let result = 3;

        assert_eq!(Solution::min_distance_v2(word1, word2), result);
    }

    #[test]
    fn test_0072_example_2() {
        let word1 = "intention".to_string();
        let word2 = "execution".to_string();
        let result = 5;

        assert_eq!(Solution::min_distance(word1, word2), result);

        let word1 = "intention".to_string();
        let word2 = "execution".to_string();
        let result = 5;

        assert_eq!(Solution::min_distance_v2(word1, word2), result);
    }
}
