/**
 * [0749] Contain Virus
 *
 * A virus is spreading rapidly, and your task is to quarantine the infected area by installing walls.
 * The world is modeled as an m x n binary grid isInfected, where isInfected[i][j] == 0 represents uninfected cells, and isInfected[i][j] == 1 represents cells contaminated with the virus. A wall (and only one wall) can be installed between any two 4-directionally adjacent cells, on the shared boundary.
 * Every night, the virus spreads to all neighboring cells in all four directions unless blocked by a wall. Resources are limited. Each day, you can install walls around only one region (i.e., the affected area (continuous block of infected cells) that threatens the most uninfected cells the following night). There will never be a tie.
 * Return the number of walls used to quarantine all the infected regions. If the world will become fully infected, return the number of walls used.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/01/virus11-grid.jpg" style="width: 500px; height: 255px;" />
 * Input: isInfected = [[0,1,0,0,0,0,0,1],[0,1,0,0,0,0,0,1],[0,0,0,0,0,0,0,1],[0,0,0,0,0,0,0,0]]
 * Output: 10
 * Explanation: There are 2 contaminated regions.
 * On the first day, add 5 walls to quarantine the viral region on the left. The board after the virus spreads is:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/01/virus12edited-grid.jpg" style="width: 500px; height: 257px;" />
 * On the second day, add 5 walls to quarantine the viral region on the right. The virus is fully contained.
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/01/virus13edited-grid.jpg" style="width: 500px; height: 261px;" />
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/01/virus2-grid.jpg" style="width: 653px; height: 253px;" />
 * Input: isInfected = [[1,1,1],[1,0,1],[1,1,1]]
 * Output: 4
 * Explanation: Even though there is only one cell saved, there are 4 walls built.
 * Notice that walls are only built on the shared boundary of two different cells.
 *
 * Example 3:
 *
 * Input: isInfected = [[1,1,1,0,0,0,0,0,0],[1,0,1,0,1,1,1,1,1],[1,1,1,0,0,0,0,0,0]]
 * Output: 13
 * Explanation: The region on the left only builds two new walls.
 *
 *  
 * Constraints:
 *
 * 	m == isInfected.length
 * 	n == isInfected[i].length
 * 	1 <= m, n <= 50
 * 	isInfected[i][j] is either 0 or 1.
 * 	There is always a contiguous viral region throughout the described process that will infect strictly more uncontaminated squares in the next round.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/contain-virus/
// discuss: https://leetcode.com/problems/contain-virus/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/contain-virus/discuss/892024/Rust-translated-8ms-100

const DIRS: [i32; 5] = [0, -1, 0, 1, 0];

impl Solution {
    pub fn contain_virus(is_infected: Vec<Vec<i32>>) -> i32 {
        let mut is_infected = is_infected;
        let mut visited = std::collections::HashSet::<i32>::new();
        let mut regions = Vec::<std::collections::HashSet<i32>>::new();
        let mut frontiers = Vec::<std::collections::HashSet<i32>>::new();
        let mut perimeters = Vec::<i32>::new();
        let m = is_infected.len();
        let n = is_infected[0].len();
        let mut result = 0;
        loop {
            visited.clear();
            regions.clear();
            frontiers.clear();
            perimeters.clear();
            for r in 0..m {
                for c in 0..n {
                    if is_infected[r][c] == 1 && !visited.contains(&((r * n + c) as i32)) {
                        regions.push(std::collections::HashSet::new());
                        frontiers.push(std::collections::HashSet::new());
                        perimeters.push(0);
                        Self::dfs_helper(
                            &mut is_infected,
                            &mut regions,
                            &mut frontiers,
                            &mut perimeters,
                            &mut visited,
                            r as i32,
                            c as i32,
                        );
                    }
                }
            }
            if regions.is_empty() {
                break;
            }

            let mut triage_index = 0;
            for i in 0..frontiers.len() {
                if frontiers[triage_index].len() < frontiers[i].len() {
                    triage_index = i;
                }
            }

            result += perimeters.get(triage_index).unwrap_or_else(|| &0);

            for i in 0..regions.len() {
                if i == triage_index {
                    for &code in &regions[i] {
                        is_infected[code as usize / n][code as usize % n] = -1;
                    }
                } else {
                    for &code in &regions[i] {
                        let r = code / n as i32;
                        let c = code % n as i32;
                        for k in 0..4 {
                            let nr = r + DIRS[k];
                            let nc = c + DIRS[k + 1];
                            if nr >= 0
                                && nr < m as i32
                                && nc >= 0
                                && nc < n as i32
                                && is_infected[nr as usize][nc as usize] == 0
                            {
                                is_infected[nr as usize][nc as usize] = 1;
                            }
                        }
                    }
                }
            }
        }
        result
    }

    fn dfs_helper(
        is_infected: &mut Vec<Vec<i32>>,
        regions: &mut Vec<std::collections::HashSet<i32>>,
        frontiers: &mut Vec<std::collections::HashSet<i32>>,
        perimeters: &mut Vec<i32>,
        visited: &mut std::collections::HashSet<i32>,
        r: i32,
        c: i32,
    ) {
        let m = is_infected.len();
        let n = is_infected[0].len();
        if !visited.contains(&(r * n as i32 + c as i32)) {
            visited.insert(r * n as i32 + c as i32);
        }
        let n2 = regions.len();
        regions[n2 - 1].insert(r * n as i32 + c as i32);
        for k in 0..4 {
            let nr = r + DIRS[k];
            let nc = c + DIRS[k + 1];
            if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                if is_infected[nr as usize][nc as usize] == 1
                    && !visited.contains(&(nr * n as i32 + nc as i32))
                {
                    Self::dfs_helper(is_infected, regions, frontiers, perimeters, visited, nr, nc);
                } else if is_infected[nr as usize][nc as usize] == 0 {
                    frontiers[n2 - 1].insert(nr * n as i32 + nc);
                    perimeters[n2 - 1] += 1;
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0749_example_1() {
        let is_infected = vec![
            vec![0, 1, 0, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let result = 10;

        assert_eq!(Solution::contain_virus(is_infected), result);
    }

    #[test]
    fn test_0749_example_2() {
        let is_infected = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let result = 4;

        assert_eq!(Solution::contain_virus(is_infected), result);
    }

    #[test]
    fn test_0749_example_3() {
        let is_infected = vec![
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
        ];
        let result = 13;

        assert_eq!(Solution::contain_virus(is_infected), result);
    }
}
