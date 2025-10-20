ALTER TABLE payments ADD COLUMN block_number BIGINT NOT NULL;

CREATE INDEX idx_payments_block_number ON payments(block_number);