/**
 * [2034] Stock Price Fluctuation
 *
 * You are given a stream of records about a particular stock. Each record contains a timestamp and the corresponding price of the stock at that timestamp.
 * Unfortunately due to the volatile nature of the stock market, the records do not come in order. Even worse, some records may be incorrect. Another record with the same timestamp may appear later in the stream correcting the price of the previous wrong record.
 * Design an algorithm that:
 *
 * 	Updates the price of the stock at a particular timestamp, correcting the price from any previous records at the timestamp.
 * 	Finds the latest price of the stock based on the current records. The latest price is the price at the latest timestamp recorded.
 * 	Finds the maximum price the stock has been based on the current records.
 * 	Finds the minimum price the stock has been based on the current records.
 *
 * Implement the StockPrice class:
 *
 * 	StockPrice() Initializes the object with no price records.
 * 	void update(int timestamp, int price) Updates the price of the stock at the given timestamp.
 * 	int current() Returns the latest price of the stock.
 * 	int maximum() Returns the maximum price of the stock.
 * 	int minimum() Returns the minimum price of the stock.
 *
 *  
 * Example 1:
 *
 * Input
 * ["StockPrice", "update", "update", "current", "maximum", "update", "maximum", "update", "minimum"]
 * [[], [1, 10], [2, 5], [], [], [1, 3], [], [4, 2], []]
 * Output
 * [null, null, null, 5, 10, null, 5, null, 2]
 * Explanation
 * StockPrice stockPrice = new StockPrice();
 * stockPrice.update(1, 10); // Timestamps are [1] with corresponding prices [10].
 * stockPrice.update(2, 5);  // Timestamps are [1,2] with corresponding prices [10,5].
 * stockPrice.current();     // return 5, the latest timestamp is 2 with the price being 5.
 * stockPrice.maximum();     // return 10, the maximum price is 10 at timestamp 1.
 * stockPrice.update(1, 3);  // The previous timestamp 1 had the wrong price, so it is updated to 3.
 *                           // Timestamps are [1,2] with corresponding prices [3,5].
 * stockPrice.maximum();     // return 5, the maximum price is 5 after the correction.
 * stockPrice.update(4, 2);  // Timestamps are [1,2,4] with corresponding prices [3,5,2].
 * stockPrice.minimum();     // return 2, the minimum price is 2 at timestamp 4.
 *
 *  
 * Constraints:
 *
 * 	1 <= timestamp, price <= 10^9
 * 	At most 10^5 calls will be made in total to update, current, maximum, and minimum.
 * 	current, maximum, and minimum will be called only after update has been called at least once.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/stock-price-fluctuation/
// discuss: https://leetcode.com/problems/stock-price-fluctuation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/stock-price-fluctuation/solutions/4501186/btreemap-hashset-and-hashmap/

struct StockPrice {
    prices: std::collections::HashMap<i32, i32>,
    curr: i32,
    ord_prices: std::collections::BTreeMap<i32, i32>,
    set: std::collections::HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {
    fn new() -> Self {
        Self {
            prices: std::collections::HashMap::new(),
            curr: 0,
            ord_prices: std::collections::BTreeMap::new(),
            set: std::collections::HashSet::new(),
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        if self.prices.contains_key(&timestamp) {
            let val = self.prices.get(&timestamp).unwrap();

            match self.ord_prices.get_mut(&val) {
                Some(x) => {
                    if *x > 1 {
                        *x -= 1;
                    } else {
                        self.ord_prices.remove(&val);
                        self.set.remove(&val);
                    }
                }
                None => {}
            }
        }

        if self.set.contains(&price) {
            match self.ord_prices.get_mut(&price) {
                Some(x) => {
                    *x += 1;
                }
                None => {}
            }
        } else {
            self.ord_prices.insert(price, 1);
            self.set.insert(price);
        }

        self.prices.insert(timestamp, price);

        if timestamp > self.curr {
            self.curr = timestamp;
        }
    }

    fn current(&self) -> i32 {
        self.prices.get(&self.curr).unwrap().clone()
    }

    fn maximum(&self) -> i32 {
        self.ord_prices.last_key_value().unwrap().0.clone()
    }

    fn minimum(&self) -> i32 {
        self.ord_prices.first_key_value().unwrap().0.clone()
    }
}

/**
 * Your StockPrice object will be instantiated and called as such:
 * let obj = StockPrice::new();
 * obj.update(timestamp, price);
 * let ret_2: i32 = obj.current();
 * let ret_3: i32 = obj.maximum();
 * let ret_4: i32 = obj.minimum();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2034_example_1() {
        let mut stock_price = StockPrice::new();
        stock_price.update(1, 10); // Timestamps are [1] with corresponding prices [10].
        stock_price.update(2, 5); // Timestamps are [1,2] with corresponding prices [10,5].
        assert_eq!(stock_price.current(), 5); // return 5, the latest timestamp is 2 with the price being 5.
        assert_eq!(stock_price.maximum(), 10); // return 10, the maximum price is 10 at timestamp 1.
        stock_price.update(1, 3); // The previous timestamp 1 had the wrong price, so it is updated to 3.
        // Timestamps are [1,2] with corresponding prices [3,5].
        assert_eq!(stock_price.maximum(), 5); // return 5, the maximum price is 5 after the correction.
        stock_price.update(4, 2); // Timestamps are [1,2,4] with corresponding prices [3,5,2].
        assert_eq!(stock_price.minimum(), 2); // return 2, the minimum price is 2 at timestamp 4.
    }
}
