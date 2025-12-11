/**
 * [2271] Maximum White Tiles Covered by a Carpet
 *
 * You are given a 2D integer array tiles where tiles[i] = [li, ri] represents that every tile j in the range li <= j <= ri is colored white.
 * You are also given an integer carpetLen, the length of a single carpet that can be placed anywhere.
 * Return the maximum number of white tiles that can be covered by the carpet.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/25/example1drawio3.png" style="width: 644px; height: 158px;" />
 * Input: tiles = [[1,5],[10,11],[12,18],[20,25],[30,32]], carpetLen = 10
 * Output: 9
 * Explanation: Place the carpet starting on tile 10.
 * It covers 9 white tiles, so we return 9.
 * Note that there may be other places where the carpet covers 9 white tiles.
 * It can be shown that the carpet cannot cover more than 9 white tiles.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/24/example2drawio.png" style="width: 231px; height: 168px;" />
 * Input: tiles = [[10,11],[1,1]], carpetLen = 2
 * Output: 2
 * Explanation: Place the carpet starting on tile 10.
 * It covers 2 white tiles, so we return 2.
 *
 *  
 * Constraints:
 *
 * 	1 <= tiles.length <= 5 * 10^4
 * 	tiles[i].length == 2
 * 	1 <= li <= ri <= 10^9
 * 	1 <= carpetLen <= 10^9
 * 	The tiles are non-overlapping.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-white-tiles-covered-by-a-carpet/
// discuss: https://leetcode.com/problems/maximum-white-tiles-covered-by-a-carpet/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2271_example_1() {
        let tiles = vec![
            vec![1, 5],
            vec![10, 11],
            vec![12, 18],
            vec![20, 25],
            vec![30, 32],
        ];
        let carpet_len = 10;

        let result = 9;

        assert_eq!(Solution::maximum_white_tiles(tiles, carpet_len), result);
    }

    #[test]
    #[ignore]
    fn test_2271_example_2() {
        let tiles = vec![vec![10, 11], vec![1, 1]];
        let carpet_len = 10;

        let result = 2;

        assert_eq!(Solution::maximum_white_tiles(tiles, carpet_len), result);
    }
}
