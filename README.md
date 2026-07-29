# My Leetcode Solution in Rust

NOTE: The [original repository](https://github.com/aylei/leetcode-rust) is great! Thanks the author and contributors of it!

Run `cargo run {id}` to initialize the template submission file of "question #id".

Run `cargo run solve {id}` to move the initialized problem into `solution/`.

Run `cargo test test_{id}` to test the solution for "question #id".

Working in progress, to do:

- [ ] auto generation of solution list (when 100 problems solved)

## Usage

* Remove all the solution .rs
* Clean lib.rs file
* Start your leetcode journey in rust by typing `cargo run {question_id}` or typing `cargo run` and then input the question id
* To solve a problem (we should start the problem first), run `cargo run solve {question_id}` or input `solve {question_id}` in interactive mode
* To passby cloudflare, please `cp .env.smaple .env`, and set `LEETCODE_COOKIE` with your cookie (could be find in Firefox or Chrome console) 
