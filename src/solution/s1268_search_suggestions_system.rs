/**
 * [1268] Search Suggestions System
 *
 * You are given an array of strings products and a string searchWord.
 * Design a system that suggests at most three product names from products after each character of searchWord is typed. Suggested products should have common prefix with searchWord. If there are more than three products with a common prefix return the three lexicographically minimums products.
 * Return a list of lists of the suggested products after each character of searchWord is typed.
 *  
 * Example 1:
 *
 * Input: products = ["mobile","mouse","moneypot","monitor","mousepad"], searchWord = "mouse"
 * Output: [["mobile","moneypot","monitor"],["mobile","moneypot","monitor"],["mouse","mousepad"],["mouse","mousepad"],["mouse","mousepad"]]
 * Explanation: products sorted lexicographically = ["mobile","moneypot","monitor","mouse","mousepad"].
 * After typing m and mo all products match and we show user ["mobile","moneypot","monitor"].
 * After typing mou, mous and mouse the system suggests ["mouse","mousepad"].
 *
 * Example 2:
 *
 * Input: products = ["havana"], searchWord = "havana"
 * Output: [["havana"],["havana"],["havana"],["havana"],["havana"],["havana"]]
 * Explanation: The only word "havana" will be always suggested while typing the search word.
 *
 *  
 * Constraints:
 *
 * 	1 <= products.length <= 1000
 * 	1 <= products[i].length <= 3000
 * 	1 <= sum(products[i].length) <= 2 * 10^4
 * 	All the strings of products are unique.
 * 	products[i] consists of lowercase English letters.
 * 	1 <= searchWord.length <= 1000
 * 	searchWord consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/search-suggestions-system/
// discuss: https://leetcode.com/problems/search-suggestions-system/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut products = products;
        products.sort();
        let search_word = search_word.as_bytes();
        let mut result = vec![Vec::new(); search_word.len()];
        for product in &products {
            let p = product.as_bytes();
            let mut i = 0;
            while i < p.len().min(search_word.len()) && p[i] == search_word[i] {
                if result[i].len() < 3 {
                    result[i].push(product.clone());
                }
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
    fn test_1268_example_1() {
        let products = vec_string!["mobile", "mouse", "moneypot", "monitor", "mousepad"];
        let search_word = "mouse".to_string();

        let result = vec![
            vec_string!["mobile", "moneypot", "monitor"],
            vec_string!["mobile", "moneypot", "monitor"],
            vec_string!["mouse", "mousepad"],
            vec_string!["mouse", "mousepad"],
            vec_string!["mouse", "mousepad"],
        ];

        assert_eq!(Solution::suggested_products(products, search_word), result);
    }

    #[test]
    fn test_1268_example_2() {
        let products = vec_string!["havana"];
        let search_word = "havana".to_string();

        let result = vec![
            vec_string!["havana"],
            vec_string!["havana"],
            vec_string!["havana"],
            vec_string!["havana"],
            vec_string!["havana"],
            vec_string!["havana"],
        ];

        assert_eq!(Solution::suggested_products(products, search_word), result);
    }
}
