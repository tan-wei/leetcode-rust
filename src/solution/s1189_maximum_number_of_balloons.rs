/**
 * [1189] Maximum Number of Balloons
 *
 * Given a string text, you want to use the characters of text to form as many instances of the word "balloon" as possible.
 * You can use each character in text at most once. Return the maximum number of instances that can be formed.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/05/1536_ex1_upd.JPG" style="width: 132px; height: 35px;" />
 *
 * Input: text = "nlaebolko"
 * Output: 1
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/05/1536_ex2_upd.JPG" style="width: 267px; height: 35px;" />
 *
 * Input: text = "loonbalxballpoon"
 * Output: 2
 *
 * Example 3:
 *
 * Input: text = "leetcode"
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= text.length <= 10^4
 * 	text consists of lower case English letters only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-balloons/
// discuss: https://leetcode.com/problems/maximum-number-of-balloons/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut hm = std::collections::HashMap::new();

        text.bytes().into_iter().for_each(|x| {
            let elem = hm.entry(x).or_insert(0);
            *elem += 1;
        });

        let ban = String::from("ban")
            .bytes()
            .into_iter()
            .map(|k| hm.get(&k).unwrap_or(&0))
            .min();

        let ol = String::from("ol")
            .bytes()
            .into_iter()
            .map(|k| hm.get(&k).unwrap_or(&0))
            .min();

        if let Some(&b) = ban {
            if let Some(&o) = ol {
                return b.min(o / 2);
            }
        }

        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1189_example_1() {
        let text = "nlaebolko".to_string();
        let result = 1;

        assert_eq!(Solution::max_number_of_balloons(text), result);
    }

    #[test]
    fn test_1189_example_2() {
        let text = "loonbalxballpoon".to_string();
        let result = 2;

        assert_eq!(Solution::max_number_of_balloons(text), result);
    }

    #[test]
    fn test_1189_example_3() {
        let text = "leetcode".to_string();
        let result = 0;

        assert_eq!(Solution::max_number_of_balloons(text), result);
    }
}
