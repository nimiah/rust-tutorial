# Database Naming Convention

## 1. Table Naming
- Use plural nouns
- Use snake_case
- Use lowercase

Examples:
- users
- articles

---

## 2. Column Naming
- Use snake_case
- Use meaningful names
- Use lowercase

Examples:
- id
- owner_id
- created_at
- password_hash
- password_salt

---

## 3. Primary Key
- Column name: `id`
- Constraint: `pk_<table>`

Examples:
- pk_users
- pk_articles

---

## 4. Foreign Key
- Constraint: `fk__<table>__<column>`

Examples:
- fk__articles__owner_id

---

## 5. Unique Constraint
- Constraint: `uq_<table>_<column>`

Examples:
- uq_users_email

---

## 6. Index Naming
- Format: `idx_<table>_<column>`

Examples:
- idx_users_email
- idx_articles_owner_id
- idx_articles_visibility

---

## 7. Timestamp Fields
- Use:
  - created_at
  - updated_at

---

## 8. Boolean Fields
- Use prefix:
  - is_
  - has_

Examples:
- is_active
- is_deleted

---

## 9. General Rules
- Always lowercase
- Use snake_case
- Avoid abbreviations
- Keep naming consistent across the schema