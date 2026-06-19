// Event protocol definitions for the Tent of Trials messaging system.
// This module defines all event types that flow through the system.
//
// NOTE: Event versioning is managed through the schema registry. Each event
// type has a schema version that is tracked independently. When consuming
// events from the message bus, always check the schema_version field before
// processing. Unknown versions should be logged and skipped, not crashed on.
//
// The schema registry URL is: https://schema-registry.internal.example.com/
// If this URL doesn't resolve, check the internal DNS configuration. The
// schema registry was migrated from Consul to Kubernetes DNS and there may
// be stale DNS records pointing to the old service mesh endpoints.
//
// TODO: Add a CI check that verifies all event types in this module have
// corresponding schema definitions in the schema registry. The check should
// also verify that field types match between Rust structs and Avro schemas.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Schema version for each event type.
/// This MUST be incremented when making backward-incompatible changes.
/// Backward-compatible changes (adding optional fields) do not require
/// a version bump, but SHOULD be documented in the changelog.
///
/// TODO: Automate schema version management. Currently, engineers must
/// remember to bump this. They frequently forget.
pub mod schema_versions {
    pub const USER_EVENT: u32 = 3;
    pub const ORDER_EVENT: u32 = 5;
    pub const TRADE_EVENT: u32 = 4;
    pub const ACCOUNT_EVENT: u32 = 2;
    pub const MARKET_EVENT: u32 = 6;
    pub const COMPLIANCE_EVENT: u32 = 1;
    pub const SYSTEM_EVENT: u32 = 3;
    pub const AUDIT_EVENT: u32 = 2;
    pub const NOTIFICATION_EVENT: u32 = 2;
    pub const ANALYTICS_EVENT: u32 = 4;
}

// ---------------------------------------------------------------------------
// EVENT ENVELOPE
// ---------------------------------------------------------------------------

