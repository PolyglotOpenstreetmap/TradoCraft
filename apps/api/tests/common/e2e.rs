//! Shared E2E helpers: `E2eFlow`, test-customer constants, and convenience wrappers.
//!
//! This is a submodule of `common`; it reaches the shared HTTP helpers (`call`,
//! `call_raw`) through `super`. Every E2E test file pulls these helpers in with
//! `use common::e2e::*;`.
#![allow(dead_code)]

use axum::http::{Method, StatusCode};
use axum::Router;
use rust_decimal::Decimal;
use serde_json::Value;

use super::{call, call_raw};

/// Accumulates identifiers produced by one E2E step and consumed by the next.
/// Fields are `Option` when they are set by a later step; `None` means "not yet reached".
pub struct E2eFlow {
    // --- Customer (set by step_register_customer) ---
    pub customer_id: String,
    pub customer_name: String,
    pub customer_email: String,    // e.g. "info@nordic.example"
    pub customer_language: String, // "en"
    pub customer_country: String,  // "NO"

    // --- Ticket (set by step_import_enquiry_email) ---
    pub ticket_id: Option<String>,
    pub ticket_number: Option<String>,

    // --- Split ticket (set by step_split_ticket) ---
    pub split_ticket_id: Option<String>,

    // --- Quotation (set by step_create_and_send_quotation) ---
    pub quotation_id: Option<String>,
    pub quotation_number: Option<String>,
    pub quotation_revision: Option<i32>,
    pub quotation_total: Option<Decimal>,

    // --- Order (set by step_convert_to_order) ---
    pub order_id: Option<String>,
    pub order_number: Option<String>,
    pub order_total: Option<Decimal>,

    // --- Serial unit (used by traceability/service tests) ---
    pub serial_unit_id: Option<String>,
}

impl E2eFlow {
    /// A fresh flow: empty customer strings and every later-produced identifier unset.
    pub fn new() -> Self {
        Self {
            customer_id: String::new(),
            customer_name: String::new(),
            customer_email: String::new(),
            customer_language: String::new(),
            customer_country: String::new(),
            ticket_id: None,
            ticket_number: None,
            split_ticket_id: None,
            quotation_id: None,
            quotation_number: None,
            quotation_revision: None,
            quotation_total: None,
            order_id: None,
            order_number: None,
            order_total: None,
            serial_unit_id: None,
        }
    }
}

// --- Test customer: "Nordic Industrial Systems" ---
// Registered via the API in the first E2E step (not seeded). Norway (NO) has a
// 230V/400V 50Hz grid in the seed data, so the power-grid compatibility check works
// out of the box; emails from `info@nordic.example` match this customer by domain.
pub const TEST_CUSTOMER_NAME: &str = "Nordic Industrial Systems";
pub const TEST_CUSTOMER_DOMAIN: &str = "nordic.example";
pub const TEST_CUSTOMER_EMAIL: &str = "info@nordic.example";
pub const TEST_CUSTOMER_COUNTRY: &str = "NO"; // Norway — 230V/400V 50Hz grid
pub const TEST_CUSTOMER_LANGUAGE: &str = "en";

/// Imports a raw RFC 822 message into the helpdesk (`POST /api/v1/helpdesk/inbound`).
/// Asserts the import succeeded and returns the response body (e.g. `new_ticket`,
/// `duplicate`, `ticket_id`). Mirrors the local `import()` helper in `helpdesk.rs`,
/// but takes the staff user as a parameter so any staff role can drive it.
pub async fn import_email(app: &Router, user: &str, raw_email: Vec<u8>) -> Value {
    let (status, r) = call_raw(
        app,
        user,
        "/api/v1/helpdesk/inbound",
        "message/rfc822",
        raw_email,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    r
}

/// Fetches a ticket by id (`GET /api/v1/tickets/{id}`), asserting it was found.
/// Mirrors the local `ticket()` helper in `helpdesk.rs` with a parameterized user.
pub async fn get_ticket(app: &Router, user: &str, id: &str) -> Value {
    let (status, t) = call(
        app,
        user,
        Method::GET,
        &format!("/api/v1/tickets/{id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{t}");
    t
}

/// Builds a raw RFC 822 message, mirroring the `mail()` helper in `helpdesk.rs`.
/// `extra_headers` is inserted verbatim before the `Content-Type` header, so callers
/// can pass `Message-ID`/`In-Reply-To` headers (with a trailing `\r\n`) for reply
/// threading, e.g. `build_email(from, subject, body, "Message-ID: <in-1@x>\r\n")`.
pub fn build_email(from: &str, subject: &str, body: &str, extra_headers: &str) -> Vec<u8> {
    format!(
        "From: {from}\r\nTo: sales@example.com\r\nSubject: {subject}\r\n{extra_headers}Content-Type: text/plain; charset=utf-8\r\n\r\n{body}\r\n"
    )
    .into_bytes()
}
