/**
 * [1140] Stone Game II
 *
 * Alice and Bob continue their games with piles of stones.  There are a number of piles arranged in a row, and each pile has a positive integer number of stones piles[i].  The objective of the game is to end with the most stones.
 * Alice and Bob take turns, with Alice starting first.  Initially, M = 1.
 * On each player's turn, that player can take all the stones in the first X remaining piles, where 1 <= X <= 2M.  Then, we set M = std::cmp::max(M, X).
 * The game continues until all the stones have been taken.
 * Assuming Alice and Bob play optimally, return the maximum number of stones Alice can get.
 *  
 * Example 1:
 *
 * Input: piles = [2,7,9,4,4]
 * Output: 10
 * Explanation:  If Alice takes one pile at the beginning, Bob takes two piles, then Alice takes 2 piles again. Alice can get 2 + 4 + 4 = 10 piles in total. If Alice takes two piles at the beginning, then Bob can take all three piles left. In this case, Alice get 2 + 7 = 9 piles in total. So we return 10 since it's larger.
 *
 * Example 2:
 *
 * Input: piles = [1,2,3,4,5,100]
 * Output: 104
 *
 *  
 * Constraints:
 *
 * 	1 <= piles.length <= 100
 * 	1 <= piles[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/stone-game-ii/
// discuss: https://leetcode.com/problems/stone-game-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut cache = vec![vec![-1; 2 * piles.len()]; piles.len()];

        Self::dfs_helper(0, 1, &mut cache, &piles)
    }

    fn dfs_helper(index: usize, m: usize, cache: &mut Vec<Vec<i32>>, piles: &Vec<i32>) -> i32 {
        if cache[index][m] != -1 {
            return cache[index][m];
        }

        let tot = match piles.len() - index <= 2 * m {
            true => return (index..piles.len()).fold(0, |acc, i| acc + piles[i]),
            false => (index..piles.len()).fold(0, |acc, i| acc + piles[i]),
        };

        cache[index][m] = tot
            - (1..std::cmp::min(2 * m + 1, piles.len() - index)).fold(tot, |acc, i| {
                std::cmp::min(
                    acc,
                    Self::dfs_helper(index + i, std::cmp::max(i, m), cache, piles),
                )
            });

        cache[index][m]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1140_example_1() {
        let piles = vec![2, 7, 9, 4, 4];
        let result = 10;

        assert_eq!(Solution::stone_game_ii(piles), result);
    }

    #[test]
    fn test_1140_example_2() {
        let piles = vec![1, 2, 3, 4, 5, 100];
        let result = 104;

        assert_eq!(Solution::stone_game_ii(piles), result);
    }
}
