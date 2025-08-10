/**
 * [2115] Find All Possible Recipes from Given Supplies
 *
 * You have information about n different recipes. You are given a string array recipes and a 2D string array ingredients. The i^th recipe has the name recipes[i], and you can create it if you have all the needed ingredients from ingredients[i]. A recipe can also be an ingredient for other recipes, i.e., ingredients[i] may contain a string that is in recipes.
 * You are also given a string array supplies containing all the ingredients that you initially have, and you have an infinite supply of all of them.
 * Return a list of all the recipes that you can create. You may return the answer in any order.
 * Note that two recipes may contain each other in their ingredients.
 *  
 * Example 1:
 *
 * Input: recipes = ["bread"], ingredients = [["yeast","flour"]], supplies = ["yeast","flour","corn"]
 * Output: ["bread"]
 * Explanation:
 * We can create "bread" since we have the ingredients "yeast" and "flour".
 *
 * Example 2:
 *
 * Input: recipes = ["bread","sandwich"], ingredients = [["yeast","flour"],["bread","meat"]], supplies = ["yeast","flour","meat"]
 * Output: ["bread","sandwich"]
 * Explanation:
 * We can create "bread" since we have the ingredients "yeast" and "flour".
 * We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".
 *
 * Example 3:
 *
 * Input: recipes = ["bread","sandwich","burger"], ingredients = [["yeast","flour"],["bread","meat"],["sandwich","meat","bread"]], supplies = ["yeast","flour","meat"]
 * Output: ["bread","sandwich","burger"]
 * Explanation:
 * We can create "bread" since we have the ingredients "yeast" and "flour".
 * We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".
 * We can create "burger" since we have the ingredient "meat" and can create the ingredients "bread" and "sandwich".
 *
 *  
 * Constraints:
 *
 * 	n == recipes.length == ingredients.length
 * 	1 <= n <= 100
 * 	1 <= ingredients[i].length, supplies.length <= 100
 * 	1 <= recipes[i].length, ingredients[i][j].length, supplies[k].length <= 10
 * 	recipes[i], ingredients[i][j], and supplies[k] consist only of lowercase English letters.
 * 	All the values of recipes and supplies combined are unique.
 * 	Each ingredients[i] does not contain any duplicate values.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/
// discuss: https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2115_example_1() {
        let recipes = vec_string!["bread"];
        let ingredients = vec![vec_string!["yeast", "flour"]];
        let supplies = vec_string!["yeast", "flour", "corn"];

        let result = vec_string!["bread"];

        assert_eq!(
            Solution::find_all_recipes(recipes, ingredients, supplies),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2115_example_2() {
        let recipes = vec_string!["bread", "sandwich"];
        let ingredients = vec![vec_string!["yeast", "flour"], vec_string!["bread", "meat"]];
        let supplies = vec_string!["yeast", "flour", "meat"];

        let result = vec_string!["bread"];

        assert_eq!(
            Solution::find_all_recipes(recipes, ingredients, supplies),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2115_example_3() {
        let recipes = vec_string!["bread", "sandwich", "burger"];
        let ingredients = vec![
            vec_string!["yeast", "flour"],
            vec_string!["bread", "meat"],
            vec_string!["sandwich", "meat", "bread"],
        ];
        let supplies = vec_string!["yeast", "flour", "meat"];

        let result = vec_string!["bread", "sandwich", "burger"];

        assert_eq!(
            Solution::find_all_recipes(recipes, ingredients, supplies),
            result
        );
    }
}
