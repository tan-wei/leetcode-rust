/**
 * [1656] Design an Ordered Stream
 *
 * There is a stream of n (idKey, value) pairs arriving in an arbitrary order, where idKey is an integer between 1 and n and value is a string. No two pairs have the same id.
 * Design a stream that returns the values in increasing order of their IDs by returning a chunk (list) of values after each insertion. The concatenation of all the chunks should result in a list of the sorted values.
 * Implement the OrderedStream class:
 *
 * 	OrderedStream(int n) Constructs the stream to take n values.
 * 	String[] insert(int idKey, String value) Inserts the pair (idKey, value) into the stream, then returns the largest possible chunk of currently inserted values that appear next in the order.
 *
 *  
 * Example:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/10/q1.gif" style="width: 682px; height: 240px;" />
 *
 * Input
 * ["OrderedStream", "insert", "insert", "insert", "insert", "insert"]
 * [[5], [3, "ccccc"], [1, "aaaaa"], [2, "bbbbb"], [5, "eeeee"], [4, "ddddd"]]
 * Output
 * [null, [], ["aaaaa"], ["bbbbb", "ccccc"], [], ["ddddd", "eeeee"]]
 * Explanation
 * // Note that the values ordered by ID is ["aaaaa", "bbbbb", "ccccc", "ddddd", "eeeee"].
 * OrderedStream os = new OrderedStream(5);
 * os.insert(3, "ccccc"); // Inserts (3, "ccccc"), returns [].
 * os.insert(1, "aaaaa"); // Inserts (1, "aaaaa"), returns ["aaaaa"].
 * os.insert(2, "bbbbb"); // Inserts (2, "bbbbb"), returns ["bbbbb", "ccccc"].
 * os.insert(5, "eeeee"); // Inserts (5, "eeeee"), returns [].
 * os.insert(4, "ddddd"); // Inserts (4, "ddddd"), returns ["ddddd", "eeeee"].
 * // Concatentating all the chunks returned:
 * // [] + ["aaaaa"] + ["bbbbb", "ccccc"] + [] + ["ddddd", "eeeee"] = ["aaaaa", "bbbbb", "ccccc", "ddddd", "eeeee"]
 * // The resulting order is the same as the order above.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 1000
 * 	1 <= id <= n
 * 	value.length == 5
 * 	value consists only of lowercase letters.
 * 	Each call to insert will have a unique id.
 * 	Exactly n calls will be made to insert.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-an-ordered-stream/
// discuss: https://leetcode.com/problems/design-an-ordered-stream/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct OrderedStream {
    stream: Vec<Option<String>>,
    next_index: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            stream: vec![None; n as usize],
            next_index: 0,
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let index = (id_key - 1) as usize;
        self.stream[index] = Some(value);

        let mut chunk = Vec::new();

        while self.next_index < self.stream.len() {
            let Some(value) = self.stream[self.next_index].take() else {
                break;
            };
            chunk.push(value);
            self.next_index += 1;
        }

        chunk
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1656_example_1() {
        let mut os = OrderedStream::new(5);
        assert_eq!(os.insert(3, "ccccc".to_string()), Vec::<String>::new()); // Inserts (3, "ccccc"), returns [].
        assert_eq!(os.insert(1, "aaaaa".to_string()), vec_string!["aaaaa"]); // Inserts (1, "aaaaa"), returns ["aaaaa"].
        assert_eq!(
            os.insert(2, "bbbbb".to_string()),
            vec_string!["bbbbb", "ccccc"]
        ); // Inserts (2, "bbbbb"), returns ["bbbbb", "ccccc"].
        assert_eq!(os.insert(5, "eeeee".to_string()), Vec::<String>::new()); // Inserts (5, "eeeee"), returns [].
        assert_eq!(
            os.insert(4, "ddddd".to_string()),
            vec_string!["ddddd", "eeeee"]
        ); // Inserts (4, "ddddd"), returns ["ddddd", "eeeee"].
    }
}
