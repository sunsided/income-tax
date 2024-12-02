# Income Tax Calculation

This Rust crate provides functionality for income tax calculation in different years.
It is currently aimed at German income tax but is welcome to additions for different
countries and years.

```rust
use income_tax::{IncomeTax, germany};

fn germany_2024() {
    let tax = germany::IncomeTax2024;

    let net_income = tax.calculate(70_000.0);
    assert_eq!(net_income, Ok(18_797.0));
    
    let refund = tax.tax_refund(100_000.0, 50_000.0);
    assert_eq!(refund, Ok(20_491.0));
}
```
