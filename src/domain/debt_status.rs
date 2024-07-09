#[derive(Debug)]
pub enum DebtStatus {
    Pending,
    Paid,
    Unpaid,
}

const PENDING_STR: &str = "pending";
const PAID_STR: &str = "paid";
const UNPAID_STR: &str = "unpaid";

impl std::fmt::Display for DebtStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::DebtStatus::{Paid, Pending, Unpaid};
        match self {
            Pending => PENDING_STR.to_string().fmt(f),
            Paid => PAID_STR.to_string().fmt(f),
            Unpaid => UNPAID_STR.to_string().fmt(f),
        }
    }
}
