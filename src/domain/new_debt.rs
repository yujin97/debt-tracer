use crate::domain::debt_amount::DebtAmount;
use crate::domain::debt_currency::DebtCurrency;
use crate::domain::debt_description::DebtDescription;
use crate::domain::debt_user_id::DebtUserId;

#[derive(Debug)]
pub struct NewDebt {
    pub debtor_id: DebtUserId,
    pub creditor_id: DebtUserId,
    pub amount: DebtAmount,
    pub currency: DebtCurrency,
    pub description: DebtDescription,
}
