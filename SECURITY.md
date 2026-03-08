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

## Integrity

Dawon stores expected test outputs as SHA-256 commitment hashes.
No plaintext answers appear in the source tree.

It compiles your code, runs it, and measures every byte.

It does not estimate. It does not negotiate.

If you find a way to make it look the other way, report it through
the channel above — do not use it.
