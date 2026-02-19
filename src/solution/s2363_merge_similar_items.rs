/**
 * [2363] Merge Similar Items
 *
 * You are given two 2D integer arrays, items1 and items2, representing two sets of items. Each array items has the following properties:
 *
 * 	items[i] = [valuei, weighti] where valuei represents the value and weighti represents the weight of the i^th item.
 * 	The value of each item in items is unique.
 *
 * Return a 2D integer array ret where ret[i] = [valuei, weighti], with weighti being the sum of weights of all items with value valuei.
 * Note: ret should be returned in ascending order by value.
 *  
 * Example 1:
 *
 * Input: items1 = [[1,1],[4,5],[3,8]], items2 = [[3,1],[1,5]]
 * Output: [[1,6],[3,9],[4,5]]
 * Explanation:
 * The item with value = 1 occurs in items1 with weight = 1 and in items2 with weight = 5, total weight = 1 + 5 = 6.
 * The item with value = 3 occurs in items1 with weight = 8 and in items2 with weight = 1, total weight = 8 + 1 = 9.
 * The item with value = 4 occurs in items1 with weight = 5, total weight = 5.  
 * Therefore, we return [[1,6],[3,9],[4,5]].
 *
 * Example 2:
 *
 * Input: items1 = [[1,1],[3,2],[2,3]], items2 = [[2,1],[3,2],[1,3]]
 * Output: [[1,4],[2,4],[3,4]]
 * Explanation:
 * The item with value = 1 occurs in items1 with weight = 1 and in items2 with weight = 3, total weight = 1 + 3 = 4.
 * The item with value = 2 occurs in items1 with weight = 3 and in items2 with weight = 1, total weight = 3 + 1 = 4.
 * The item with value = 3 occurs in items1 with weight = 2 and in items2 with weight = 2, total weight = 2 + 2 = 4.
 * Therefore, we return [[1,4],[2,4],[3,4]].
 * Example 3:
 *
 * Input: items1 = [[1,3],[2,2]], items2 = [[7,1],[2,2],[1,4]]
 * Output: [[1,7],[2,4],[7,1]]
 * Explanation:
 * The item with value = 1 occurs in items1 with weight = 3 and in items2 with weight = 4, total weight = 3 + 4 = 7.
 * The item with value = 2 occurs in items1 with weight = 2 and in items2 with weight = 2, total weight = 2 + 2 = 4.
 * The item with value = 7 occurs in items2 with weight = 1, total weight = 1.
 * Therefore, we return [[1,7],[2,4],[7,1]].
 *
 *  
 * Constraints:
 *
 * 	1 <= items1.length, items2.length <= 1000
 * 	items1[i].length == items2[i].length == 2
 * 	1 <= valuei, weighti <= 1000
 * 	Each valuei in items1 is unique.
 * 	Each valuei in items2 is unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-similar-items/
// discuss: https://leetcode.com/problems/merge-similar-items/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = items1
            .into_iter()
            .chain(items2.into_iter())
            .fold(std::collections::HashMap::new(), |mut m, kv| {
                let k = kv[0];
                let v = kv[1];
                *m.entry(k).or_default() += v;
                m
            })
            .into_iter()
            .map(|x| vec![x.0, x.1])
            .collect();
        result.sort_unstable_by_key(|x| x[0]);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2363_example_1() {
        let items1 = vec![vec![1, 1], vec![4, 5], vec![3, 8]];
        let items2 = vec![vec![3, 1], vec![1, 5]];

        let result = vec![vec![1, 6], vec![3, 9], vec![4, 5]];

        assert_eq!(Solution::merge_similar_items(items1, items2), result);
    }

    #[test]
    fn test_2363_example_2() {
        let items1 = vec![vec![1, 1], vec![3, 2], vec![2, 3]];
        let items2 = vec![vec![2, 1], vec![3, 2], vec![1, 3]];

        let result = vec![vec![1, 4], vec![2, 4], vec![3, 4]];

        assert_eq!(Solution::merge_similar_items(items1, items2), result);
    }

    #[test]
    fn test_2363_example_3() {
        let items1 = vec![vec![1, 3], vec![2, 2]];
        let items2 = vec![vec![7, 1], vec![2, 2], vec![1, 4]];

        let result = vec![vec![1, 7], vec![2, 4], vec![7, 1]];

        assert_eq!(Solution::merge_similar_items(items1, items2), result);
    }
}
