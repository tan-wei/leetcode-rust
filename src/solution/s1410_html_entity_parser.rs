/**
 * [1410] HTML Entity Parser
 *
 * HTML entity parser is the parser that takes HTML code as input and replace all the entities of the special characters by the characters itself.
 * The special characters and their entities for HTML are:
 *
 * 	Quotation Mark: the entity is &amp;quot; and symbol character is ".
 * 	Single Quote Mark: the entity is &amp;apos; and symbol character is '.
 * 	Ampersand: the entity is &amp;amp; and symbol character is &amp;.
 * 	Greater Than Sign: the entity is &amp;gt; and symbol character is >.
 * 	Less Than Sign: the entity is &amp;lt; and symbol character is <.
 * 	Slash: the entity is &amp;frasl; and symbol character is /.
 *
 * Given the input text string to the HTML parser, you have to implement the entity parser.
 * Return the text after replacing the entities by the special characters.
 *  
 * Example 1:
 *
 * Input: text = "&amp;amp; is an HTML entity but &amp;ambassador; is not."
 * Output: "&amp; is an HTML entity but &amp;ambassador; is not."
 * Explanation: The parser will replace the &amp;amp; entity by &amp;
 *
 * Example 2:
 *
 * Input: text = "and I quote: &amp;quot;...&amp;quot;"
 * Output: "and I quote: \"...\""
 *
 *  
 * Constraints:
 *
 * 	1 <= text.length <= 10^5
 * 	The string may contain any possible characters out of all the 256 ASCII characters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/html-entity-parser/
// discuss: https://leetcode.com/problems/html-entity-parser/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn entity_parser(text: String) -> String {
        let mut result = String::new();
        let mut i = 0;
        while i < text.len() {
            if text[i..].starts_with("&quot;") {
                result.push('"');
                i += 6;
            } else if text[i..].starts_with("&apos;") {
                result.push('\'');
                i += 6;
            } else if text[i..].starts_with("&amp;") {
                result.push('&');
                i += 5;
            } else if text[i..].starts_with("&gt;") {
                result.push('>');
                i += 4;
            } else if text[i..].starts_with("&lt;") {
                result.push('<');
                i += 4;
            } else if text[i..].starts_with("&frasl;") {
                result.push('/');
                i += 7;
            } else {
                result.push(text[i..].chars().next().unwrap());
                i += 1;
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1410_example_1() {
        let text = "&amp; is an HTML entity but &ambassador; is not.".to_string();

        let result = "& is an HTML entity but &ambassador; is not.".to_string();

        assert_eq!(Solution::entity_parser(text), result);
    }

    #[test]
    fn test_1410_example_2() {
        let text = "and I quote: &quot;...&quot;".to_string();

        let result = "and I quote: \"...\"".to_string();

        assert_eq!(Solution::entity_parser(text), result);
    }
}
