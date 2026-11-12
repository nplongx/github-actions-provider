//! Provider intent data accepted by this adapter.
//!
//! These DTOs intentionally contain no persistence implementation. A host may
//! obtain them from any durable ledger, or construct them from an immutable
//! request after applying its own reservation and authorization protocol.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Provider dispatch lifecycle attached to one durable intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProviderDispatchState {
    /// Intent was durably reserved before the provider call.
    Reserved,
    /// Provider acknowledged a durable run identity.
    Accepted,
    /// Provider authoritatively rejected the request.
    Rejected,
    /// External outcome is ambiguous and requires reconciliation.
    Unknown,
}

/// Immutable provider dispatch request supplied by the host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderIntentRequest {
    /// Workflow identity.
    pub workflow_id: String,
    /// Task identity.
    pub task_id: String,
    /// Positive workflow/task attempt number.
    pub attempt: u64,
    /// Stable provider adapter identifier.
    pub provider_id: String,
    /// Stable key reused for idempotency and reconciliation.
    pub idempotency_key: String,
    /// Exact active lease identity.
    pub lease_id: String,
    /// Exact worker identity.
    pub worker_id: String,
    /// Digest of the approved immutable plan.
    pub plan_sha256: String,
    /// Digest of the exact provider request payload.
    pub request_sha256: String,
    /// Exact non-secret provider payload; absent only for historical records.
    #[serde(default)]
    pub payload: Option<Value>,
}

/// Durable provider intent supplied to the adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderIntentRecord {
    /// Stable intent identity.
    pub intent_id: String,
    /// Exact original request binding.
    pub request: ProviderIntentRequest,
    /// Current provider dispatch lifecycle.
    pub state: ProviderDispatchState,
    /// External provider run identity, when accepted.
    pub provider_run_id: Option<String>,
    /// Exact terminal outcome payload, when available.
    pub outcome: Option<ProviderIntentOutcome>,
    /// Creation timestamp.
    pub created_at: String,
    /// Last state transition timestamp.
    pub updated_at: String,
}

/// Terminal outcome observed for one provider intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "status",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
pub enum ProviderIntentOutcome {
    /// Provider accepted and returned a durable run identity.
    Accepted {
        /// Durable provider run identifier.
        provider_run_id: String,
    },
    /// Provider rejected before creating work.
    Rejected {
        /// Stable provider rejection code.
        reason_code: String,
        /// Human-readable diagnostic.
        diagnostic: String,
    },
    /// External outcome is ambiguous; automatic replay is forbidden.
    Unknown {
        /// Explanation of the unresolved external outcome.
        diagnostic: String,
    },
}
