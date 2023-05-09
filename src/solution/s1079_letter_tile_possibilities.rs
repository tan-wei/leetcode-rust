/**
 * [1079] Letter Tile Possibilities
 *
 * You have n  tiles, where each tile has one letter tiles[i] printed on it.
 * Return the number of possible non-empty sequences of letters you can make using the letters printed on those tiles.
 *  
 * Example 1:
 *
 * Input: tiles = "AAB"
 * Output: 8
 * Explanation: The possible sequences are "A", "B", "AA", "AB", "BA", "AAB", "ABA", "BAA".
 *
 * Example 2:
 *
 * Input: tiles = "AAABBC"
 * Output: 188
 *
 * Example 3:
 *
 * Input: tiles = "V"
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= tiles.length <= 7
 * 	tiles consists of uppercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/letter-tile-possibilities/
// discuss: https://leetcode.com/problems/letter-tile-possibilities/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut dict = vec![0; 26];
        for c in tiles.chars() {
            dict[c as usize - 'A' as usize] += 1;
        }
        let mut count = 0;
        Solution::dfs_helper(&mut dict, &mut count);
        count
    }

    pub fn dfs_helper(dict: &mut Vec<i32>, count: &mut i32) {
        for i in 0..dict.len() {
            if dict[i] > 0 {
                *count += 1;
                dict[i] -= 1;
                Solution::dfs_helper(dict, count);
                dict[i] += 1;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1079_example_1() {
        let tiles = "AAB".to_string();
        let result = 8;

        assert_eq!(Solution::num_tile_possibilities(tiles), result);
    }

    #[test]
    fn test_1079_example_2() {
        let tiles = "AAABBC".to_string();
        let result = 188;

        assert_eq!(Solution::num_tile_possibilities(tiles), result);
    }

    #[test]
    fn test_1079_example_3() {
        let tiles = "V".to_string();
        let result = 1;

        assert_eq!(Solution::num_tile_possibilities(tiles), result);
    }
}
