use rust_decimal::prelude::*;

#[derive(Debug)]
pub struct DebtAmount(Decimal);

impl DebtAmount {
    pub fn parse(amount: f64) -> Result<Self, String> {
        let is_amount_negative = amount < 0.0;

        let is_amount_not_a_number = amount.is_nan();

        if is_amount_negative || is_amount_not_a_number {
            return Err(format!("{} is not a positive number.", amount));
        }

        match Decimal::from_f64(amount) {
            Some(amount) => Ok(Self(amount)),
            None => Err(format!("{} is not a valid amount.", amount)),
        }
    }

    pub fn inner(&self) -> Decimal {
        self.0
    }

    pub fn inner_f64(&self) -> Result<f64, String> {
        match self.0.to_f64() {
            Some(inner) => Ok(inner),
            None => Err(format!("{} cannot be converted into f64", self.0)),
        }
    }
}

impl AsRef<Decimal> for DebtAmount {
    fn as_ref(&self) -> &Decimal {
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
