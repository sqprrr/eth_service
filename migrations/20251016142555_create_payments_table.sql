CREATE TABLE payments (
    id            BIGSERIAL PRIMARY KEY,

    tx_hash       VARCHAR(66) NOT NULL,

    log_index     BIGINT NOT NULL,

    sender        VARCHAR(42) NOT NULL,
    recipient     VARCHAR(42) NOT NULL,

    amount_text   TEXT NOT NULL,
    amount_token  VARCHAR(50),

    "timestamp"   TIMESTAMPTZ NOT NULL,

    CONSTRAINT unique_tx_log UNIQUE (tx_hash, log_index)
);