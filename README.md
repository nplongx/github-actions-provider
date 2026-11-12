# Runalith GitHub Actions Provider

A Rust adapter for dispatching and reconciling GitHub Actions workflow runs from a durable workflow host.

This repository extracts the provider boundary from Runalith. It contains the provider contracts, versioned provider-effect DTOs, GitHub Actions target validation, lookup/pagination, dispatch classification, bounded curl transport, and terminal-run evidence collection.

## Terminology

See [TERMINOLOGY.md](TERMINOLOGY.md) for shared identifier, lifecycle, evidence, time-unit, and compatibility conventions.

## Scope

- Bind dispatch to an immutable provider intent and bounded JSON payload.
- Require a complete, exact-scope lookup before sending a workflow dispatch.
- Send at most one dispatch POST per adapter invocation.
- Perform one postflight lookup after an accepted POST.
- Return `UNKNOWN` when transmission or remote outcome is ambiguous. Ambiguity is not permission to retry.
- Bind versioned provider-effect observations and terminal proofs to exact provider account, run identity, control-fence fields, and response bytes.
- Inject HTTP transport for deterministic tests. The included curl transport accepts a credential separately from the immutable request and writes curl configuration through stdin rather than process arguments.

## Non-goals and host responsibilities

This crate is not a workflow engine, scheduler, lease manager, identity provider, authorization service, or durable transaction coordinator. It does not independently authorize a workflow dispatch.

The host must:

1. Authenticate the caller and authorize the exact repository, workflow, ref, provider account, and requested effect.
2. Persist and validate a durable provider intent before calling the adapter.
3. Enforce single-flight dispatch for each immutable intent across threads and processes. The adapter's lookup-before-POST sequence is not an atomic provider-side reservation; concurrent calls can race between lookup and POST.
4. Supply a trusted Control-fence binding and verify the receipt/authority inputs before treating a versioned request as authorized.
5. Keep credentials outside immutable payloads, logs, source control, and user-controlled arguments. Rotate and scope credentials outside this crate.
6. Treat `UNKNOWN` as unresolved. Reconcile using the same intent; never create a replacement intent or blindly replay because a run is not immediately visible.
7. Independently verify provider terminal evidence and decide whether it is sufficient for workflow settlement.

A successful local test does not prove GitHub account permissions, repository policy, workflow correctness, remote execution, or terminal completion. The included test suite uses injected mock transports and local curl fixtures; it does not contact GitHub.

## Structure

- `src/lib.rs`: provider adapter boundary, immutable request validation, and dispatch result types.
- `src/contract_v1.rs`: versioned provider-effect, Control-fence, observation, and terminal-proof contracts.
- `src/github_actions.rs`: GitHub Actions adapter, exact run lookup, transport seam, curl executor, and terminal evidence collection.

## Dependencies

The crate pins its two Runalith-independent contract dependencies to immutable Git revisions:

- [`workflow-domain`](https://github.com/nplongx/workflow-domain)
- [`provider-intent-ledger`](https://github.com/nplongx/provider-intent-ledger)

They are Git dependencies, not crates.io dependencies. This package is currently intended for Cargo Git dependencies; `cargo package`/crates.io publication is not supported until those dependency crates are published or vendored.

## Build and test

Requires Rust 1.85 or newer.

```sh
cargo test --locked
cargo fmt -- --check
cargo clippy --all-targets --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps
```

The GitHub Actions workflow runs the same checks on Rust 1.85 and stable.

## Provenance

Extracted from `nplongx/runalith` at source commit `29c24cfa2b6ead5704d26cf2746bc384f2938b80`, from `rust/crates/runalith-providers`. The extracted crate replaces workspace path dependencies with the pinned public repositories above; provider behavior is otherwise retained.

## License

MIT. See [LICENSE](LICENSE).
