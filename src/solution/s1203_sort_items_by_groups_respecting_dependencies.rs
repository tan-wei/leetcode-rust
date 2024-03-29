/**
 * [1203] Sort Items by Groups Respecting Dependencies
 *
 * There are n items each belonging to zero or one of m groups where group[i] is the group that the i-th item belongs to and it's equal to -1 if the i-th item belongs to no group. The items and the groups are zero indexed. A group can have no item belonging to it.
 * Return a sorted list of the items such that:
 *
 * 	The items that belong to the same group are next to each other in the sorted list.
 * 	There are some relations between these items where beforeItems[i] is a list containing all the items that should come before the i-th item in the sorted array (to the left of the i-th item).
 *
 * Return any solution if there is more than one solution and return an empty list if there is no solution.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/11/1359_ex1.png" style="width: 191px; height: 181px;" />
 *
 * Input: n = 8, m = 2, group = [-1,-1,1,0,0,1,0,-1], beforeItems = [[],[6],[5],[6],[3,6],[],[],[]]
 * Output: [6,3,4,1,5,2,0,7]
 *
 * Example 2:
 *
 * Input: n = 8, m = 2, group = [-1,-1,1,0,0,1,0,-1], beforeItems = [[],[6],[5],[6],[3],[],[4],[]]
 * Output: []
 * Explanation: This is the same as example 1 except that 4 needs to be before 6 in the sorted list.
 *
 *  
 * Constraints:
 *
 * 	1 <= m <= n <= 3 * 10^4
 * 	group.length == beforeItems.length == n
 * 	-1 <= group[i] <= m - 1
 * 	0 <= beforeItems[i].length <= n - 1
 * 	0 <= beforeItems[i][j] <= n - 1
 * 	i != beforeItems[i][j]
 * 	beforeItems[i] does not contain duplicates elements.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-items-by-groups-respecting-dependencies/
// discuss: https://leetcode.com/problems/sort-items-by-groups-respecting-dependencies/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/sort-items-by-groups-respecting-dependencies/solutions/3067050/just-a-runnable-solution/
    pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        let (n, m) = (n as usize, m as usize);
        let mut result = vec![];
        let mut stat = vec![0; n + 2 * m];
        let mut al = vec![std::collections::HashSet::new(); n + 2 * m];
        for i in 0..n {
            if group[i] != -1 {
                al[n + group[i] as usize].insert(i as i32);
                al[i].insert(n as i32 + m as i32 + group[i]);
            }
            for &j in before_items[i].iter() {
                if group[i] != -1 && group[i] == group[j as usize] {
                    al[j as usize].insert(i as i32);
                } else {
                    let ig = if group[i] == -1 {
                        i as i32
                    } else {
                        n as i32 + group[i]
                    };
                    let jg = if group[j as usize] == -1 {
                        j
                    } else {
                        n as i32 + m as i32 + group[j as usize]
                    };
                    al[jg as usize].insert(ig);
                }
            }
        }
        for n in (0..al.len()).rev() {
            if !Self::top_sort(&al, n, &mut result, &mut stat) {
                return vec![];
            }
        }
        result.reverse();
        result.retain(|&i| i < n as i32);
        result
    }

    fn top_sort(
        al: &Vec<std::collections::HashSet<i32>>,
        i: usize,
        result: &mut Vec<i32>,
        stat: &mut Vec<i32>,
    ) -> bool {
        if stat[i] != 0 {
            return stat[i] == 2;
        }
        stat[i] = 1;
        for &n in al[i].iter() {
            if !Self::top_sort(al, n as usize, result, stat) {
                return false;
            }
        }
        stat[i as usize] = 2;
        result.push(i as i32);
        true
    }
}

// submission codes end

#[cfg(test)]
#[ignore]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1203_example_1() {
        let n = 8;
        let m = 2;
        let group = vec![-1, -1, 1, 0, 0, 1, 0, -1];
        let before_items = vec![
            vec![],
            vec![6],
            vec![5],
            vec![6],
            vec![3, 6],
            vec![],
            vec![],
            vec![],
        ];
        let result = vec![6, 3, 4, 1, 5, 2, 0, 7];

        assert_eq!(Solution::sort_items(n, m, group, before_items), result);
    }
    use super::*;

    #[test]
    #[ignore]
    fn test_1203_example_2() {
        let n = 8;
        let m = 2;
        let group = vec![-1, -1, 1, 0, 0, 1, 0, -1];
        let before_items = vec![
            vec![],
            vec![6],
            vec![5],
            vec![6],
            vec![3],
            vec![],
            vec![4],
            vec![],
        ];
        let result = vec![];

        assert_eq!(Solution::sort_items(n, m, group, before_items), result);
    }
}
