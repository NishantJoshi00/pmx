# Security Policy

## Supported Versions

The following versions of pmx are currently supported with security updates:

| Version | Supported          |
| ------- | ------------------ |
| Latest  | :white_check_mark: |
| < Latest| :x:                |

We recommend always using the latest version of pmx to ensure you have the most recent security patches.

## Reporting a Vulnerability

We take the security of pmx seriously. If you believe you have found a security vulnerability, please report it to us responsibly.

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report security vulnerabilities by emailing the maintainer directly. You can find the maintainer's contact information in the Cargo.toml file or through the GitHub profile.

When reporting a vulnerability, please include:

- A description of the vulnerability and its potential impact
- Steps to reproduce the issue
- Any relevant code snippets or proof of concept
- Your suggested fix (if you have one)

## Security Update Policy

- Security vulnerabilities will be addressed as quickly as possible
- We will acknowledge receipt of your vulnerability report within 48 hours
- We aim to provide a fix within 7 days of verification
- Security updates will be released as patch versions

## Scope

The following are considered in-scope for security vulnerabilities:

- Unauthorized access to or modification of user configuration files
- Path traversal vulnerabilities in profile management
- Command injection through user input
- Privilege escalation issues
- Vulnerabilities in dependencies that affect pmx

The following are out of scope:

- Vulnerabilities in third-party tools that pmx interacts with (Claude, Codex)
- Issues that require physical access to the user's machine
- Social engineering attacks

Thank you for helping keep pmx secure!