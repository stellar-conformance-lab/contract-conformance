# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in the **Stellar Contract Conformance** framework itself, please use the repository's private GitHub security reporting mechanism (when available) to report it. Do not open a public issue.

## Conformance Failures vs. Security Vulnerabilities

Please clearly distinguish between:
* Security vulnerabilities in this project (the testing framework).
* Conformance failures discovered in a smart contract being tested.

The project must not present conformance failures as security vulnerabilities automatically. If a contract fails a conformance test, it is a behavioral violation, not necessarily a security vulnerability in the framework.
