/**
 * [0744] Find Smallest Letter Greater Than Target
 *
 * Given a characters array letters that is sorted in non-decreasing order and a character target, return the smallest character in the array that is larger than target.
 * Note that the letters wrap around.
 *
 * 	For example, if target == 'z' and letters == ['a', 'b'], the answer is 'a'.
 *
 *  
 * Example 1:
 *
 * Input: letters = ["c","f","j"], target = "a"
 * Output: "c"
 *
 * Example 2:
 *
 * Input: letters = ["c","f","j"], target = "c"
 * Output: "f"
 *
 * Example 3:
 *
 * Input: letters = ["c","f","j"], target = "d"
 * Output: "f"
 *
 *  
 * Constraints:
 *
 * 	2 <= letters.length <= 10^4
 * 	letters[i] is a lowercase English letter.
 * 	letters is sorted in non-decreasing order.
 * 	letters contains at least two different characters.
 * 	target is a lowercase English letter.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-smallest-letter-greater-than-target/
// discuss: https://leetcode.com/problems/find-smallest-letter-greater-than-target/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        *letters.iter().min_by_key(|&&c| (c <= target, c)).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0744_example_1() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'a';
        let result = 'c';

        assert_eq!(Solution::next_greatest_letter(letters, target), result);
    }

    #[test]
    fn test_0744_example_2() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'c';
        let result = 'f';

        assert_eq!(Solution::next_greatest_letter(letters, target), result);
    }

    #[test]
    fn test_0744_example_3() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'd';
        let result = 'f';

        assert_eq!(Solution::next_greatest_letter(letters, target), result);
    }
}
