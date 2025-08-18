/**
 * [2125] Number of Laser Beams in a Bank
 *
 * Anti-theft security devices are activated inside a bank. You are given a 0-indexed binary string array bank representing the floor plan of the bank, which is an m x n 2D matrix. bank[i] represents the i^th row, consisting of '0's and '1's. '0' means the cell is empty, while'1' means the cell has a security device.
 * There is one laser beam between any two security devices if both conditions are met:
 *
 * 	The two devices are located on two different rows: r1 and r2, where r1 < r2.
 * 	For each row i where r1 < i < r2, there are no security devices in the i^th row.
 *
 * Laser beams are independent, i.e., one beam does not interfere nor join with another.
 * Return the total number of laser beams in the bank.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/24/laser1.jpg" style="width: 400px; height: 368px;" />
 * Input: bank = ["011001","000000","010100","001000"]
 * Output: 8
 * Explanation: Between each of the following device pairs, there is one beam. In total, there are 8 beams:
 *  * bank[0][1] -- bank[2][1]
 *  * bank[0][1] -- bank[2][3]
 *  * bank[0][2] -- bank[2][1]
 *  * bank[0][2] -- bank[2][3]
 *  * bank[0][5] -- bank[2][1]
 *  * bank[0][5] -- bank[2][3]
 *  * bank[2][1] -- bank[3][2]
 *  * bank[2][3] -- bank[3][2]
 * Note that there is no beam between any device on the 0^th row with any on the 3^rd row.
 * This is because the 2^nd row contains security devices, which breaks the second condition.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/24/laser2.jpg" style="width: 244px; height: 325px;" />
 * Input: bank = ["000","111","000"]
 * Output: 0
 * Explanation: There does not exist two devices located on two different rows.
 *
 *  
 * Constraints:
 *
 * 	m == bank.length
 * 	n == bank[i].length
 * 	1 <= m, n <= 500
 * 	bank[i][j] is either '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-laser-beams-in-a-bank/
// discuss: https://leetcode.com/problems/number-of-laser-beams-in-a-bank/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.iter()
            .map(|s| s.chars().filter(|&c| c == '1').count())
            .filter(|&i| i > 0)
            .collect::<Vec<_>>()
            .windows(2)
            .map(|x| x[0] * x[1])
            .sum::<usize>() as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2125_example_1() {
        let bank = vec_string!["011001", "000000", "010100", "001000"];

        let result = 8;

        assert_eq!(Solution::number_of_beams(bank), result);
    }

    #[test]
    fn test_2125_example_2() {
        let bank = vec_string!["000", "111", "000"];

        let result = 0;

        assert_eq!(Solution::number_of_beams(bank), result);
    }
}