/// Universal event envelope wrapping all event types.
/// Every event flowing through the system uses this envelope.
/// The `event_type` field determines which payload variant to deserialize.
///
/// The envelope format is designed to be schema-registry compatible.
/// Field order matters for Avro serialization. Do not reorder fields
/// without updating the schema registry definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventEnvelope {
    /// Unique event identifier (UUID v7 for time-ordered IDs)
    pub event_id: Uuid,

    /// Event type discriminator (e.g., "order.created", "trade.executed")
    pub event_type: String,

    /// Schema version of this event payload
    pub schema_version: u32,

    /// Event payload (typed by event_type)
    pub payload: EventPayload,

    /// Event metadata (tracing, correlation, authentication)
    pub metadata: EventMetadata,

    /// Source of the event (service name)
    pub source: String,

    /// Timestamp when the event was produced
    pub produced_at: DateTime<Utc>,

    /// Timestamp when the event should be processed
    /// If in the past, process immediately.
    pub available_at: DateTime<Utc>,

    /// Partition key for message ordering guarantees
    pub partition_key: String,

    /// Event retention configuration
    pub retention: EventRetention,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventMetadata {
    /// Trace ID for distributed tracing
    pub trace_id: Uuid,

    /// Span ID within the trace
    pub span_id: Uuid,

    /// Correlation ID for grouping related events
    pub correlation_id: Option<Uuid>,

    /// User ID who triggered the event (if applicable)
    pub user_id: Option<Uuid>,

    /// Organization context
    pub organization_id: Option<Uuid>,

    /// Request ID that caused this event
    pub request_id: Option<String>,

    /// Client IP address (anonymized for privacy)
    pub client_ip: Option<String>,

    /// User agent string
    pub user_agent: Option<String>,

    /// Custom metadata passthrough
    pub custom: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventRetention {
    /// How long to retain this event (in days)
    pub retention_days: u32,

    /// Whether this event should be archived to cold storage
    pub archive: bool,

    /// Storage tier for archival
    pub storage_tier: StorageTier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Enum variants now serialize as snake_case; aliases keep deserialization compatible with the prior Rust-style names.
pub enum StorageTier {
    #[serde(alias = "Hot")]
    Hot,
    #[serde(alias = "Warm")]
    Warm,
    #[serde(alias = "Cold")]
    Cold,
    #[serde(alias = "Glacier")]
    Glacier,
}

// ---------------------------------------------------------------------------
// EVENT PAYLOAD
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", content = "data")]
#[serde(rename_all = "snake_case")]
// Enum variants now serialize as snake_case; aliases keep deserialization compatible with the prior Rust-style names.
pub enum EventPayload {
    // User events
    #[serde(alias = "UserCreated")]
    UserCreated(UserCreated),
    #[serde(alias = "UserUpdated")]
    UserUpdated(UserUpdated),
    #[serde(alias = "UserDeleted")]
    UserDeleted(UserDeleted),
    #[serde(alias = "UserLoggedIn")]
    UserLoggedIn(UserLoggedIn),
    #[serde(alias = "UserLoggedOut")]
    UserLoggedOut(UserLoggedOut),
    #[serde(alias = "UserPasswordChanged")]
    UserPasswordChanged(UserPasswordChanged),
    #[serde(alias = "UserPasswordReset")]
    UserPasswordReset(UserPasswordReset),
    #[serde(alias = "UserEmailVerified")]
    UserEmailVerified(UserEmailVerified),
    #[serde(alias = "UserMFAEnabled")]
    UserMFAEnabled(UserMFAEnabled),
    #[serde(alias = "UserMFADisabled")]
    UserMFADisabled(UserMFADisabled),
    #[serde(alias = "UserMFARecoveryUsed")]
    UserMFARecoveryUsed(UserMFARecoveryUsed),
    #[serde(alias = "UserPermissionsChanged")]
    UserPermissionsChanged(UserPermissionsChanged),
    #[serde(alias = "UserRoleChanged")]
    UserRoleChanged(UserRoleChanged),
    #[serde(alias = "UserSuspended")]
    UserSuspended(UserSuspended),
    #[serde(alias = "UserReactivated")]
    UserReactivated(UserReactivated),

    // Order events
    #[serde(alias = "OrderCreated")]
    OrderCreated(OrderCreated),
    #[serde(alias = "OrderUpdated")]
    OrderUpdated(OrderUpdated),
    #[serde(alias = "OrderCancelled")]
    OrderCancelled(OrderCancelled),
    #[serde(alias = "OrderFilled")]
    OrderFilled(OrderFilled),
    #[serde(alias = "OrderPartiallyFilled")]
    OrderPartiallyFilled(OrderPartiallyFilled),
    #[serde(alias = "OrderRejected")]
    OrderRejected(OrderRejected),
    #[serde(alias = "OrderExpired")]
    OrderExpired(OrderExpired),
    #[serde(alias = "OrderAmended")]
    OrderAmended(OrderAmended),
    #[serde(alias = "OrderSuspended")]
    OrderSuspended(OrderSuspended),
    #[serde(alias = "OrderResumed")]
    OrderResumed(OrderResumed),

    // Trade events
    #[serde(alias = "TradeExecuted")]
    TradeExecuted(TradeExecuted),
    #[serde(alias = "TradeSettled")]
    TradeSettled(TradeSettled),
    #[serde(alias = "TradeFailed")]
    TradeFailed(TradeFailed),
    #[serde(alias = "TradeDisputed")]
    TradeDisputed(TradeDisputed),
    #[serde(alias = "TradeResolved")]
    TradeResolved(TradeResolved),
    #[serde(alias = "TradeRollback")]
    TradeRollback(TradeRollback),

    // Account events
    #[serde(alias = "AccountCreated")]
    AccountCreated(AccountCreated),
    #[serde(alias = "AccountUpdated")]
    AccountUpdated(AccountUpdated),
    #[serde(alias = "AccountClosed")]
    AccountClosed(AccountClosed),
    #[serde(alias = "AccountFrozen")]
    AccountFrozen(AccountFrozen),
    #[serde(alias = "AccountUnfrozen")]
    AccountUnfrozen(AccountUnfrozen),
    #[serde(alias = "AccountDeposit")]
    AccountDeposit(AccountDeposit),
    #[serde(alias = "AccountWithdrawal")]
    AccountWithdrawal(AccountWithdrawal),
    #[serde(alias = "AccountTransfer")]
    AccountTransfer(AccountTransfer),
    #[serde(alias = "AccountBalanceChanged")]
    AccountBalanceChanged(AccountBalanceChanged),
    #[serde(alias = "AccountMarginCalled")]
    AccountMarginCalled(AccountMarginCalled),
    #[serde(alias = "AccountLiquidated")]
    AccountLiquidated(AccountLiquidated),

    // Market events
    #[serde(alias = "InstrumentAdded")]
    InstrumentAdded(InstrumentAdded),
    #[serde(alias = "InstrumentUpdated")]
    InstrumentUpdated(InstrumentUpdated),
    #[serde(alias = "InstrumentRemoved")]
    InstrumentRemoved(InstrumentRemoved),
    #[serde(alias = "MarketOpened")]
    MarketOpened(MarketOpened),
    #[serde(alias = "MarketClosed")]
    MarketClosed(MarketClosed),
    #[serde(alias = "MarketHalted")]
    MarketHalted(MarketHalted),
    #[serde(alias = "MarketResumed")]
    MarketResumed(MarketResumed),
    #[serde(alias = "CircuitBreakerTriggered")]
    CircuitBreakerTriggered(CircuitBreakerTriggered),
    #[serde(alias = "PriceFeedUpdated")]
    PriceFeedUpdated(PriceFeedUpdated),
    #[serde(alias = "PriceFeedError")]
    PriceFeedError(PriceFeedError),

    // Compliance events
    #[serde(alias = "ComplianceCheckPassed")]
    ComplianceCheckPassed(ComplianceCheckPassed),
    #[serde(alias = "ComplianceCheckFailed")]
    ComplianceCheckFailed(ComplianceCheckFailed),
    #[serde(alias = "ComplianceReviewRequired")]
    ComplianceReviewRequired(ComplianceReviewRequired),
    #[serde(alias = "ComplianceViolation")]
    ComplianceViolation(ComplianceViolation),
    #[serde(alias = "ComplianceReportGenerated")]
    ComplianceReportGenerated(ComplianceReportGenerated),

    // System events
    #[serde(alias = "ServiceStarted")]
    ServiceStarted(ServiceStarted),
    #[serde(alias = "ServiceStopped")]
    ServiceStopped(ServiceStopped),
    #[serde(alias = "ServiceHealthChanged")]
    ServiceHealthChanged(ServiceHealthChanged),
    #[serde(alias = "ConfigChanged")]
    ConfigChanged(ConfigChanged),
    #[serde(alias = "DeploymentStarted")]
    DeploymentStarted(DeploymentStarted),
    #[serde(alias = "DeploymentCompleted")]
    DeploymentCompleted(DeploymentCompleted),
    #[serde(alias = "DeploymentFailed")]
    DeploymentFailed(DeploymentFailed),
    #[serde(alias = "BackupCompleted")]
    BackupCompleted(BackupCompleted),
    #[serde(alias = "BackupFailed")]
    BackupFailed(BackupFailed),
    #[serde(alias = "MaintenanceStarted")]
    MaintenanceStarted(MaintenanceStarted),
    #[serde(alias = "MaintenanceCompleted")]
    MaintenanceCompleted(MaintenanceCompleted),

    // Audit events
    #[serde(alias = "AuditTrailEntry")]
    AuditTrailEntry(AuditTrailEntry),
    #[serde(alias = "DataAccessAudit")]
    DataAccessAudit(DataAccessAudit),
    #[serde(alias = "PermissionChangeAudit")]
    PermissionChangeAudit(PermissionChangeAudit),
    #[serde(alias = "ConfigChangeAudit")]
    ConfigChangeAudit(ConfigChangeAudit),
    #[serde(alias = "SecurityEvent")]
    SecurityEvent(SecurityEvent),

    // Notification events
    #[serde(alias = "NotificationSent")]
    NotificationSent(NotificationSent),
    #[serde(alias = "NotificationDelivered")]
    NotificationDelivered(NotificationDelivered),
    #[serde(alias = "NotificationFailed")]
    NotificationFailed(NotificationFailed),
    #[serde(alias = "NotificationBounced")]
    NotificationBounced(NotificationBounced),
    #[serde(alias = "NotificationClicked")]
    NotificationClicked(NotificationClicked),

    // Analytics events
    #[serde(alias = "PageView")]
    PageView(PageView),
    #[serde(alias = "FeatureUsed")]
    FeatureUsed(FeatureUsed),
    #[serde(alias = "ErrorOccurred")]
    ErrorOccurred(ErrorOccurred),
    #[serde(alias = "PerformanceMetric")]
    PerformanceMetric(PerformanceMetric),
    #[serde(alias = "UserFeedback")]
    UserFeedback(UserFeedback),
    #[serde(alias = "ABTestAssignment")]
    ABTestAssignment(ABTestAssignment),
    #[serde(alias = "ABTestConversion")]
    ABTestConversion(ABTestConversion),

    // Fallback for unknown events
    #[serde(other)]
    Unknown,
}

// ---------------------------------------------------------------------------
// USER EVENTS
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserCreated {
    pub user_id: Uuid,
    pub email: String,
    pub name: String,
    pub signup_method: String,
    pub referral_code: Option<String>,
    pub accepted_terms_version: String,
    pub accepted_privacy_version: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserUpdated {
    pub user_id: Uuid,
    pub changed_fields: Vec<String>,
    pub previous_values: HashMap<String, serde_json::Value>,
    pub new_values: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserDeleted {
    pub user_id: Uuid,
    pub reason: Option<String>,
    pub deletion_type: String,
    pub account_age_days: u64,
    pub data_exported: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserLoggedIn {
    pub user_id: Uuid,
    pub login_method: String,
    pub ip_address: Option<String>,
    pub device_id: Option<String>,
    pub mfa_used: bool,
    pub session_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserLoggedOut {
    pub user_id: Uuid,
    pub session_id: Option<Uuid>,
    pub logout_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPasswordChanged {
    pub user_id: Uuid,
    pub changed_via_reset: bool,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPasswordReset {
    pub user_id: Uuid,
    pub reset_method: String,
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserEmailVerified {
    pub user_id: Uuid,
    pub email: String,
    pub verification_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserMFAEnabled {
    pub user_id: Uuid,
    pub mfa_type: String,
    pub device_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserMFADisabled {
    pub user_id: Uuid,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserMFARecoveryUsed {
    pub user_id: Uuid,
    pub remaining_codes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPermissionsChanged {
    pub user_id: Uuid,
    pub added_permissions: Vec<String>,
    pub removed_permissions: Vec<String>,
    pub changed_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserRoleChanged {
    pub user_id: Uuid,
    pub previous_role: String,
    pub new_role: String,
    pub changed_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserSuspended {
    pub user_id: Uuid,
    pub reason: String,
    pub suspended_by: Uuid,
    pub duration_hours: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserReactivated {
    pub user_id: Uuid,
    pub reactivated_by: Uuid,
    pub reason: Option<String>,
}

// ---------------------------------------------------------------------------
// ORDER EVENTS
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OrderCreated {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub instrument_id: String,
    pub side: String,
    pub order_type: String,
    pub price: Option<f64>,
    pub quantity: f64,
    pub time_in_force: String,
    pub client_order_id: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OrderUpdated {
    pub order_id: Uuid,
    pub changed_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OrderCancelled {
    pub order_id: Uuid,
    pub cancelled_by: String,
    pub reason: Option<String>,
    pub filled_quantity_before_cancel: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OrderFilled {
    pub order_id: Uuid,
    pub fill_price: f64,
    pub fill_quantity: f64,
    pub total_filled_quantity: f64,
    pub remaining_quantity: f64,
    pub trade_id: Uuid,
    pub fees: f64,
    pub fee_currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OrderPartiallyFilled {
    pub order_id: Uuid,
    pub fill_price: f64,
    pub fill_quantity: f64,
    pub cumulative_quantity: f64,
    pub cumulative_fees: f64,
    pub remaining_quantity: f64,
    pub trade_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OrderRejected {
    pub order_id: Uuid,
    pub reason: String,
    pub reject_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OrderExpired {
    pub order_id: Uuid,
    pub unfilled_quantity: f64,
    pub expiry_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OrderAmended {
    pub order_id: Uuid,
    pub previous_quantity: f64,
    pub new_quantity: f64,
    pub previous_price: Option<f64>,
    pub new_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OrderSuspended {
    pub order_id: Uuid,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OrderResumed {
    pub order_id: Uuid,
}

// ---------------------------------------------------------------------------
// TRADE EVENTS
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TradeExecuted {
    pub trade_id: Uuid,
    pub buy_order_id: Uuid,
    pub sell_order_id: Uuid,
    pub instrument_id: String,
    pub price: f64,
    pub quantity: f64,
    pub total: f64,
    pub buyer_user_id: Uuid,
    pub seller_user_id: Uuid,
    pub buyer_account_id: Uuid,
    pub seller_account_id: Uuid,
    pub buyer_fee: f64,
    pub seller_fee: f64,
    pub execution_time: DateTime<Utc>,
    pub liquidation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TradeSettled {
    pub trade_id: Uuid,
    pub settlement_time: DateTime<Utc>,
    pub delivery_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TradeFailed {
    pub trade_id: Uuid,
    pub reason: String,
    pub failure_code: String,
    pub will_retry: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TradeDisputed {
    pub trade_id: Uuid,
    pub disputing_user_id: Uuid,
    pub reason: String,
    pub dispute_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TradeResolved {
    pub trade_id: Uuid,
    pub resolution: String,
    pub resolved_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TradeRollback {
    pub trade_id: Uuid,
    pub reason: String,
    pub affected_orders: Vec<Uuid>,
}

// ---------------------------------------------------------------------------
// ACCOUNT EVENTS
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountCreated {
    pub account_id: Uuid,
    pub user_id: Uuid,
    pub account_type: String,
    pub currency: String,
    pub initial_balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountUpdated {
    pub account_id: Uuid,
    pub changed_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountClosed {
    pub account_id: Uuid,
    pub reason: String,
    pub final_balance: f64,
    pub assets_transferred_to: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountFrozen {
    pub account_id: Uuid,
    pub reason: String,
    pub frozen_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountUnfrozen {
    pub account_id: Uuid,
    pub reason: String,
    pub unfrozen_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountDeposit {
    pub account_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub deposit_method: String,
    pub reference: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountWithdrawal {
    pub account_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub withdrawal_method: String,
    pub reference: String,
    pub status: String,
    pub fee: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountTransfer {
    pub from_account_id: Uuid,
    pub to_account_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub transfer_type: String,
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountBalanceChanged {
    pub account_id: Uuid,
    pub previous_balance: f64,
    pub new_balance: f64,
    pub change_amount: f64,
    pub reason: String,
    pub reference_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountMarginCalled {
    pub account_id: Uuid,
    pub margin_requirement: f64,
    pub account_equity: f64,
    pub margin_shortfall: f64,
    pub deadline: DateTime<Utc>,
    pub positions_affected: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountLiquidated {
    pub account_id: Uuid,
    pub liquidation_reason: String,
    pub positions_liquidated: u32,
    pub total_loss: f64,
    pub remaining_balance: f64,
}

// ---------------------------------------------------------------------------
// MARKET EVENTS
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InstrumentAdded {
    pub instrument_id: String,
    pub symbol: String,
    pub name: String,
    pub instrument_type: String,
    pub exchange: String,
    pub currency: String,
    pub listing_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InstrumentUpdated {
    pub instrument_id: String,
    pub changed_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InstrumentRemoved {
    pub instrument_id: String,
    pub reason: String,
    pub delisting_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarketOpened {
    pub exchange: String,
    pub open_time: DateTime<Utc>,
    pub trading_session: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarketClosed {
    pub exchange: String,
    pub close_time: DateTime<Utc>,
    pub trading_session: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarketHalted {
    pub exchange: String,
    pub instrument_id: Option<String>,
    pub halt_reason: String,
    pub halt_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarketResumed {
    pub exchange: String,
    pub instrument_id: Option<String>,
    pub resume_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CircuitBreakerTriggered {
    pub exchange: String,
    pub trigger_level: String,
    pub price_change_pct: f64,
    pub halt_duration_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PriceFeedUpdated {
    pub instrument_id: String,
    pub source: String,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub volume_24h: f64,
    pub sequence: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PriceFeedError {
    pub instrument_id: String,
    pub source: String,
    pub error: String,
    pub will_retry: bool,
}

// ---------------------------------------------------------------------------
// COMPLIANCE EVENTS
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ComplianceCheckPassed {
    pub check_id: Uuid,
    pub user_id: Uuid,
    pub check_type: String,
    pub reference_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ComplianceCheckFailed {
    pub check_id: Uuid,
    pub user_id: Uuid,
    pub check_type: String,
    pub reference_id: String,
    pub failure_reason: String,
    pub action_required: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ComplianceReviewRequired {
    pub review_id: Uuid,
    pub user_id: Uuid,
    pub reason: String,
    pub assigned_to: Option<Uuid>,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ComplianceViolation {
    pub violation_id: Uuid,
    pub user_id: Uuid,
    pub rule_id: String,
    pub violation_type: String,
    pub severity: String,
    pub details: serde_json::Value,
    pub reported_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ComplianceReportGenerated {
    pub report_id: Uuid,
    pub report_type: String,
    pub jurisdiction: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub record_count: u64,
}

// ---------------------------------------------------------------------------
// SYSTEM EVENTS
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ServiceStarted {
    pub service_name: String,
    pub version: String,
    pub host: String,
    pub pid: u32,
    pub startup_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ServiceStopped {
    pub service_name: String,
    pub reason: String,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ServiceHealthChanged {
    pub service_name: String,
    pub previous_status: String,
    pub new_status: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConfigChanged {
    pub service_name: String,
    pub config_key: String,
    pub previous_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub changed_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeploymentStarted {
    pub deployment_id: Uuid,
    pub service_name: String,
    pub version: String,
    pub strategy: String,
    pub triggered_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeploymentCompleted {
    pub deployment_id: Uuid,
    pub service_name: String,
    pub version: String,
    pub duration_seconds: u64,
    pub healthy_instances: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeploymentFailed {
    pub deployment_id: Uuid,
    pub service_name: String,
    pub version: String,
    pub reason: String,
    pub rollback_initiated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BackupCompleted {
    pub backup_id: Uuid,
    pub backup_type: String,
    pub size_bytes: u64,
    pub duration_seconds: u64,
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BackupFailed {
    pub backup_id: Uuid,
    pub backup_type: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MaintenanceStarted {
    pub maintenance_id: Uuid,
    pub service_name: String,
    pub reason: String,
    pub estimated_duration_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MaintenanceCompleted {
    pub maintenance_id: Uuid,
    pub service_name: String,
    pub actual_duration_minutes: u32,
}

// ---------------------------------------------------------------------------
// AUDIT EVENTS
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditTrailEntry {
    pub audit_id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub changes: serde_json::Value,
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataAccessAudit {
    pub audit_id: Uuid,
    pub user_id: Uuid,
    pub access_type: String,
    pub data_type: String,
    pub data_ids: Vec<String>,
    pub purpose: String,
    pub authorized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PermissionChangeAudit {
    pub audit_id: Uuid,
    pub target_user_id: Uuid,
    pub changed_by: Uuid,
    pub changes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConfigChangeAudit {
    pub audit_id: Uuid,
    pub service_name: String,
    pub config_path: String,
    pub old_value: serde_json::Value,
    pub new_value: serde_json::Value,
    pub changed_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecurityEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub severity: String,
    pub user_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub details: serde_json::Value,
    pub detected_by: String,
}

// ---------------------------------------------------------------------------
// NOTIFICATION EVENTS
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotificationSent {
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub channel: String,
    pub template_id: String,
    pub subject: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotificationDelivered {
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub channel: String,
    pub delivery_time: DateTime<Utc>,
    pub provider_response: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotificationFailed {
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub channel: String,
    pub reason: String,
    pub will_retry: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotificationBounced {
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub channel: String,
    pub bounce_type: String,
    pub bounce_reason: String,
    pub permanent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotificationClicked {
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub channel: String,
    pub target_url: String,
    pub click_time: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// ANALYTICS EVENTS
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PageView {
    pub page_id: String,
    pub user_id: Option<Uuid>,
    pub session_id: Uuid,
    pub url: String,
    pub referrer: Option<String>,
    pub duration_ms: u64,
    pub device_type: String,
    pub browser: String,
    pub os: String,
    pub screen_resolution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeatureUsed {
    pub user_id: Option<Uuid>,
    pub feature_name: String,
    pub feature_group: String,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ErrorOccurred {
    pub error_id: Uuid,
    pub user_id: Option<Uuid>,
    pub error_type: String,
    pub error_message: String,
    pub stack_trace: Option<String>,
    pub component: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PerformanceMetric {
    pub metric_name: String,
    pub metric_value: f64,
    pub unit: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserFeedback {
    pub feedback_id: Uuid,
    pub user_id: Option<Uuid>,
    pub feedback_type: String,
    pub rating: u8,
    pub comment: Option<String>,
    pub page_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ABTestAssignment {
    pub user_id: Uuid,
    pub experiment_id: String,
    pub variant: String,
    pub assignment_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ABTestConversion {
    pub user_id: Uuid,
    pub experiment_id: String,
    pub variant: String,
    pub conversion_event: String,
    pub conversion_time: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// EVENT HELPERS
// ---------------------------------------------------------------------------

impl EventEnvelope {
    pub fn new(
        event_type: impl Into<String>,
        payload: EventPayload,
        metadata: EventMetadata,
        source: impl Into<String>,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type: event_type.into(),
            schema_version: 1,
            payload,
            metadata,
            source: source.into(),
            produced_at: Utc::now(),
            available_at: Utc::now(),
            partition_key: String::new(),
            retention: EventRetention {
                retention_days: 90,
                archive: false,
                storage_tier: StorageTier::Hot,
            },
        }
    }

    pub fn with_partition_key(mut self, key: impl Into<String>) -> Self {
        self.partition_key = key.into();
        self
    }

    pub fn with_retention(mut self, retention: EventRetention) -> Self {
        self.retention = retention;
        self
    }

    pub fn delayed(mut self, delay: chrono::Duration) -> Self {
        self.available_at = Utc::now() + delay;
        self
    }
}

impl EventMetadata {
    pub fn new(trace_id: Uuid, span_id: Uuid) -> Self {
        Self {
            trace_id,
            span_id,
            correlation_id: None,
            user_id: None,
            organization_id: None,
            request_id: None,
            client_ip: None,
            user_agent: None,
            custom: HashMap::new(),
        }
    }
}
