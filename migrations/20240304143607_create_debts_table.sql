CREATE TABLE debts(
    debt_id uuid PRIMARY KEY,
    creditor_id uuid NOT NULL
        REFERENCES users (user_id),
    debtor_id uuid NOT NULL
        REFERENCES users (user_id),
    amount NUMERIC(10,2) NOT NULL,
    currency TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at timestamptz NOT NULL
);
