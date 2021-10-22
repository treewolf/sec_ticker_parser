# sec_ticker_parser

Retrieves and creates a mapping between a stock ticker and its current CIK.
These are defined per the [sec.gov website](https://www.sec.gov/include/ticker.txt).

## Example

```rust
use sec_ticker_parser;

fn main() {
    let map = sec_ticker_parser::list().unwrap();
    let my_cik = sec_ticker_parser::cik(&map, "vz");
    println!("Verizon's CIK is {}", my_cik);
}
```

Output
```plain
Verizon's CIK is 732712
```