/**
 * [0721] Accounts Merge
 *
 * Given a list of accounts where each element accounts[i] is a list of strings, where the first element accounts[i][0] is a name, and the rest of the elements are emails representing emails of the account.
 * Now, we would like to merge these accounts. Two accounts definitely belong to the same person if there is some common email to both accounts. Note that even if two accounts have the same name, they may belong to different people as people could have the same name. A person can have any number of accounts initially, but all of their accounts definitely have the same name.
 * After merging the accounts, return the accounts in the following format: the first element of each account is the name, and the rest of the elements are emails in sorted order. The accounts themselves can be returned in any order.
 *  
 * Example 1:
 *
 * Input: accounts = [["John","johnsmith@mail.com","john_newyork@mail.com"],["John","johnsmith@mail.com","john00@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
 * Output: [["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
 * Explanation:
 * The first and second John's are the same person as they have the common email "johnsmith@mail.com".
 * The third John and Mary are different people as none of their email addresses are used by other accounts.
 * We could return these lists in any order, for example the answer [['Mary', 'mary@mail.com'], ['John', 'johnnybravo@mail.com'],
 * ['John', 'john00@mail.com', 'john_newyork@mail.com', 'johnsmith@mail.com']] would still be accepted.
 *
 * Example 2:
 *
 * Input: accounts = [["Gabe","Gabe0@m.co","Gabe3@m.co","Gabe1@m.co"],["Kevin","Kevin3@m.co","Kevin5@m.co","Kevin0@m.co"],["Ethan","Ethan5@m.co","Ethan4@m.co","Ethan0@m.co"],["Hanzo","Hanzo3@m.co","Hanzo1@m.co","Hanzo0@m.co"],["Fern","Fern5@m.co","Fern1@m.co","Fern0@m.co"]]
 * Output: [["Ethan","Ethan0@m.co","Ethan4@m.co","Ethan5@m.co"],["Gabe","Gabe0@m.co","Gabe1@m.co","Gabe3@m.co"],["Hanzo","Hanzo0@m.co","Hanzo1@m.co","Hanzo3@m.co"],["Kevin","Kevin0@m.co","Kevin3@m.co","Kevin5@m.co"],["Fern","Fern0@m.co","Fern1@m.co","Fern5@m.co"]]
 *
 *  
 * Constraints:
 *
 * 	1 <= accounts.length <= 1000
 * 	2 <= accounts[i].length <= 10
 * 	1 <= accounts[i][j].length <= 30
 * 	accounts[i][0] consists of English letters.
 * 	accounts[i][j] (for j > 0) is a valid email.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/accounts-merge/
// discuss: https://leetcode.com/problems/accounts-merge/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/accounts-merge/discuss/1602116/Rust-UnionFind-solution

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        self.parent[y] = x
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut hm = std::collections::HashMap::new();
        let mut uf = UnionFind::new(accounts.len());
        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                if let Some(&j) = hm.get(email) {
                    uf.union(i, j);
                } else {
                    hm.insert(email, i);
                }
            }
        }

        let mut components = std::collections::HashMap::new();
        for (&email, &i) in &hm {
            components
                .entry(uf.find(i))
                .or_insert_with(Vec::new)
                .push(email.clone());
        }

        let mut result = Vec::new();
        for (&i, emails) in components.iter_mut() {
            let mut merged = vec![accounts[i][0].clone()];
            emails.sort();
            merged.extend(emails.iter().cloned());
            result.push(merged);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0721_example_1() {
        let accounts = vec![
            vec_string!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            vec_string!["John", "johnsmith@mail.com", "john00@mail.com"],
            vec_string!["Mary", "mary@mail.com"],
            vec_string!["John", "johnnybravo@mail.com"],
        ];

        let result = vec![
            vec_string![
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com"
            ],
            vec_string!["Mary", "mary@mail.com"],
            vec_string!["John", "johnnybravo@mail.com"],
        ];

        assert_eq_sorted!(Solution::accounts_merge(accounts), result);
    }

    #[test]
    fn test_0721_example_2() {
        let accounts = vec![
            vec_string!["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"],
            vec_string!["Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"],
            vec_string!["Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"],
            vec_string!["Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"],
            vec_string!["Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"],
        ];

        let result = vec![
            vec_string!["Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"],
            vec_string!["Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"],
            vec_string!["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"],
            vec_string!["Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"],
            vec_string!["Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"],
        ];

        assert_eq_sorted!(Solution::accounts_merge(accounts), result);
    }
}
