/**
 * [0433] Minimum Genetic Mutation
 *
 * A gene string can be represented by an 8-character long string, with choices from 'A', 'C', 'G', and 'T'.
 * Suppose we need to investigate a mutation from a gene string start to a gene string end where one mutation is defined as one single character changed in the gene string.
 *
 * 	For example, "AACCGGTT" --> "AACCGGTA" is one mutation.
 *
 * There is also a gene bank bank that records all the valid gene mutations. A gene must be in bank to make it a valid gene string.
 * Given the two gene strings start and end and the gene bank bank, return the minimum number of mutations needed to mutate from start to end. If there is no such a mutation, return -1.
 * Note that the starting point is assumed to be valid, so it might not be included in the bank.
 *  
 * Example 1:
 *
 * Input: start = "AACCGGTT", end = "AACCGGTA", bank = ["AACCGGTA"]
 * Output: 1
 *
 * Example 2:
 *
 * Input: start = "AACCGGTT", end = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"]
 * Output: 2
 *
 * Example 3:
 *
 * Input: start = "AAAAACCC", end = "AACCCCCC", bank = ["AAAACCCC","AAACCCCC","AACCCCCC"]
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	start.length == 8
 * 	end.length == 8
 * 	0 <= bank.length <= 10
 * 	bank[i].length == 8
 * 	start, end, and bank[i] consist of only the characters ['A', 'C', 'G', 'T'].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-genetic-mutation/
// discuss: https://leetcode.com/problems/minimum-genetic-mutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-genetic-mutation/discuss/1181572/Rust-cheapest-and-best
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let bank: std::collections::HashSet<String> =
            bank.into_iter().chain(vec![start.clone()]).collect();

        if !bank.contains(&end) {
            return -1;
        }

        let (mut seen_left, mut seen_right) = (
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
        );
        let (mut q1, mut q2) = (
            std::collections::VecDeque::new(),
            std::collections::VecDeque::new(),
        );
        q1.push_back((start, 0));
        q2.push_back((end, 0));

        while q1.len() > 0 || q2.len() > 0 {
            if let Some((a, steps_left)) = q1.pop_front() {
                for b in bank
                    .iter()
                    .filter(|s| !seen_left.contains_key(*s) && s != &&a)
                {
                    if Self::is_mutation(a.as_str(), b.as_str()) {
                        if let Some(steps_right) = seen_right.get(b) {
                            return steps_left + 1 + steps_right;
                        }

                        q1.push_back((b.clone(), steps_left + 1));
                    }
                }

                seen_left.insert(a, steps_left);
            }

            if let Some((a, steps_right)) = q2.pop_front() {
                for b in bank
                    .iter()
                    .filter(|s| !seen_right.contains_key(*s) && s != &&a)
                {
                    if Self::is_mutation(a.as_str(), b.as_str()) {
                        if let Some(steps_left) = seen_left.get(b) {
                            return steps_left + 1 + steps_right;
                        }

                        q2.push_back((b.clone(), steps_right + 1));
                    }
                }

                seen_right.insert(a, steps_right);
            }
        }

        -1
    }

    fn is_mutation(a: &str, b: &str) -> bool {
        let mut diff = 0;
        let (mut a_chars, mut b_chars) = (a.chars(), b.chars());

        while let (Some(a), Some(b)) = (a_chars.next(), b_chars.next()) {
            if a != b {
                diff += 1;
                if diff > 1 {
                    return false;
                }
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0433_example_1() {
        let start = "AACCGGTT".to_string();
        let end = "AACCGGTA".to_string();
        let bank = vec_string!["AACCGGTA"];
        let result = 1;

        assert_eq!(Solution::min_mutation(start, end, bank), result);
    }
}
