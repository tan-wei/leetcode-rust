/**
 * [1169] Invalid Transactions
 *
 * A transaction is possibly invalid if:
 *
 * 	the amount exceeds $1000, or;
 * 	if it occurs within (and including) 60 minutes of another transaction with the same name in a different city.
 *
 * You are given an array of strings transaction where transactions[i] consists of comma-separated values representing the name, time (in minutes), amount, and city of the transaction.
 * Return a list of transactions that are possibly invalid. You may return the answer in any order.
 *  
 * Example 1:
 *
 * Input: transactions = ["alice,20,800,mtv","alice,50,100,beijing"]
 * Output: ["alice,20,800,mtv","alice,50,100,beijing"]
 * Explanation: The first transaction is invalid because the second transaction occurs within a difference of 60 minutes, have the same name and is in a different city. Similarly the second one is invalid too.
 * Example 2:
 *
 * Input: transactions = ["alice,20,800,mtv","alice,50,1200,mtv"]
 * Output: ["alice,50,1200,mtv"]
 *
 * Example 3:
 *
 * Input: transactions = ["alice,20,800,mtv","bob,50,1200,mtv"]
 * Output: ["bob,50,1200,mtv"]
 *
 *  
 * Constraints:
 *
 * 	transactions.length <= 1000
 * 	Each transactions[i] takes the form "{name},{time},{amount},{city}"
 * 	Each {name} and {city} consist of lowercase English letters, and have lengths between 1 and 10.
 * 	Each {time} consist of digits, and represent an integer between 0 and 1000.
 * 	Each {amount} consist of digits, and represent an integer between 0 and 2000.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/invalid-transactions/
// discuss: https://leetcode.com/problems/invalid-transactions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/invalid-transactions/solutions/901363/rust-4ms-faster-than-100-at-the-time-of-writing/

#[derive(Debug, Eq, PartialEq, Clone)]
struct Transaction {
    name: String,
    time: i32,
    amount: i32,
    city: String,
}

impl Transaction {
    pub fn new(name: String, time: i32, amount: i32, city: String) -> Self {
        Transaction {
            name,
            time,
            amount,
            city,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{},{}", self.name, self.time, self.amount, self.city)
    }
}

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut trans = Vec::with_capacity(transactions.len());
        for transaction in transactions.iter() {
            let data = transaction.split(",").collect::<Vec<&str>>();
            let tran = Transaction::new(
                data[0].to_string(),
                data[1].parse::<i32>().unwrap(),
                data[2].parse::<i32>().unwrap(),
                data[3].to_string(),
            );
            trans.push(tran);
        }

        let mut result = Vec::new();

        for i in 0..trans.len() {
            for j in 0..trans.len() {
                if i == j {
                    continue;
                }
                if trans[i].name == trans[j].name
                    && (trans[i].time - trans[j].time).abs() <= 60
                    && trans[i].city != trans[j].city
                {
                    result.push(trans[i].to_string());
                    break;
                }
                if trans[i].amount >= 1000 {
                    result.push(trans[i].to_string());
                    break;
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1169_example_1() {
        let transactions = vec_string!["alice,20,800,mtv", "alice,50,100,beijing"];
        let result = vec_string!["alice,20,800,mtv", "alice,50,100,beijing"];

        assert_eq!(Solution::invalid_transactions(transactions), result);
    }

    #[test]
    fn test_1169_example_2() {
        let transactions = vec_string!["alice,20,800,mtv", "alice,50,1200,mtv"];
        let result = vec_string!["alice,50,1200,mtv"];

        assert_eq!(Solution::invalid_transactions(transactions), result);
    }

    #[test]
    fn test_1169_example_3() {
        let transactions = vec_string!["alice,20,800,mtv", "bob,50,1200,mtv"];
        let result = vec_string!["bob,50,1200,mtv"];

        assert_eq!(Solution::invalid_transactions(transactions), result);
    }
}
