# Contributing

Keep changes inside this repository. Do not copy host scheduler, worker, lease, approval, authorization, or database orchestration into the adapter.

For dispatch changes, add deterministic tests that inject a mock `GitHubActionsTransport` or `GitHubHttpExecutor`. Tests must not contact GitHub or require credentials. Preserve fail-closed behavior:

- incomplete lookup is not authoritative absence;
- ambiguous POST outcome is `UNKNOWN`;
- an invocation sends no more than one POST;
- postflight lookup is evidence collection, never a retry;
- missing, mismatched, or payload-less durable intent never authorizes a dispatch;
- secrets never enter immutable payloads or process arguments.

Run `cargo test --locked`, `cargo fmt -- --check`, `cargo clippy --all-targets --locked -- -D warnings`, and rustdoc with warnings denied.

The package has no sibling-package dependencies. Keep provider intent DTOs and hashing logic local; do not add host-runtime or external repository dependencies.
