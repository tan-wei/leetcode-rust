/**
 * [2284] Sender With Largest Word Count
 *
 * You have a chat log of n messages. You are given two string arrays messages and senders where messages[i] is a message sent by senders[i].
 * A message is list of words that are separated by a single space with no leading or trailing spaces. The word count of a sender is the total number of words sent by the sender. Note that a sender may send more than one message.
 * Return the sender with the largest word count. If there is more than one sender with the largest word count, return the one with the lexicographically largest name.
 * Note:
 *
 * 	Uppercase letters come before lowercase letters in lexicographical order.
 * 	"Alice" and "alice" are distinct.
 *
 *  
 * Example 1:
 *
 * Input: messages = ["Hello userTwooo","Hi userThree","Wonderful day Alice","Nice day userThree"], senders = ["Alice","userTwo","userThree","Alice"]
 * Output: "Alice"
 * Explanation: Alice sends a total of 2 + 3 = 5 words.
 * userTwo sends a total of 2 words.
 * userThree sends a total of 3 words.
 * Since Alice has the largest word count, we return "Alice".
 *
 * Example 2:
 *
 * Input: messages = ["How is leetcode for everyone","Leetcode is useful for practice"], senders = ["Bob","Charlie"]
 * Output: "Charlie"
 * Explanation: Bob sends a total of 5 words.
 * Charlie sends a total of 5 words.
 * Since there is a tie for the largest word count, we return the sender with the lexicographically larger name, Charlie.
 *  
 * Constraints:
 *
 * 	n == messages.length == senders.length
 * 	1 <= n <= 10^4
 * 	1 <= messages[i].length <= 100
 * 	1 <= senders[i].length <= 10
 * 	messages[i] consists of uppercase and lowercase English letters and ' '.
 * 	All the words in messages[i] are separated by a single space.
 * 	messages[i] does not have leading or trailing spaces.
 * 	senders[i] consists of uppercase and lowercase English letters only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sender-with-largest-word-count/
// discuss: https://leetcode.com/problems/sender-with-largest-word-count/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/sender-with-largest-word-count/solutions/5300813/two-line-solution-beats-88-at-speed-hash-gr14/
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        let generate_map = || {
            let mut map = std::collections::HashMap::new();
            map.insert(String::new(), 0);
            map
        };

        senders
            .into_iter()
            .zip(messages.into_iter())
            .fold(
                (String::new(), generate_map()),
                |(max_sender, mut map), (sender, message)| {
                    *map.entry(sender.clone()).or_insert(0) += message.split(" ").count() as i32;
                    let (cmp1, cmp2) = (
                        map[&sender] > map[&max_sender],
                        map[&sender] == map[&max_sender],
                    );
                    if cmp1 || cmp2 && sender > max_sender {
                        (sender, map)
                    } else {
                        (max_sender, map)
                    }
                },
            )
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2284_example_1() {
        let messages = vec_string![
            "Hello userTwooo",
            "Hi userThree",
            "Wonderful day Alice",
            "Nice day userThree"
        ];
        let senders = vec_string!["Alice", "userTwo", "userThree", "Alice"];

        let result = "Alice".to_string();

        assert_eq!(Solution::largest_word_count(messages, senders), result);
    }

    #[test]
    fn test_2284_example_2() {
        let messages = vec_string![
            "How is leetcode for everyone",
            "Leetcode is useful for practice"
        ];
        let senders = vec_string!["Bob", "Charlie"];

        let result = "Charlie".to_string();

        assert_eq!(Solution::largest_word_count(messages, senders), result);
    }
}
