# Income Tax Calculation

[![Crates.io](https://img.shields.io/crates/v/income-tax)](https://crates.io/crates/income-tax)
[![Crates.io](https://img.shields.io/crates/l/income-tax)](https://crates.io/crates/income-tax)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/sunsided/rust-aligned-allocations/rust.yml)](https://github.com/sunsided/fixedstack-rs/actions/workflows/rust.yml)
[![Safety Dance][safety-image]][safety-link]
[![docs.rs](https://img.shields.io/docsrs/income-tax)](https://docs.rs/income-tax/)
[![codecov](https://codecov.io/gh/sunsided/income-tax-rs/graph/badge.svg?token=7KOXJwVQqi)](https://codecov.io/gh/sunsided/income-tax-rs)

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

[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg

[safety-link]: https://github.com/rust-secure-code/safety-dance/
