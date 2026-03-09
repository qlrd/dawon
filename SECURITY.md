# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

Older versions are not patched. Upgrade to the latest release.

---

## Reporting a vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Use GitHub's private vulnerability reporting:

```
https://github.com/qlrd/dawon/security/advisories/new
```

Include in your report:

- A clear description of the vulnerability
- Steps to reproduce it
- The potential impact
- A suggested fix or mitigation, if you have one

You will receive an acknowledgement within 48 hours. If the issue
is confirmed, a patch will be published and you will be credited
in the release notes unless you prefer anonymity.

---

## Legal and compliance contact

If you represent 42 School, École 42, or any affiliated entity
and believe this project infringes on your intellectual property
or violates your terms of service, **please contact the
maintainer privately before taking any action**:

```
https://github.com/qlrd/dawon/security/advisories/new
```

Use the subject line: `[Legal] <your organisation> — <brief
description>`.

We are prepared to respond within 48 hours and to remove or
modify any content immediately upon notification.

### What Dawon does not contain

- No exam subjects, exercise texts, or moulinette source.
- No expected outputs in plaintext — all test vectors are stored
  as SHA-256 commitment hashes only.
- No redistribution of norminette or any 42-internal tool.
  Norminette is called as an external subprocess that the user
  must install separately.

---

## Integrity

Dawon stores expected test outputs as SHA-256 commitment hashes.
No plaintext answers appear in the source tree.

It compiles your code, runs it, and measures every byte.

It does not estimate. It does not negotiate.

If you find a way to make it look the other way, report it through
the channel above — do not use it.
