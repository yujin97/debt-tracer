BEGIN;
    ALTER TABLE debts add COLUMN status TEXT;
    UPDATE debts SET status = 'pending';
    ALTER TABLE debts ALTER COLUMN status SET NOT NULL;
COMMIT;
