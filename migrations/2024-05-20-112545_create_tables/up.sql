-- Your SQL goes here
CREATE TABLE users (
    user_id SERIAL,
    user_name VARCHAR(50) NOT NULL,
    password VARCHAR(50) NOT NULL,
    PRIMARY KEY(user_id)
);

CREATE TABLE accounts (
    account_id SERIAL,
    balance INT NOT NULL,
    currency VARCHAR(255) NOT NULL,
    account_type VARCHAR(255) NOT NULL,
    PRIMARY KEY(account_id)
);

CREATE TABLE user_account (
    user_account_id SERIAL,
    user_id INT NOT NULL,
    account_id INT NOT NULL,
    PRIMARY KEY(user_account_id),
    CONSTRAINT fk_user
      FOREIGN KEY(user_id)
        REFERENCES users(user_id),
    CONSTRAINT fk_accounts
      FOREIGN KEY(account_id)
        REFERENCES accounts(account_id)
);

CREATE TABLE deposit (
    deposit_id SERIAL,
    account_id INT NOT NULL,
    amount INT NOT NULL,
    deposit_date VARCHAR(255) NOT NULL,
    comment VARCHAR(255) NOT NULL,
    PRIMARY KEY(deposit_id),
    CONSTRAINT fk_accounts
      FOREIGN KEY(account_id)
        REFERENCES accounts(account_id)
);

CREATE TABLE withdraw (
    withdraw_id SERIAL,
    account_id INT NOT NULL,
    amount INT NOT NULL,
    withdraw_date VARCHAR(255) NOT NULL,
    comment VARCHAR(255) NOT NULL,
    PRIMARY KEY(withdraw_id),
    CONSTRAINT fk_accounts
      FOREIGN KEY(account_id)
        REFERENCES accounts(account_id)
);