mod debt_amount;
mod debt_currency;
mod debt_description;
mod debt_status;
mod debt_user_id;

pub use debt_amount::DebtAmount;
pub use debt_currency::DebtCurrency;
pub use debt_description::DebtDescription;
pub use debt_status::DebtStatus;
pub use debt_user_id::DebtUserId;

#[derive(Debug)]
pub struct NewDebt {
    pub debtor_id: DebtUserId,
    pub creditor_id: DebtUserId,
    pub amount: DebtAmount,
    pub currency: DebtCurrency,
    pub description: DebtDescription,
    pub status: DebtStatus,
}
