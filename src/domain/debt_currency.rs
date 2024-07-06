pub enum DebtCurrency {
    AUD,
    CAD,
    CHF,
    CNY,
    EUR,
    GBP,
    HKD,
    JPY,
    KRW,
    NZD,
    SEK,
    SGD,
    USD,
}

const AUD_STR: &str = "AUD";
const CAD_STR: &str = "CAD";
const CHF_STR: &str = "CHF";
const CNY_STR: &str = "CNY";
const EUR_STR: &str = "EUR";
const GBP_STR: &str = "GBP";
const HKD_STR: &str = "HKD";
const JPY_STR: &str = "JPY";
const KRW_STR: &str = "KRW";
const NZD_STR: &str = "NZD";
const SEK_STR: &str = "SEK";
const SGD_STR: &str = "SGD";
const USD_STR: &str = "USD";

impl std::fmt::Display for DebtCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::DebtCurrency::{AUD, CAD, CHF, CNY, EUR, GBP, HKD, JPY, KRW, NZD, SEK, SGD, USD};
        match self {
            AUD => AUD_STR.to_string().fmt(f),
            CAD => CAD_STR.to_string().fmt(f),
            CHF => CHF_STR.to_string().fmt(f),
            CNY => CNY_STR.to_string().fmt(f),
            EUR => EUR_STR.to_string().fmt(f),
            GBP => GBP_STR.to_string().fmt(f),
            HKD => HKD_STR.to_string().fmt(f),
            JPY => JPY_STR.to_string().fmt(f),
            KRW => KRW_STR.to_string().fmt(f),
            NZD => NZD_STR.to_string().fmt(f),
            SEK => SEK_STR.to_string().fmt(f),
            SGD => SGD_STR.to_string().fmt(f),
            USD => USD_STR.to_string().fmt(f),
        }
    }
}

impl DebtCurrency {
    pub fn parse(s: String) -> Result<Self, String> {
        use self::DebtCurrency::{AUD, CAD, CHF, CNY, EUR, GBP, HKD, JPY, KRW, NZD, SEK, SGD, USD};

        // case insensitive
        match s.to_uppercase().as_str() {
            AUD_STR => Ok(AUD),
            CAD_STR => Ok(CAD),
            CHF_STR => Ok(CHF),
            CNY_STR => Ok(CNY),
            EUR_STR => Ok(EUR),
            GBP_STR => Ok(GBP),
            HKD_STR => Ok(HKD),
            JPY_STR => Ok(JPY),
            KRW_STR => Ok(KRW),
            NZD_STR => Ok(NZD),
            SEK_STR => Ok(SEK),
            SGD_STR => Ok(SGD),
            USD_STR => Ok(USD),
            _ => Err(format!("{} is not a valid currency", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_currency_is_rejected() {
        let currency = "deez".to_string();

        assert!(DebtCurrency::parse(currency).is_err());
    }

    #[test]
    fn usd_is_parsed_successfully() {
        let currency = USD_STR.to_string();

        assert!(DebtCurrency::parse(currency).is_ok());
    }
}
