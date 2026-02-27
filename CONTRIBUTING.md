# Contributing to Hporterly

Thanks for contributing. Hporterly is published under `GPL-3.0-or-later` and is
also available under a separate commercial license.

## Ground rules

- Never commit real patient/facility data.
- Use synthetic demo data only.
- Keep any fake patient fields confined to `examples/` and seed/generator scripts.
- Do not commit secrets, keys, tokens, private URLs, or production configs.
- Keep locations and identifiers generic (for example: `Unit A`, `Imaging`, `R-101`).

## Workflow

1. Fork and create a feature branch.
2. Run format/lint/tests and `scripts/sanity_check_no_sensitive_strings.py`.
3. Add or update tests for behavior changes.
4. Update docs for API/configuration/schema changes.
5. Open a pull request with a concise summary and risk notes.

## Pull request expectations

- Explain behavioral impact (queue logic, statuses, realtime/offline behavior).
- Call out migrations or breaking changes explicitly.
- Use sanitized logs/screenshots only.

## CLA (recommended)

Maintainers may request a signed CLA from `CLA/` to support dual licensing.
By submitting a contribution, you confirm you have rights to submit it and that
it does not contain confidential materials or regulated real-world data.

## Security issues

Do not report vulnerabilities publicly. Follow `SECURITY.md`.
