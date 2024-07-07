#[derive(Debug)]
pub struct DebtAmount(f64);

impl DebtAmount {
    pub fn parse(amount: f64) -> Result<Self, String> {
        let is_amount_negative = amount < 0.0;

        let is_amount_not_a_number = amount.is_nan();

        if is_amount_negative || is_amount_not_a_number {
            return Err(format!("{} is not a positive number.", amount));
        }

        Ok(Self(amount))
    }

    pub fn inner(&self) -> f64 {
        self.0
    }
}

impl AsRef<f64> for DebtAmount {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_number_is_rejected() {
        let amount = -69.0;

        assert!(DebtAmount::parse(amount).is_err());
    }

    #[test]
    fn positive_number_is_parsed_successfully() {
        let amount = 420.69;

        assert!(DebtAmount::parse(amount).is_ok());
    }
}
