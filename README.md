# Rust LeetCode Solutions

## Table of Contents
- [Problem 6: Zigzag Conversion](problem_6_Zigzag-Conversion/README.md)

## How to Create a New Problem

1. Create a new directory for the problem:
    ```sh
    ./add_problem.sh <problem_number> "<problem_name>"
    ```


3. Implement the solution in `src/lib.rs` and write tests in `tests/lib.rs`.

4. Update the `README.md` file in the root directory to include the new problem in the Table of Contents.

## How to Run Tests

1. Navigate to the main directory:
    ```sh
    cargo test -p <problem_number>_<problem_name>
    ```

## TODO
Add the solution's default function signature to the generated main.rs and pull down the question from the leetcode graphql api
or use https://github.com/faisal-shohag/leetcode_api
```
LEETCODE_API_ENDPOINT=https://leetcode.com/graphql?query=query
```

