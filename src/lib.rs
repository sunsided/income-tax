pub mod germany;

/// Calculation of income taxes.
pub trait IncomeTax {
    /// Gets the year of the income tax.
    fn year(&self) -> u32;

    /// Calculates the taxes on the specified income.
    fn calculate(&self, income: f64) -> Result<f64, IncomeTaxError>;

    /// Calculate the tax refund for an income before any adjustments (the basic income),
    /// and an income after any adjustments (e.g. tax deductions). The value returned is
    /// the tax refund (if positive) or tax due (if negative).
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
    NegativeIncome(f64)
}

#[cfg(feature = "std")]
impl std::error::Error for IncomeTaxError {
}

impl std::fmt::Display for IncomeTaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncomeTaxError::IncomeNotFinite(_) => {
                f.write_str("The provided income was not a finite number")
            },
            IncomeTaxError::NegativeIncome(_) => {
                f.write_str("The provided income was negative")
            }
        }
    }
}
