/**
 * [1773] Count Items Matching a Rule
 *
 * You are given an array items, where each items[i] = [typei, colori, namei] describes the type, color, and name of the i^th item. You are also given a rule represented by two strings, ruleKey and ruleValue.
 * The i^th item is said to match the rule if one of the following is true:
 *
 * 	ruleKey == "type" and ruleValue == typei.
 * 	ruleKey == "color" and ruleValue == colori.
 * 	ruleKey == "name" and ruleValue == namei.
 *
 * Return the number of items that match the given rule.
 *  
 * Example 1:
 *
 * Input: items = [["phone","blue","pixel"],["computer","silver","lenovo"],["phone","gold","iphone"]], ruleKey = "color", ruleValue = "silver"
 * Output: 1
 * Explanation: There is only one item matching the given rule, which is ["computer","silver","lenovo"].
 *
 * Example 2:
 *
 * Input: items = [["phone","blue","pixel"],["computer","silver","phone"],["phone","gold","iphone"]], ruleKey = "type", ruleValue = "phone"
 * Output: 2
 * Explanation: There are only two items matching the given rule, which are ["phone","blue","pixel"] and ["phone","gold","iphone"]. Note that the item ["computer","silver","phone"] does not match.
 *  
 * Constraints:
 *
 * 	1 <= items.length <= 10^4
 * 	1 <= typei.length, colori.length, namei.length, ruleValue.length <= 10
 * 	ruleKey is equal to either "type", "color", or "name".
 * 	All strings consist only of lowercase letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-items-matching-a-rule/
// discuss: https://leetcode.com/problems/count-items-matching-a-rule/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let key_index = match &rule_key[..] {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => return 0,
        };

        items
            .iter()
            .filter(|item| item[key_index] == rule_value)
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1773_example_1() {
        let items = vec![
            vec_string!["phone", "blue", "pixel"],
            vec_string!["computer", "silver", "lenovo"],
            vec_string!["phone", "gold", "iphone"],
        ];
        let rule_key = "color".to_string();
        let rule_value = "silver".to_string();

        let result = 1;

        assert_eq!(Solution::count_matches(items, rule_key, rule_value), result);
    }

    #[test]
    fn test_1773_example_2() {
        let items = vec![
            vec_string!["phone", "blue", "pixel"],
            vec_string!["computer", "silver", "phone"],
            vec_string!["phone", "gold", "iphone"],
        ];
        let rule_key = "type".to_string();
        let rule_value = "phone".to_string();

        let result = 2;

        assert_eq!(Solution::count_matches(items, rule_key, rule_value), result);
    }
}
