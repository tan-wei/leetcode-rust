/**
 * [2349] Design a Number Container System
 *
 * Design a number container system that can do the following:
 *
 * 	Insert or Replace a number at the given index in the system.
 * 	Return the smallest index for the given number in the system.
 *
 * Implement the NumberContainers class:
 *
 * 	NumberContainers() Initializes the number container system.
 * 	void change(int index, int number) Fills the container at index with the number. If there is already a number at that index, replace it.
 * 	int find(int number) Returns the smallest index for the given number, or -1 if there is no index that is filled by number in the system.
 *
 *  
 * Example 1:
 *
 * Input
 * ["NumberContainers", "find", "change", "change", "change", "change", "find", "change", "find"]
 * [[], [10], [2, 10], [1, 10], [3, 10], [5, 10], [10], [1, 20], [10]]
 * Output
 * [null, -1, null, null, null, null, 1, null, 2]
 * Explanation
 * NumberContainers nc = new NumberContainers();
 * nc.find(10); // There is no index that is filled with number 10. Therefore, we return -1.
 * nc.change(2, 10); // Your container at index 2 will be filled with number 10.
 * nc.change(1, 10); // Your container at index 1 will be filled with number 10.
 * nc.change(3, 10); // Your container at index 3 will be filled with number 10.
 * nc.change(5, 10); // Your container at index 5 will be filled with number 10.
 * nc.find(10); // Number 10 is at the indices 1, 2, 3, and 5. Since the smallest index that is filled with 10 is 1, we return 1.
 * nc.change(1, 20); // Your container at index 1 will be filled with number 20. Note that index 1 was filled with 10 and then replaced with 20.
 * nc.find(10); // Number 10 is at the indices 2, 3, and 5. The smallest index that is filled with 10 is 2. Therefore, we return 2.
 *
 *  
 * Constraints:
 *
 * 	1 <= index, number <= 10^9
 * 	At most 10^5 calls will be made in total to change and find.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-a-number-container-system/
// discuss: https://leetcode.com/problems/design-a-number-container-system/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct NumberContainers {
    hash: std::collections::HashMap<i32, i32>,
    numbers: std::collections::HashMap<i32, std::collections::BTreeSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        Self {
            hash: std::collections::HashMap::new(),
            numbers: std::collections::HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if self.hash.contains_key(&index) {
            if self.numbers.contains_key(&self.hash[&index]) {
                self.numbers
                    .get_mut(&self.hash[&index])
                    .unwrap()
                    .remove(&index);
            }
        }

        *self.hash.entry(index).or_insert(number) = number;

        self.numbers
            .entry(number)
            .or_insert(std::collections::BTreeSet::new())
            .insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        if self.numbers.contains_key(&number) {
            if let Some(idx) = self.numbers[&number].first() {
                return *idx;
            }
        }
        -1
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2349_example_1() {
        let mut nc = NumberContainers::new();
        assert_eq!(nc.find(10), -1); // There is no index that is filled with number 10. Therefore, we return -1.
        nc.change(2, 10); // Your container at index 2 will be filled with number 10.
        nc.change(1, 10); // Your container at index 1 will be filled with number 10.
        nc.change(3, 10); // Your container at index 3 will be filled with number 10.
        nc.change(5, 10); // Your container at index 5 will be filled with number 10.
        assert_eq!(nc.find(10), 1); // Number 10 is at the indices 1, 2, 3, and 5. Since the smallest index that is filled with 10 is 1, we return 1.
        nc.change(1, 20); // Your container at index 1 will be filled with number 20. Note that index 1 was filled with 10 and then replaced with 20. 
        assert_eq!(nc.find(10), 2); // Number 10 is at the indices 2, 3, and 5. The smallest index that is filled with 10 is 2. Therefore, we return 2.
    }
}
