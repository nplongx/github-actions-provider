# GitHub Actions Provider

A standalone Rust adapter for dispatching and reconciling GitHub Actions workflow runs.

## Terminology

See [TERMINOLOGY.md](TERMINOLOGY.md) for shared identifier, lifecycle, evidence, time-unit, and compatibility conventions.

## Install

```toml
[dependencies]
github-actions-provider = { git = "https://github.com/nplongx/github-actions-provider", tag = "v0.2.0" }
```

All adapter DTOs and hashing code are included in this crate. No sibling package is fetched.

## Scope

- Validate bounded, immutable provider dispatch requests.
- Require complete exact-scope lookup before dispatch.
- Send at most one dispatch POST per adapter invocation.
- Perform one postflight lookup after an accepted POST.
- Keep ambiguous outcomes unresolved; never blindly replay.
- Bind versioned provider-effect observations and terminal proofs to exact provider account, run identity, fence fields, and response bytes.
- Inject HTTP transport for deterministic offline tests. The curl transport keeps credentials out of argv and immutable payloads.

## Host responsibilities

The host authenticates callers, authorizes repository/workflow/ref/provider account/effect, persists the exact intent before dispatch, enforces single-flight reservation, supplies trusted fence observations, and decides whether verified terminal evidence can settle a workflow. This package does not own workflow state, leases, approvals, or durable intent persistence.

`ProviderIntentRecord` and related request/state/outcome DTOs are local input contracts. The host may map its own durable ledger into them. No sibling package is fetched or required.

## Provider identity

The optional workflow input `provider_intent_id` binds a provider run to its stable intent identity. This identifier is owned by the adapter contract and is not a host database key.

## Build and test

Requires Rust 1.85 or newer.

```sh
cargo test --locked
cargo fmt -- --check
cargo clippy --all-targets --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps
```

Tests use mock transports and local fixtures; they do not contact GitHub or trigger remote workflows.

## Provenance

Created from immutable source snapshot `29c24cfa2b6ead5704d26cf2746bc384f2938b80`. The standalone crate contains its own DTOs, hashing helper, adapter, tests, manifest, and CI workflow.

## License

MIT. See [LICENSE](LICENSE).
