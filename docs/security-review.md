# Security Review: Note Search

## Finding

The note search handler constructs an SQL statement using user-controlled input.

## Vulnerable code

```rust
let sql = format!(
    "SELECT id, title, content, created_at
     FROM notes
     WHERE title = '{}'",
    query.title
);