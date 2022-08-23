/**
 * [0799] Champagne Tower
 *
 * We stack glasses in a pyramid, where the first row has 1 glass, the second row has 2 glasses, and so on until the 100^th row.  Each glass holds one cup of champagne.
 *
 * Then, some champagne is poured into the first glass at the top.  When the topmost glass is full, any excess liquid poured will fall equally to the glass immediately to the left and right of it.  When those glasses become full, any excess champagne will fall equally to the left and right of those glasses, and so on.  (A glass at the bottom row has its excess champagne fall on the floor.)
 *
 * For example, after one cup of champagne is poured, the top most glass is full.  After two cups of champagne are poured, the two glasses on the second row are half full.  After three cups of champagne are poured, those two cups become full - there are 3 full glasses total now.  After four cups of champagne are poured, the third row has the middle glass half full, and the two outside glasses are a quarter full, as pictured below.
 *
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/03/09/tower.png" style="height: 241px; width: 350px;" />
 *
 * Now after pouring some non-negative integer cups of champagne, return how full the j^th glass in the i^th row is (both i and j are 0-indexed.)
 *
 *  
 * Example 1:
 *
 *
 * Input: poured = 1, query_row = 1, query_glass = 1
 * Output: 0.00000
 * Explanation: We poured 1 cup of champange to the top glass of the tower (which is indexed as (0, 0)). There will be no excess liquid so all the glasses under the top glass will remain empty.
 *
 *
 * Example 2:
 *
 *
 * Input: poured = 2, query_row = 1, query_glass = 1
 * Output: 0.50000
 * Explanation: We poured 2 cups of champange to the top glass of the tower (which is indexed as (0, 0)). There is one cup of excess liquid. The glass indexed as (1, 0) and the glass indexed as (1, 1) will share the excess liquid equally, and each will get half cup of champange.
 *
 *
 * Example 3:
 *
 *
 * Input: poured = 100000009, query_row = 33, query_glass = 17
 * Output: 1.00000
 *
 *
 *  
 * Constraints:
 *
 *
 * 	0 <= poured <= 10^9
 * 	0 <= query_glass <= query_row < 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/champagne-tower/
// discuss: https://leetcode.com/problems/champagne-tower/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut volumes = vec![0.0; 100];
        volumes[0] = poured as f64;
        for i in 1..=query_row as usize {
            let mut carry = 0.0;
            for volume in volumes.iter_mut().take(i + 1) {
                let query = ((*volume - 1.0) / 2.0).max(0.0);
                *volume = query + carry;
                carry = query;
            }
        }
        volumes[query_glass as usize].min(1.0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0799_example_1() {
        let poured = 1;
        let query_row = 1;
        let query_glass = 1;
        let result = 0.00000;

        assert_f64_near!(
            Solution::champagne_tower(poured, query_row, query_glass),
            result
        );
    }

    #[test]
    fn test_0799_example_2() {
        let poured = 2;
        let query_row = 1;
        let query_glass = 1;
        let result = 0.50000;

        assert_f64_near!(
            Solution::champagne_tower(poured, query_row, query_glass),
            result
        );
    }

    #[test]
    fn test_0799_example_3() {
        let poured = 100000009;
        let query_row = 33;
        let query_glass = 17;
        let result = 1.00000;

        assert_f64_near!(
            Solution::champagne_tower(poured, query_row, query_glass),
            result
        );
    }
}
