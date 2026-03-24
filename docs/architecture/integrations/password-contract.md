# Password Contract

## Algorithm

- Hash function: bcrypt
- Cost factor: 12
- Salt: automatically generated per hash

## Validation Rules

### Minimum Requirements

- Length: >= 8 characters
- Character set: any UTF-8

### No Maximum Length

- bcrypt truncates at 72 bytes internally
- UI may warn but does not enforce maximum

## Hash Format

bcrypt output format: `$2b$12$<salt><hash>`

- Version identifier: `$2b$`
- Cost factor: `12`
- Salt: 22 base64 characters
- Hash: 31 base64 characters

## Operations

### Hash Password

```rust
let hash = bcrypt::hash(password, 12)?;
```

### Verify Password

```rust
let valid = bcrypt::verify(password, &stored_hash)?;
```

## Timing Safety

- bcrypt verify is constant-time
- No early exit on mismatch

## Error Handling

- Hash failure returns `500 storage_error`
- Verify failure returns `401 unauthorized`
- No distinction between wrong user and wrong password
