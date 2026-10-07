# Stellar Contract Conformance

The project provides independent behavioral conformance testing for Soroban smart-contract standards, beginning with SEP-41.

## Purpose

The project's goal is to provide a reusable behavioral conformance testing framework. It must distinguish between interface compatibility and behavioral conformance. The project verifies that a contract behaves according to the selected standard, not merely that the contract exposes the expected functions.

**Note:** The project does not provide security auditing, formal verification, or certification.

## Current Status

The project is currently under active development. SEP-41 is the first planned conformance profile.

## Scope

The project intends to test:
* contract behavior
* state transitions
* events
* authorization
* edge cases
* standard-specific requirements

## Non-goals

The project is not currently:
* a security auditor
* a vulnerability scanner
* a blockchain explorer
* a live-network contract scanner
* an official Stellar/SDF project

## Development

* [ROADMAP.md](ROADMAP.md)
* [CONTRIBUTING.md](CONTRIBUTING.md)
