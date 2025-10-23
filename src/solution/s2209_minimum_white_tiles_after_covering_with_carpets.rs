/**
 * [2209] Minimum White Tiles After Covering With Carpets
 *
 * You are given a 0-indexed binary string floor, which represents the colors of tiles on a floor:
 *
 * 	floor[i] = '0' denotes that the i^th tile of the floor is colored black.
 * 	On the other hand, floor[i] = '1' denotes that the i^th tile of the floor is colored white.
 *
 * You are also given numCarpets and carpetLen. You have numCarpets black carpets, each of length carpetLen tiles. Cover the tiles with the given carpets such that the number of white tiles still visible is minimum. Carpets may overlap one another.
 * Return the minimum number of white tiles still visible.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/02/10/ex1-1.png" style="width: 400px; height: 73px;" />
 * Input: floor = "10110101", numCarpets = 2, carpetLen = 2
 * Output: 2
 * Explanation:
 * The figure above shows one way of covering the tiles with the carpets such that only 2 white tiles are visible.
 * No other way of covering the tiles with the carpets can leave less than 2 white tiles visible.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/02/10/ex2.png" style="width: 353px; height: 123px;" />
 * Input: floor = "11111", numCarpets = 2, carpetLen = 3
 * Output: 0
 * Explanation:
 * The figure above shows one way of covering the tiles with the carpets such that no white tiles are visible.
 * Note that the carpets are able to overlap one another.
 *
 *  
 * Constraints:
 *
 * 	1 <= carpetLen <= floor.length <= 1000
 * 	floor[i] is either '0' or '1'.
 * 	1 <= numCarpets <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-white-tiles-after-covering-with-carpets/
// discuss: https://leetcode.com/problems/minimum-white-tiles-after-covering-with-carpets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2209_example_1() {
        let floor = "10110101".to_string();
        let num_carpets = 2;
        let carpet_len = 2;

        let result = 2;

        assert_eq!(
            Solution::minimum_white_tiles(floor, num_carpets, carpet_len),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2209_example_2() {
        let floor = "11111".to_string();
        let num_carpets = 2;
        let carpet_len = 3;

        let result = 0;

        assert_eq!(
            Solution::minimum_white_tiles(floor, num_carpets, carpet_len),
            result
        );
    }
}
