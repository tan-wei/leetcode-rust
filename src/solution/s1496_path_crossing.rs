/**
 * [1496] Path Crossing
 *
 * Given a string path, where path[i] = 'N', 'S', 'E' or 'W', each representing moving one unit north, south, east, or west, respectively. You start at the origin (0, 0) on a 2D plane and walk on the path specified by path.
 * Return true if the path crosses itself at any point, that is, if at any time you are on a location you have previously visited. Return false otherwise.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/10/screen-shot-2020-06-10-at-123929-pm.png" style="width: 400px; height: 358px;" />
 * Input: path = "NES"
 * Output: false
 * Explanation: Notice that the path doesn't cross any point more than once.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/10/screen-shot-2020-06-10-at-123843-pm.png" style="width: 400px; height: 339px;" />
 * Input: path = "NESWW"
 * Output: true
 * Explanation: Notice that the path visits the origin twice.
 *  
 * Constraints:
 *
 * 	1 <= path.length <= 10^4
 * 	path[i] is either 'N', 'S', 'E', or 'W'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/path-crossing/
// discuss: https://leetcode.com/problems/path-crossing/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        path.chars()
            .fold(
                (
                    std::collections::HashMap::<(i32, i32), usize>::from([((0, 0), 1)]),
                    (0, 0),
                ),
                |(mut hm, (mut x, mut y)), c| {
                    match c {
                        'N' => y += 1,
                        'S' => y -= 1,
                        'E' => x += 1,
                        'W' => x -= 1,
                        _ => unreachable!(),
                    };
                    *hm.entry((x, y)).or_insert(0) += 1;
                    (hm, (x, y))
                },
            )
            .0
            .values()
            .any(|x| x > &1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1496_example_1() {
        let path = "NES".to_string();

        let result = false;

        assert_eq!(Solution::is_path_crossing(path), result);
    }

    #[test]
    fn test_1496_example_2() {
        let path = "NESWW".to_string();

        let result = true;

        assert_eq!(Solution::is_path_crossing(path), result);
    }
}
