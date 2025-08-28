# replicant

The replicant chat protocol is meant to provide private, federated e2e encrypted messaging.
All traffic sent to a node is encrypted by the client and can only be decrypted by the intended recipient.
Additionally, replicant is entirely no-kyc and does not require any personal information to be shared.

# Protocol

The replicant chat protocol is kept simple and lightweight, making it easy to implement and use across different platforms and programming languages.
As documentation grows, it may be relocated to a dedicated documentation site, but this file will do for now.

## Registering on a node

Before registering, your client needs to generate an PGP keypair. A public key is required to register on a node.
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

The resulting auth_token will be the primary way of authenticating with a node and should be stored securely. If an auth_token is lost, there is no way to recover messages or even the account itself.

> While PGP keys are the recommended way of encrypting and decrypting messages, clients are free to choose any other encryption method. This option is mainly left open to isolated instances as encrypting differently to other clients will lead to compatibility issues.


## Communicating

Exchanging information between clients is done through messages. Replicant messages can both be chat messages or events. A guide for the structure of exchanged data can be found in the client specification, which defines the recommended way of communication in the main network of replicant.

To authenticate and dispatch a message, use your auth token in the `Authorization` header.

`POST /send`

```json
{
  "to": "recipient_username",
  "content": "encrypted_message_content"
}
```

The message content should be encrypted using AES-GCM.

---

This will return a response as follows:

```json
{
  "message": "Failure or success message"
}
```
