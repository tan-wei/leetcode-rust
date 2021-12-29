/**
 * [0466] Count The Repetitions
 *
 * We define str = [s, n] as the string str which consists of the string s concatenated n times.
 *
 * 	For example, str == ["abc", 3] =="abcabcabc".
 *
 * We define that string s1 can be obtained from string s2 if we can remove some characters from s2 such that it becomes s1.
 *
 * 	For example, s1 = "abc" can be obtained from s2 = "ab<u>dbe</u>c" based on our definition by removing the bolded underlined characters.
 *
 * You are given two strings s1 and s2 and two integers n1 and n2. You have the two strings str1 = [s1, n1] and str2 = [s2, n2].
 * Return the maximum integer m such that str = [str2, m] can be obtained from str1.
 *  
 * Example 1:
 * Input: s1 = "acb", n1 = 4, s2 = "ab", n2 = 2
 * Output: 2
 * Example 2:
 * Input: s1 = "acb", n1 = 1, s2 = "acb", n2 = 1
 * Output: 1
 *  
 * Constraints:
 *
 * 	1 <= s1.length, s2.length <= 100
 * 	s1 and s2 consist of lowercase English letters.
 * 	1 <= n1, n2 <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-repetitions/
// discuss: https://leetcode.com/problems/count-the-repetitions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/count-the-repetitions/discuss/791499/Rust-translated-8ms-100
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        if n1 == 0 {
            return 0;
        };
        let mut indices = vec![0; n1 as usize + 1];
        let mut counts = vec![0; n1 as usize + 1];
        let mut index = 0;
        let mut count = 0;
        for i in 1..=n1 as usize {
            for j in 0..s1.len() {
                if s1.as_bytes()[j] == s2.as_bytes()[index] {
                    index += 1;
                }
                if index == s2.len() {
                    index = 0;
                    count += 1;
                }
            }
            counts[i] = count;
            indices[i] = index;
            for k in 0..i {
                if indices[k] == index {
                    let pre_count = counts[k];
                    let pattern_count = (n1 - k as i32) / (i - k) as i32 * (counts[i] - pre_count);
                    let remain_count = counts[k + (n1 as usize - k) % (i - k)] - pre_count;
                    return (pre_count + pattern_count + remain_count) / n2;
                }
            }
        }
        counts[n1 as usize] / n2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0466_example_1() {
        let s1 = "acb".to_string();
        let n1 = 4;
        let s2 = "ab".to_string();
        let n2 = 2;
        let result = 2;

        assert_eq!(Solution::get_max_repetitions(s1, n1, s2, n2), result);
    }

    #[test]
    fn test_0466_example_2() {
        let s1 = "acb".to_string();
        let n1 = 1;
        let s2 = "acb".to_string();
        let n2 = 1;
        let result = 1;

        assert_eq!(Solution::get_max_repetitions(s1, n1, s2, n2), result);
    }
}
