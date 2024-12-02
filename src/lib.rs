//! # Income Tax Calculation
//!
//! This Rust crate provides functionality for income tax calculation in different years.
//! It is currently aimed at German income tax but is welcome to additions for different
//! countries and years.
//!
//! ## Example
//!
//! Tax calculations are abstracted over the [`IncomeTax`] trait's [`calculate`](IncomeTax::calculate)
//! function:
//!
//! ```
//! use income_tax::{IncomeTax, germany};
//!
//! let tax = germany::IncomeTax2024;
//! let net_income = tax.calculate(70_000.0);
//! assert_eq!(net_income, Ok(18_797.0));
//! ```
//!
//! Convenience functions exist to directly calculate tax refunds:
//!
//! ```
//! use income_tax::{IncomeTax, germany};
//!
//! let tax = germany::IncomeTax2024;
//! let refund = tax.tax_refund(100_000.0, 50_000.0);
//! assert_eq!(refund, Ok(20_491.0));
//! ```

#![forbid(unsafe_code)]

pub mod germany;

/// Calculation of income taxes.
pub trait IncomeTax {
    /// Gets the year of the income tax.
    fn year(&self) -> u32;

    /// Calculates the taxes on the specified income.
    ///
    /// ## Example
    ///
    /// ```
    /// use income_tax::{IncomeTax, germany};
    ///
    /// let tax = germany::IncomeTax2024;
    /// let net_income = tax.calculate(70_000.0);
    /// assert_eq!(net_income, Ok(18_797.0));
    /// ```
    fn calculate(&self, income: f64) -> Result<f64, IncomeTaxError>;

    /// Calculate the tax refund for an income before any adjustments (the basic income),
    /// and an income after any adjustments (e.g. tax deductions). The value returned is
    /// the tax refund (if positive) or tax due (if negative).
    ///
    /// ## Example
    ///
    /// ```
    /// use income_tax::{IncomeTax, germany};
    ///
    /// let tax = germany::IncomeTax2024;
    /// let refund = tax.tax_refund(100_000.0, 50_000.0);
    /// assert_eq!(refund, Ok(20_491.0));
    /// ```
    fn tax_refund(&self, income_before: f64, income_after: f64) -> Result<f64, IncomeTaxError> {
        let tax_before = self.calculate(income_before)?;
        let tax_after = self.calculate(income_after)?;
        Ok(tax_before - tax_after)
    }
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum IncomeTaxError {
    /// The provided income was not a finite number.
    IncomeNotFinite(f64),
    /// The provided income was negative.
    NegativeIncome(f64),
}

#[cfg(feature = "std")]
impl std::error::Error for IncomeTaxError {}

impl std::fmt::Display for IncomeTaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncomeTaxError::IncomeNotFinite(_) => {
                f.write_str("The provided income was not a finite number")
            }
            IncomeTaxError::NegativeIncome(_) => f.write_str("The provided income was negative"),
        }
    }
}
