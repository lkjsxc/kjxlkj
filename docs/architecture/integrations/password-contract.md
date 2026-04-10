# Password Contract

## Algorithm

- Hash function: Argon2id
- Version: `v=19`
- Format: PHC string
- Salt: automatically generated per hash

## Validation Rules

### Minimum Requirements

- Length: >= 8 characters
- Character set: any UTF-8

### No Maximum Length

- Argon2id does not use the old 72-byte bcrypt truncation behavior.
- UI must not enforce a password maximum beyond transport and form limits.

## Hash Format

Argon2id output format: `$argon2id$v=19$m=<memory>,t=<iterations>,p=<lanes>$<salt>$<hash>`

- Version identifier: `$argon2id$`
- Parameters are embedded in the PHC string.
- Salt is random for each hash.
- Stored hashes must begin with `$argon2id$`.

## Operations

### Hash Password

```rust
let hash = password::hash_password(password)?;
```

### Verify Password

```rust
let valid = password::verify_password(password, &stored_hash)?;
```

## Setup Code

- Initial setup requires a one-time setup code when no admin user exists.
- The server generates the setup code at startup and logs it to the console.
- Verification may provide a deterministic setup code through `SETUP_CODE`.
- Setup code comparison must not leak which submitted field failed.

## Password Reset Token

- Forgotten-password reset uses a one-time token logged to the server console.
- Only an Argon2id hash of the token is stored.
- Reset tokens expire and are consumed on successful use.
- Successful reset invalidates existing sessions.

## Timing Safety

- Argon2id PHC verification is used for stored password and token hashes.
- No early exit on mismatch

## Legacy Hash Rule

- bcrypt-era hashes are not accepted after the migration.
- Existing disposable environments must complete setup or password reset again.

## Error Handling

- Hash failure returns `500 storage_error`
- Verify failure returns `401 unauthorized`
- No distinction between wrong user and wrong password
