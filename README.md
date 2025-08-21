# replicant

The replicant chat protocol is meant to provide private, federated e2e encrypted messaging.
All traffic sent to a node is encrypted by the client and can only be decrypted by the intended recipient.

# Todos

- [ ] sqlx/postgres rewrite
- [ ] index `recipient` and `sent_at`

# Protocol

The replicant chat protocol is kept simple and lightweight, making it easy to implement and use across different platforms and programming languages.
As documentation grows, it may be relocated to a dedicated documentation site, but this file will do for now.

# Registering on a node

Before registering, your client needs to generate a <INFO NEEDED> keypair. A public key is required to register on a node.
Usernames must be unique to a node, alphanumeric and up to 20 characters long.

`POST /register`

```json
{
  "username": "user123",
  "public_key": "public+key"
}
```

---

This will return a response as follows:

```json
{
  "message": "Failure or success message",
  "auth_token": "auth_token or null"
}
```
