/**
 * [1286] Iterator for Combination
 *
 * Design the CombinationIterator class:
 *
 * 	CombinationIterator(string characters, int combinationLength) Initializes the object with a string characters of sorted distinct lowercase English letters and a number combinationLength as arguments.
 * 	next() Returns the next combination of length combinationLength in lexicographical order.
 * 	hasNext() Returns true if and only if there exists a next combination.
 *
 *
 * Example 1:
 *
 * Input
 * ["CombinationIterator", "next", "hasNext", "next", "hasNext", "next", "hasNext"]
 * [["abc", 2], [], [], [], [], [], []]
 * Output
 * [null, "ab", true, "ac", true, "bc", false]
 * Explanation
 * CombinationIterator itr = new CombinationIterator("abc", 2);
 * itr.next();    // return "ab"
 * itr.hasNext(); // return True
 * itr.next();    // return "ac"
 * itr.hasNext(); // return True
 * itr.next();    // return "bc"
 * itr.hasNext(); // return False
 *
 *
 * Constraints:
 *
 * 	1 <= combinationLength <= characters.length <= 15
 * 	All the characters of characters are unique.
 * 	At most 10^4 calls will be made to next and hasNext.
 * 	It is guaranteed that all calls of the function next are valid.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/iterator-for-combination/
// discuss: https://leetcode.com/problems/iterator-for-combination/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct CombinationIterator {
    characters: Vec<u8>,
    next_combination: Option<u16>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        let next_combination =
            ((1_u16 << (combination_length as u16)) - 1_u16) << 16 - combination_length;
        Self {
            characters: characters.as_bytes().to_vec(),
            next_combination: Some(next_combination),
        }
    }

    fn next(&mut self) -> String {
        let result = self
            .next_combination
            .and_then(|combination| {
                let mut c = combination.clone();
                let mut s = Vec::<u8>::new();
                while c != 0 {
                    let leading = c.leading_zeros();
                    c &= !(1 << (15 - leading));
                    s.push(self.characters.get(leading as usize).unwrap().clone());
                }
                unsafe { Some(String::from_utf8_unchecked(s)) }
            })
            .unwrap();
        self.next_combination = self.next_combination.and_then(|combination| {
            let mut c = combination.clone() >> (16 - self.characters.len());
            // actually c.trailing_ones()
            let trailing_1s = (!c).trailing_zeros();
            // i.e. 110011 -> 1100 or 10010 -> 10010
            c >>= trailing_1s;
            if c == 0 {
                return None;
            }
            let trailing_0s = c.trailing_zeros();
            // i.e. 1100 -> 1000 or 10010 -> 10000
            c &= !(1 << trailing_0s);
            // i.e. 1000 -> 100000 or 10000 -> 10000
            c <<= trailing_1s;
            // i.e. 100000 -> 101110 or 10000 -> 10001
            c |= ((1 << (trailing_1s + 1)) - 1) << (trailing_0s - 1);
            Some(c << (16 - self.characters.len()))
        });
        result
    }

    fn has_next(&self) -> bool {
        self.next_combination.is_some()
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1286_example_1() {
        let mut itr = CombinationIterator::new("abc".to_string(), 2);
        assert_eq!(itr.next(), "ab".to_string());
        assert_eq!(itr.has_next(), true);
        assert_eq!(itr.next(), "ac".to_string());
        assert_eq!(itr.has_next(), true);
        assert_eq!(itr.next(), "bc".to_string());
        assert_eq!(itr.has_next(), false);
    }
}
