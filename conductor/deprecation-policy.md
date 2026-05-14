# Deprecation And Withdrawal Policy

Use this policy when a plugin, provider, release channel, or integration is no
longer safe or useful.

## Statuses

- `active`: implemented and supported within documented limits.
- `experimental`: available behind opt-in flags or preview wording.
- `deprecated`: still present but should not be used for new work.
- `withdrawn`: removed or disabled because it is unsafe, broken, or unsupported.
- `superseded`: replaced by another track, plugin, provider, or release channel.

## Requirements

- Deprecation needs a replacement path or explicit rationale.
- Public docs must update before or with the status change.
- Release notes must mention user-visible deprecations.
- Tests should ensure deprecated features fail safely or warn clearly.
- Withdrawn integrations must not remain advertised as available.
