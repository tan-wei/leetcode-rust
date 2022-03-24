/**
 * [0593] Valid Square
 *
 * Given the coordinates of four points in 2D space p1, p2, p3 and p4, return true if the four points construct a square.
 * The coordinate of a point pi is represented as [xi, yi]. The input is not given in any order.
 * A valid square has four equal sides with positive length and four equal angles (90-degree angles).
 *  
 * Example 1:
 *
 * Input: p1 = [0,0], p2 = [1,1], p3 = [1,0], p4 = [0,1]
 * Output: true
 *
 * Example 2:
 *
 * Input: p1 = [0,0], p2 = [1,1], p3 = [1,0], p4 = [0,12]
 * Output: false
 *
 * Example 3:
 *
 * Input: p1 = [1,0], p2 = [-1,0], p3 = [0,1], p4 = [0,-1]
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	p1.length == p2.length == p3.length == p4.length == 2
 * 	-10^4 <= xi, yi <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-square/
// discuss: https://leetcode.com/problems/valid-square/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut vx = vec![p1, p2, p3, p4];
        vx.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        let dist = |i: usize, j: usize| -> i32 {
            (vx[i][0] - vx[j][0]).pow(2) + (vx[i][1] - vx[j][1]).pow(2)
        };

        dist(0, 1) != 0
            && dist(0, 1) == dist(1, 3)
            && dist(1, 3) == dist(3, 2)
            && dist(3, 2) == dist(2, 0)
            && dist(0, 3) == dist(1, 2)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0593_example_1() {
        let p1 = vec![0, 0];
        let p2 = vec![1, 1];
        let p3 = vec![1, 0];
        let p4 = vec![0, 1];
        let result = true;

        assert_eq!(Solution::valid_square(p1, p2, p3, p4), result);
    }

    #[test]
    fn test_0593_example_2() {
        let p1 = vec![0, 0];
        let p2 = vec![1, 1];
        let p3 = vec![1, 0];
        let p4 = vec![0, 12];
        let result = false;

        assert_eq!(Solution::valid_square(p1, p2, p3, p4), result);
    }

    #[test]
    fn test_0593_example_3() {
        let p1 = vec![1, 0];
        let p2 = vec![-1, 0];
        let p3 = vec![0, 1];
        let p4 = vec![0, -1];
        let result = true;

        assert_eq!(Solution::valid_square(p1, p2, p3, p4), result);
    }
}
