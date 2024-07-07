use crate::domain::debt_amount::DebtAmount;
use crate::domain::debt_currency::DebtCurrency;
use crate::domain::debt_description::DebtDescription;
use uuid::Uuid;

#[derive(Debug)]
pub struct NewDebt {
    pub debtor_id: Uuid,
    pub creditor_id: Uuid,
    pub amount: DebtAmount,
    pub currency: DebtCurrency,
    pub description: DebtDescription,
}
