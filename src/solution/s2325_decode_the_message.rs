/**
 * [2325] Decode the Message
 *
 * You are given the strings key and message, which represent a cipher key and a secret message, respectively. The steps to decode message are as follows:
 * <ol>
 * 	Use the first appearance of all 26 lowercase English letters in key as the order of the substitution table.
 * 	Align the substitution table with the regular English alphabet.
 * 	Each letter in message is then substituted using the table.
 * 	Spaces ' ' are transformed to themselves.
 * </ol>
 *
 * 	For example, given key = "<u>hap</u>p<u>y</u> <u>bo</u>y" (actual key would have at least one instance of each letter in the alphabet), we have the partial substitution table of ('h' -> 'a', 'a' -> 'b', 'p' -> 'c', 'y' -> 'd', 'b' -> 'e', 'o' -> 'f').
 *
 * Return the decoded message.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/05/08/ex1new4.jpg" style="width: 752px; height: 150px;" />
 * Input: key = "the quick brown fox jumps over the lazy dog", message = "vkbs bs t suepuv"
 * Output: "this is a secret"
 * Explanation: The diagram above shows the substitution table.
 * It is obtained by taking the first appearance of each letter in "<u>the</u> <u>quick</u> <u>brown</u> <u>f</u>o<u>x</u> <u>j</u>u<u>mps</u> o<u>v</u>er the <u>lazy</u> <u>d</u>o<u>g</u>".
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/05/08/ex2new.jpg" style="width: 754px; height: 150px;" />
 * Input: key = "eljuxhpwnyrdgtqkviszcfmabo", message = "zwx hnfx lqantp mnoeius ycgk vcnjrdb"
 * Output: "the five boxing wizards jump quickly"
 * Explanation: The diagram above shows the substitution table.
 * It is obtained by taking the first appearance of each letter in "<u>eljuxhpwnyrdgtqkviszcfmabo</u>".
 *
 *  
 * Constraints:
 *
 * 	26 <= key.length <= 2000
 * 	key consists of lowercase English letters and ' '.
 * 	key contains every letter in the English alphabet ('a' to 'z') at least once.
 * 	1 <= message.length <= 2000
 * 	message consists of lowercase English letters and ' '.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decode-the-message/
// discuss: https://leetcode.com/problems/decode-the-message/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut set = std::collections::HashSet::from([' ']);

        let mut map: std::collections::HashMap<_, _> = key
            .chars()
            .filter_map(|c| match set.insert(c) {
                true => Some(c),
                false => None,
            })
            .zip("abcdefghijklmnopqrstuvwxyz".to_string().chars())
            .collect();

        message
            .chars()
            .map(|c| map.get(&c).unwrap_or(&' '))
            .collect::<String>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2325_example_1() {
        let key = "the quick brown fox jumps over the lazy dog".to_string();
        let message = "vkbs bs t suepuv".to_string();

        let result = "this is a secret".to_string();

        assert_eq!(Solution::decode_message(key, message), result);
    }

    #[test]
    fn test_2325_example_2() {
        let key = "eljuxhpwnyrdgtqkviszcfmabo".to_string();
        let message = "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string();

        let result = "the five boxing wizards jump quickly".to_string();

        assert_eq!(Solution::decode_message(key, message), result);
    }
}
