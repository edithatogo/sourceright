# ADR 0004: OJS Plugin After Platform-Neutral Contract

## Status

Accepted.

## Decision

OJS integration should call the platform-neutral Sourceright screening contract
instead of embedding separate reference-verification logic.

## Rationale

The journal workflow should stay consistent across OJS and later editorial
platforms. OJS packaging and PKP/Gallery readiness are a distribution layer, not
a second verification engine.
