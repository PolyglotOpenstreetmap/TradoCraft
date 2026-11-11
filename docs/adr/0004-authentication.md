# 0004 — OIDC authentication, roles per company in the ERP

**Status:** Accepted

**Decision.**
* Production: OpenID Connect. The API verifies bearer tokens against the issuer's JWKS
  (`AUTH_MODE=oidc`, `OIDC_ISSUER`, `OIDC_AUDIENCE`). If the company uses Microsoft 365, Entra ID
  is the identity provider — no Keycloak to operate for 20–50 users.
* The identity provider proves *who* someone is; the ERP decides *what* they may do. Users are
  provisioned in `core.users`; on first login the token subject is bound to the user by e-mail.
* Roles per company map to permissions in code (`core::auth::role_grants`), checked in handlers.
* `AUTH_MODE=dev` trusts an `X-Dev-User` header for local development only.

**Next.** Interactive sign-in in the Flutter app (authorization code + PKCE), MFA enforced at the
identity provider, field-level restrictions (e.g. purchase prices and margins hidden from sales).
