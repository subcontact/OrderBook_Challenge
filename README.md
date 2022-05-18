# Order Book Programming Exercise

Produce a program which maintains price-time limit order books, one per trading symbol. The program
should accept new orders, order cancellations, and flushes from a CSV file and publish top of book (best bid
and ask) changes for each order book. Supporting trades or matching is optional. See the details below.

# Instructions

To compile the program run the following:

```
cargo build
```
To run the test cases:

```
cargo test -- --show-output
```

To run the input file sample:

```
cargo run input_csv.csv
```

# Considerations:

Some bonus challenges were not solved in this solution, including:
```
    - Implementation of trade orders 
    - Scenarios 13 and 14
    - Including extra scenarios
    - Containerizing the program
```
    
    
# Improvements To be Done:
    - Reuse the same buffer while reading each line of the csv file to avoid allocating a new string for each line. (line 11: src/csv_parse.rs)
    - While doing cancellations, the book is searched linearly. This could be improved using a binary search (line 77: src/process_order.rs).
    - Implement threads to process operations simultaneously
