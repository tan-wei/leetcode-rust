/**
 * [0423] Reconstruct Original Digits from English
 *
 * Given a string s containing an out-of-order English representation of digits 0-9, return the digits in ascending order.
 *  
 * Example 1:
 * Input: s = "owoztneoer"
 * Output: "012"
 * Example 2:
 * Input: s = "fviefuro"
 * Output: "45"
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is one of the characters ["e","g","f","i","h","o","n","s","r","u","t","w","v","x","z"].
 * 	s is guaranteed to be valid.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reconstruct-original-digits-from-english/
// discuss: https://leetcode.com/problems/reconstruct-original-digits-from-english/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn original_digits(s: String) -> String {
        let counts = s.as_bytes().iter().fold([0; 26], |mut acc, x| {
            acc[(x - b'a') as usize] += 1;
            acc
        });
        let mut answer = [0; 10];
        answer[0] = counts[(b'z' - b'a') as usize];
        answer[2] = counts[(b'w' - b'a') as usize];
        answer[4] = counts[(b'u' - b'a') as usize];
        answer[6] = counts[(b'x' - b'a') as usize];
        answer[8] = counts[(b'g' - b'a') as usize];
        answer[3] = counts[(b'h' - b'a') as usize] - answer[8];
        answer[5] = counts[(b'f' - b'a') as usize] - answer[4];
        answer[7] = counts[(b's' - b'a') as usize] - answer[6];
        answer[1] = counts[(b'o' - b'a') as usize] - answer[0] - answer[2] - answer[4];
        answer[9] = counts[(b'i' - b'a') as usize] - answer[5] - answer[6] - answer[8];
        (0..10)
            .flat_map(|i| std::iter::repeat((i as u8 + b'0') as char).take(answer[i]))
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0423_example_1() {
        let s = "owoztneoer".to_string();
        let result = "012".to_string();

        assert_eq!(Solution::original_digits(s), result);
    }

    #[test]
    fn test_0423_example_2() {
        let s = "fviefuro".to_string();
        let result = "45".to_string();

        assert_eq!(Solution::original_digits(s), result);
    }
}
