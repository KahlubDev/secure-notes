# Security Review: Note Search

## Finding

The note search functionality must never construct SQL statements by concatenating user-controlled input.

## Risk

If user input becomes part of an SQL statement directly, an attacker might alter the intended query.

## Secure requirement

All user-controlled values must use SQLx parameter binding.

Example:

```rust
sqlx::query("SELECT id, title, content, created_at FROM notes WHERE title = ?")
    .bind(user_input)