/**
 * [1049] Last Stone Weight II
 *
 * You are given an array of integers stones where stones[i] is the weight of the i^th stone.
 * We are playing a game with the stones. On each turn, we choose any two stones and smash them together. Suppose the stones have weights x and y with x <= y. The result of this smash is:
 *
 * 	If x == y, both stones are destroyed, and
 * 	If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
 *
 * At the end of the game, there is at most one stone left.
 * Return the smallest possible weight of the left stone. If there are no stones left, return 0.
 *  
 * Example 1:
 *
 * Input: stones = [2,7,4,1,8,1]
 * Output: 1
 * Explanation:
 * We can combine 2 and 4 to get 2, so the array converts to [2,7,1,8,1] then,
 * we can combine 7 and 8 to get 1, so the array converts to [2,1,1,1] then,
 * we can combine 2 and 1 to get 1, so the array converts to [1,1,1] then,
 * we can combine 1 and 1 to get 0, so the array converts to [1], then that's the optimal value.
 *
 * Example 2:
 *
 * Input: stones = [31,26,33,21,40]
 * Output: 5
 *
 *  
 * Constraints:
 *
 * 	1 <= stones.length <= 30
 * 	1 <= stones[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/last-stone-weight-ii/
// discuss: https://leetcode.com/problems/last-stone-weight-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        let n = stones.len();
        set.insert(stones[0]);
        set.insert(-1 * stones[0]);

        for i in 1..n {
            let mut new_set = std::collections::HashSet::new();
            let v1 = stones[i];
            for v2 in set {
                new_set.insert(v1 + v2);
                new_set.insert(v1 - v2);
            }
            set = new_set;
        }

        let mut result = 10000;
        for v in set {
            result = std::cmp::min(v.abs(), result);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1049_example_1() {
        let stones = vec![2, 7, 4, 1, 8, 1];
        let result = 1;

        assert_eq!(Solution::last_stone_weight_ii(stones), result);
    }

    #[test]
    fn test_1049_example_2() {
        let stones = vec![31, 26, 33, 21, 40];
        let result = 5;

        assert_eq!(Solution::last_stone_weight_ii(stones), result);
    }
}
