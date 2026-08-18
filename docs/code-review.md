# Code Review: Note Search

## Finding

Reviewer identified SQL injection in the note search handler.

## Problem

The handler builds an SQL statement with `format!()` using `query.title`.

```rust
let sql = format!(
    "SELECT id, title, content, created_at
     FROM notes
     WHERE title = '{}'",
    query.title
);