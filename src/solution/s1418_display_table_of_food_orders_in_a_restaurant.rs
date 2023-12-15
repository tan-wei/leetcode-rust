/**
 * [1418] Display Table of Food Orders in a Restaurant
 *
 * Given the array orders, which represents the orders that customers have done in a restaurant. More specifically orders[i]=[customerNamei,tableNumberi,foodItemi] where customerNamei is the name of the customer, tableNumberi is the table customer sit at, and foodItemi is the item customer orders.
 *
 * Return the restaurant's &ldquo;display table&rdquo;. The &ldquo;display table&rdquo; is a table whose row entries denote how many of each food item each table ordered. The first column is the table number and the remaining columns correspond to each food item in alphabetical order. The first row should be a header whose first column is &ldquo;Table&rdquo;, followed by the names of the food items. Note that the customer names are not part of the table. Additionally, the rows should be sorted in numerically increasing order.
 *
 *  
 * Example 1:
 *
 *
 * Input: orders = [["David","3","Ceviche"],["Corina","10","Beef Burrito"],["David","3","Fried Chicken"],["Carla","5","Water"],["Carla","5","Ceviche"],["Rous","3","Ceviche"]]
 * Output: [["Table","Beef Burrito","Ceviche","Fried Chicken","Water"],["3","0","2","1","0"],["5","0","1","0","1"],["10","1","0","0","0"]]
 * Explanation:
 * The displaying table looks like:
 * Table,Beef Burrito,Ceviche,Fried Chicken,Water
 * 3    ,0           ,2      ,1            ,0
 * 5    ,0           ,1      ,0            ,1
 * 10   ,1           ,0      ,0            ,0
 * For the table 3: David orders "Ceviche" and "Fried Chicken", and Rous orders "Ceviche".
 * For the table 5: Carla orders "Water" and "Ceviche".
 * For the table 10: Corina orders "Beef Burrito".
 *
 *
 * Example 2:
 *
 *
 * Input: orders = [["James","12","Fried Chicken"],["Ratesh","12","Fried Chicken"],["Amadeus","12","Fried Chicken"],["Adam","1","Canadian Waffles"],["Brianna","1","Canadian Waffles"]]
 * Output: [["Table","Canadian Waffles","Fried Chicken"],["1","2","0"],["12","0","3"]]
 * Explanation:
 * For the table 1: Adam and Brianna order "Canadian Waffles".
 * For the table 12: James, Ratesh and Amadeus order "Fried Chicken".
 *
 *
 * Example 3:
 *
 *
 * Input: orders = [["Laura","2","Bean Burrito"],["Jhon","2","Beef Burrito"],["Melissa","2","Soda"]]
 * Output: [["Table","Bean Burrito","Beef Burrito","Soda"],["2","1","1","1"]]
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= orders.length <= 5 * 10^4
 * 	orders[i].length == 3
 * 	1 <= customerNamei.length, foodItemi.length <= 20
 * 	customerNamei and foodItemi consist of lowercase and uppercase English letters and the space character.
 * 	tableNumberi is a valid integer between 1 and 500.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/display-table-of-food-orders-in-a-restaurant/
// discuss: https://leetcode.com/problems/display-table-of-food-orders-in-a-restaurant/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::str::FromStr;

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut foods = std::collections::BTreeMap::new();
        let mut tables = std::collections::BTreeSet::new();

        for (table, food) in orders
            .into_iter()
            .map(|mut v| (i32::from_str(&v[1]).unwrap(), v.pop().unwrap()))
        {
            tables.insert(table);
            *foods
                .entry(food)
                .or_insert(std::collections::HashMap::new())
                .entry(table)
                .or_insert(0) += 1;
        }

        let mut result = Vec::with_capacity(tables.len() + 1);

        result.push(
            std::iter::once("Table".to_owned())
                .chain(foods.keys().cloned())
                .collect(),
        );

        result.extend(tables.into_iter().map(|t| {
            std::iter::once(t.to_string())
                .chain(foods.values().map(|f| f.get(&t).unwrap_or(&0).to_string()))
                .collect()
        }));

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1418_example_1() {
        let orders = vec![
            vec_string!["David", "3", "Ceviche"],
            vec_string!["Corina", "10", "Beef Burrito"],
            vec_string!["David", "3", "Fried Chicken"],
            vec_string!["Carla", "5", "Water"],
            vec_string!["Carla", "5", "Ceviche"],
            vec_string!["Rous", "3", "Ceviche"],
        ];

        let result = vec![
            vec_string!["Table", "Beef Burrito", "Ceviche", "Fried Chicken", "Water"],
            vec_string!["3", "0", "2", "1", "0"],
            vec_string!["5", "0", "1", "0", "1"],
            vec_string!["10", "1", "0", "0", "0"],
        ];

        assert_eq!(Solution::display_table(orders), result);
    }

    #[test]
    fn test_1418_example_2() {
        let orders = vec![
            vec_string!["James", "12", "Fried Chicken"],
            vec_string!["Ratesh", "12", "Fried Chicken"],
            vec_string!["Amadeus", "12", "Fried Chicken"],
            vec_string!["Adam", "1", "Canadian Waffles"],
            vec_string!["Brianna", "1", "Canadian Waffles"],
        ];

        let result = vec![
            vec_string!["Table", "Canadian Waffles", "Fried Chicken"],
            vec_string!["1", "2", "0"],
            vec_string!["12", "0", "3"],
        ];

        assert_eq!(Solution::display_table(orders), result);
    }

    #[test]
    fn test_1418_example_3() {
        let orders = vec![
            vec_string!["Laura", "2", "Bean Burrito"],
            vec_string!["Jhon", "2", "Beef Burrito"],
            vec_string!["Melissa", "2", "Soda"],
        ];

        let result = vec![
            vec_string!["Table", "Bean Burrito", "Beef Burrito", "Soda"],
            vec_string!["2", "1", "1", "1"],
        ];

        assert_eq!(Solution::display_table(orders), result);
    }
}
