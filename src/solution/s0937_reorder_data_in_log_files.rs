/**
 * [0937] Reorder Data in Log Files
 *
 * You are given an array of logs. Each log is a space-delimited string of words, where the first word is the identifier.
 * There are two types of logs:
 *
 * 	Letter-logs: All words (except the identifier) consist of lowercase English letters.
 * 	Digit-logs: All words (except the identifier) consist of digits.
 *
 * Reorder these logs so that:
 * <ol>
 * 	The letter-logs come before all digit-logs.
 * 	The letter-logs are sorted lexicographically by their contents. If their contents are the same, then sort them lexicographically by their identifiers.
 * 	The digit-logs maintain their relative std::cmp::ordering.
 * </ol>
 * Return the final order of the logs.
 *  
 * Example 1:
 *
 * Input: logs = ["dig1 8 1 5 1","let1 art can","dig2 3 6","let2 own kit dig","let3 art zero"]
 * Output: ["let1 art can","let3 art zero","let2 own kit dig","dig1 8 1 5 1","dig2 3 6"]
 * Explanation:
 * The letter-log contents are all different, so their ordering is "art can", "art zero", "own kit dig".
 * The digit-logs have a relative order of "dig1 8 1 5 1", "dig2 3 6".
 *
 * Example 2:
 *
 * Input: logs = ["a1 9 2 3 1","g1 act car","zo4 4 7","ab1 off key dog","a8 act zoo"]
 * Output: ["g1 act car","a8 act zoo","ab1 off key dog","a1 9 2 3 1","zo4 4 7"]
 *
 *  
 * Constraints:
 *
 * 	1 <= logs.length <= 100
 * 	3 <= logs[i].length <= 100
 * 	All the tokens of logs[i] are separated by a single space.
 * 	logs[i] is guaranteed to have an identifier and at least one word after the identifier.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reorder-data-in-log-files/
// discuss: https://leetcode.com/problems/reorder-data-in-log-files/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Clone, PartialEq, Eq, Ord)]
enum Log {
    Digit(String, String),
    Letter(String, String),
}

impl Log {
    pub fn is_dig(&self) -> bool {
        match &self {
            Log::Digit(_, _) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Log {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if other.is_dig() && !self.is_dig() {
            Some(std::cmp::Ordering::Less)
        } else if !other.is_dig() && self.is_dig() {
            Some(std::cmp::Ordering::Greater)
        } else if self.is_dig() && other.is_dig() {
            Some(std::cmp::Ordering::Equal)
        } else {
            let mut result = std::cmp::Ordering::Equal;
            let (id1, v1) = if let Log::Letter(id, v) = &self {
                (id, v)
            } else {
                return Some(result);
            };
            let (id2, v2) = if let Log::Letter(id, v) = &other {
                (id, v)
            } else {
                return Some(result);
            };
            result = v1.cmp(&v2);
            if result == std::cmp::Ordering::Equal {
                result = id1.cmp(&id2);
            }
            Some(result)
        }
    }
}

impl std::str::FromStr for Log {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tok: Vec<&str> = s.splitn(2, ' ').collect();
        if (tok[1].as_bytes()[0] as char).is_digit(10) {
            Ok(Log::Digit(tok[0].to_string(), tok[1].to_string()))
        } else {
            Ok(Log::Letter(tok[0].to_string(), tok[1].to_string()))
        }
    }
}

impl ToString for Log {
    fn to_string(&self) -> String {
        match &self {
            Log::Digit(a, b) => format!("{} {}", a, b),
            Log::Letter(a, b) => format!("{} {}", a, b),
        }
    }
}
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut r: Vec<Log> = logs
            .into_iter()
            .map(|x| x.parse::<Log>().unwrap())
            .collect();
        r.sort();
        r.iter().map(|x| x.to_string()).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0937_example_1() {
        let logs = vec_string![
            "dig1 8 1 5 1",
            "let1 art can",
            "dig2 3 6",
            "let2 own kit dig",
            "let3 art zero"
        ];
        let result = vec_string![
            "let1 art can",
            "let3 art zero",
            "let2 own kit dig",
            "dig1 8 1 5 1",
            "dig2 3 6"
        ];

        assert_eq!(Solution::reorder_log_files(logs), result);
    }

    #[test]
    fn test_0937_example_2() {
        let logs = vec_string![
            "a1 9 2 3 1",
            "g1 act car",
            "zo4 4 7",
            "ab1 off key dog",
            "a8 act zoo"
        ];
        let result = vec_string![
            "g1 act car",
            "a8 act zoo",
            "ab1 off key dog",
            "a1 9 2 3 1",
            "zo4 4 7"
        ];

        assert_eq!(Solution::reorder_log_files(logs), result);
    }
}
