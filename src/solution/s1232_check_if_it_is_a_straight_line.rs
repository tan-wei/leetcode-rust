/**
 * [1232] Check If It Is a Straight Line
 *
 * You are given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate of a point. Check if these points make a straight line in the XY plane.
 *  
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/15/untitled-diagram-2.jpg" style="width: 336px; height: 336px;" />
 *
 * Input: coordinates = [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/09/untitled-diagram-1.jpg" style="width: 348px; height: 336px;" />
 *
 * Input: coordinates = [[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	2 <= coordinates.length <= 1000
 * 	coordinates[i].length == 2
 * 	-10^4 <= coordinates[i][0], coordinates[i][1] <= 10^4
 * 	coordinates contains no duplicate point.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-it-is-a-straight-line/
// discuss: https://leetcode.com/problems/check-if-it-is-a-straight-line/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let (i, j) = (coordinates[0].clone(), coordinates[1].clone());

        coordinates.iter().all(|k| {
            i[0] * j[1] + j[0] * k[1] + k[0] * i[1] - j[0] * i[1] - k[0] * j[1] - i[0] * k[1] == 0
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1232_example_1() {
        let coordinates = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
        ];
        let result = true;

        assert_eq!(Solution::check_straight_line(coordinates), result);
    }

    #[test]
    fn test_1232_example_2() {
        let coordinates = vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7],
        ];
        let result = false;

        assert_eq!(Solution::check_straight_line(coordinates), result);
    }
}
