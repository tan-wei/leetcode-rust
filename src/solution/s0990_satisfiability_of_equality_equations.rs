/**
 * [0990] Satisfiability of Equality Equations
 *
 * You are given an array of strings equations that represent relationships between variables where each string equations[i] is of length 4 and takes one of two different forms: "xi==yi" or "xi!=yi".Here, xi and yi are lowercase letters (not necessarily different) that represent one-letter variable names.
 * Return true if it is possible to assign integers to variable names so as to satisfy all the given equations, or false otherwise.
 *  
 * Example 1:
 *
 * Input: equations = ["a==b","b!=a"]
 * Output: false
 * Explanation: If we assign say, a = 1 and b = 1, then the first equation is satisfied, but not the second.
 * There is no way to assign the variables to satisfy both equations.
 *
 * Example 2:
 *
 * Input: equations = ["b==a","a==b"]
 * Output: true
 * Explanation: We could assign a = 1 and b = 1 to satisfy both equations.
 *
 *  
 * Constraints:
 *
 * 	1 <= equations.length <= 500
 * 	equations[i].length == 4
 * 	equations[i][0] is a lowercase letter.
 * 	equations[i][1] is either '=' or '!'.
 * 	equations[i][2] is '='.
 * 	equations[i][3] is a lowercase letter.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/satisfiability-of-equality-equations/
// discuss: https://leetcode.com/problems/satisfiability-of-equality-equations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        UnionFind { parent }
    }

    fn find(&self, i: usize) -> usize {
        let j = self.parent[i];
        if i == j {
            i
        } else {
            self.find(j)
        }
    }

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            self.parent[i] = j;
        }
    }
}

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut uf = UnionFind::new(26);
        let equals = equations.iter().filter(|s| s.as_bytes()[1] == b'=');
        let not_equals = equations.iter().filter(|s| s.as_bytes()[1] == b'!');
        for equal in equals {
            let chars: Vec<char> = equal.chars().collect();
            let c1 = chars[0] as usize - 'a' as usize;
            let c2 = chars[3] as usize - 'a' as usize;
            uf.union(c1, c2);
        }
        for not_equal in not_equals {
            let chars: Vec<char> = not_equal.chars().collect();
            let c1 = chars[0] as usize - 'a' as usize;
            let c2 = chars[3] as usize - 'a' as usize;
            if uf.find(c1) == uf.find(c2) {
                return false;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0990_example_1() {
        let equations = vec_string!["a==b", "b!=a"];
        let result = false;

        assert_eq!(Solution::equations_possible(equations), result);
    }

    #[test]
    fn test_0990_example_2() {
        let equations = vec_string!["b==a", "a==b"];
        let result = true;

        assert_eq!(Solution::equations_possible(equations), result);
    }
}
