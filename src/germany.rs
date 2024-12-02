//! Provides income tax calculations for Germany.

use crate::{IncomeTax, IncomeTaxError};

/// Provides different calculations based on the year.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "year"))]
pub enum IncomeTaxType {
    /// Income taxes based on rules for the year 2024.
    #[cfg_attr(feature = "serde", serde(rename = "2024"))]
    IncomeTax2024(IncomeTax2024),
}

impl IncomeTax for IncomeTaxType {
    fn year(&self) -> u32 {
        match self {
            IncomeTaxType::IncomeTax2024(tax) => tax.year(),
        }
    }

    fn calculate(&self, income: f64) -> Result<f64, IncomeTaxError> {
        match self {
            IncomeTaxType::IncomeTax2024(tax) => tax.calculate(income),
        }
    }

    fn tax_refund(&self, income_before: f64, income_after: f64) -> Result<f64, IncomeTaxError> {
        match self {
            IncomeTaxType::IncomeTax2024(tax) => tax.tax_refund(income_before, income_after),
        }
    }
}

/// Configuration for income taxes according to § 32a EStG (2024).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IncomeTax2024;

impl IncomeTax for IncomeTax2024 {
    fn year(&self) -> u32 {
        2024
    }

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
    fn calculate(&self, income: f64) -> Result<f64, IncomeTaxError> {
        if !income.is_finite() {
            return Err(IncomeTaxError::IncomeNotFinite(income));
        }
        if !income.is_sign_positive() {
            return Err(IncomeTaxError::NegativeIncome(income));
        }

        // https://www.steuertipps.de/gesetze/estg/32a-einkommensteuertarif
        // "[...] des auf einen vollen Euro-Betrag abgerundeten zu versteuernden Einkommens"
        let income = income.floor();
        let taxes = match income {
            0.0..11_605.0 => {
                // Grundfreibetrag
                0.0
            }
            11_605.0..17_005.0 => {
                // Progressionszone I / Untere Progressionszone
                let y = (income - 11_605.0) / 10_000.0;
                (922.98 * y + 1_400.0) * y
            }
            17_005.0..66_760.0 => {
                // Progressionszone II / Obere Progressionszone
                let z = (income - 17_005.0) / 10_000.0;
                (181.19 * z + 2_397.0) * z + 1_025.38
            }
            66_760.0..277_825.0 => {
                // Proportionalzone I / ("Spitzensteuersatz")
                let x = income;
                0.42 * x - 10_602.13
            }
            277_825.0.. => {
                // Proportionalzone II / "Reichensteuer"
                let x = income;
                0.45 * x - 18_936.88
            }
            _ => unreachable!(),
        };

        // "Der sich ergebende Steuerbetrag ist auf den nächsten vollen Euro-Betrag abzurunden."
        Ok(taxes.floor())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_income() {
        let tax = IncomeTax2024;
        assert_eq!(tax.calculate(0.0), Ok(0.0));
    }

    #[test]
    fn test_below_basic_allowance() {
        let tax = IncomeTax2024;
        assert_eq!(tax.calculate(11_000.0), Ok(0.0)); // Below Grundfreibetrag, should be 0 tax
    }

    #[test]
    fn test_at_basic_allowance() {
        let tax = IncomeTax2024;
        assert_eq!(tax.calculate(11_605.0), Ok(0.0)); // Grundfreibetrag, no tax
    }

    #[test]
    fn test_in_lower_progression_zone() {
        let tax = IncomeTax2024;
        let result = tax.calculate(12_000.0);
        assert_eq!(result, Ok(56.0)); // Tax should be positive
    }

    #[test]
    fn test_in_upper_progression_zone() {
        let tax = IncomeTax2024;
        let result = tax.calculate(20_000.0);
        assert_eq!(result, Ok(1_759.0)); // Tax should be positive and higher than lower zones
    }

    #[test]
    fn test_in_upper_progression_zone_2() {
        let tax = IncomeTax2024;
        let result = tax.calculate(50_000.0);
        assert_eq!(result, Ok(10_906.0));
    }

    #[test]
    fn test_in_top_rate_zone() {
        let tax = IncomeTax2024;
        let result = tax.calculate(70_000.0);
        assert_eq!(result, Ok(18_797.0)); // Tax should be significant
    }

    #[test]
    fn test_at_rich_tax_zone() {
        let tax = IncomeTax2024;
        let result = tax.calculate(300_000.0);
        assert_eq!(result, Ok(116_063.0)); // Reichensteuer applies
    }

    #[test]
    fn test_negative_income() {
        let tax = IncomeTax2024;
        assert!(tax.calculate(-5000.0).is_err());
    }

    #[test]
    fn test_infinite_income() {
        let tax = IncomeTax2024;
        assert_eq!(
            tax.calculate(f64::INFINITY),
            Err(IncomeTaxError::IncomeNotFinite(f64::INFINITY))
        );
        assert_eq!(
            tax.calculate(f64::NEG_INFINITY),
            Err(IncomeTaxError::IncomeNotFinite(f64::NEG_INFINITY))
        );
    }

    #[test]
    fn test_non_finite_values() {
        let tax = IncomeTax2024;
        assert!(tax.calculate(f64::NAN).is_err());
    }

    #[test]
    fn test_large_income() {
        let tax = IncomeTax2024;
        let result = tax.calculate(1_000_000.0);
        assert!(result.expect("nonzero tax expected") > 0.0);
    }

    #[test]
    fn test_tax_refund() {
        let tax = IncomeTax2024;
        let refund = tax.tax_refund(100_000.0, 50_000.0);
        assert_eq!(refund, Ok(20_491.0));
    }
}
