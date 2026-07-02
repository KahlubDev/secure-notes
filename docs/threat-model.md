# Threat Model: Secure Notes API

## Feature

Create and retrieve personal notes through a REST API.

## Assets

- User notes
- SQLite database
- API availability
- Application logs

## Actors

### Legitimate User
Can create, view, and delete notes.

### Administrator
Maintains the application and database.

### Attacker
Attempts to steal, modify, or destroy data.

## Entry Points

- POST /notes
- GET /notes
- DELETE /notes/{id}

## Trust Boundaries

Internet
        │
        ▼
Axum HTTP Server
        │
        ▼
Business Logic
        │
        ▼
SQLite Database

## Threats

### Spoofing

An attacker pretends to be another user.

Mitigation

Authentication and session validation.

---

### Tampering

An attacker modifies stored notes.

Mitigation

Input validation and parameterized SQL queries.

---

### Repudiation

A user denies creating or deleting a note.

Mitigation

Application logging.

---

### Information Disclosure

Sensitive data is exposed.

Mitigation

Return only required fields.
Avoid verbose error messages.

---

### Denial of Service

Large requests overwhelm the server.

Mitigation

Limit request size.
Validate input length.

---

### Elevation of Privilege

A normal user performs administrator actions.

Mitigation

Authorization checks before sensitive operations.

## Security Controls

- Input validation
- Parameterized SQL
- Structured logging
- Error handling
- Security tests