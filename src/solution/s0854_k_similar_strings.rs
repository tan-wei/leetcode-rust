/**
 * [0854] K-Similar Strings
 *
 * Strings s1 and s2 are k-similar (for some non-negative integer k) if we can swap the positions of two letters in s1 exactly k times so that the resulting string equals s2.
 * Given two anagrams s1 and s2, return the smallest k for which s1 and s2 are k-similar.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s1 = "ab", s2 = "ba"
 * Output: 1
 * Explanation: The two string are 1-similar because we can use one swap to change s1 to s2: "ab" --> "ba".
 *
 * <strong class="example">Example 2:
 *
 * Input: s1 = "abc", s2 = "bca"
 * Output: 2
 * Explanation: The two strings are 2-similar because we can use two swaps to change s1 to s2: "abc" --> "bac" --> "bca".
 *
 *  
 * Constraints:
 *
 * 	1 <= s1.length <= 20
 * 	s2.length == s1.length
 * 	s1 and s2 contain only lowercase letters from the set {'a', 'b', 'c', 'd', 'e', 'f'}.
 * 	s2 is an anagram of s1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/k-similar-strings/
// discuss: https://leetcode.com/problems/k-similar-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/k-similar-strings/solutions/409380/rust-with-bfs/
    pub fn k_similarity(s1: String, s2: String) -> i32 {
        let string_len = s1.len();

        let mut similarity = 0;
        let mut seen = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(s1);

        while queue.len() > 0 {
            let mut queue_size = queue.len();
            for current_iter in 0..queue_size {
                let front = queue.pop_front().unwrap();
                if &front == &s2 {
                    return similarity;
                }

                let mut i = 0;

                let mut front_itr = &front.chars();
                let mut s2_itr = &s2.chars();

                while &front.chars().skip(i).next().unwrap() == &s2.chars().skip(i).next().unwrap()
                {
                    i += 1;
                }

                let target = s2.chars().skip(i).next().unwrap();

                let mut j = i + 1;

                while j < string_len {
                    if &front.chars().skip(j).next().unwrap() == &target {
                        let string = Self::swap_string(i, j, front.clone());
                        if seen.insert(string.clone()) {
                            queue.push_back(string);
                        }
                    }

                    j += 1;
                }
            }

            similarity += 1;
        }

        similarity
    }

    fn swap_string(i: usize, j: usize, mut s: String) -> String {
        unsafe {
            let mut slice = s.as_bytes_mut();
            let temp = slice[i];
            slice[i] = slice[j];
            slice[j] = temp;
        }

        s
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0854_example_1() {
        let s1 = "ab".to_string();
        let s2 = "ba".to_string();
        let result = 1;

        assert_eq!(Solution::k_similarity(s1, s2), result);
    }

    #[test]
    fn test_0854_example_2() {
        let s1 = "abc".to_string();
        let s2 = "bca".to_string();
        let result = 2;

        assert_eq!(Solution::k_similarity(s1, s2), result);
    }
}
