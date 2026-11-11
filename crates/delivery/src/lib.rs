//! Delivery: delivery notes for sales orders (ADR 0035).
//!
//! A delivery note plans what leaves for a customer and when: lines picked from the stock that is
//! reserved for the order, a planned date, a carrier. Shipping it takes the stock out through the
//! inventory ledger (consuming the reservations) and moves the order to `partially_shipped` or
//! `shipped`. Planning a date on a day the customer is closed follows the company's closure
//! policy (see [`settings`]).

pub mod notes;
pub mod pdf;
pub mod settings;

use tradocraft_core::AppState;
use utoipa_axum::router::OpenApiRouter;

/// Builds the delivery router (settings and delivery notes).
pub fn router() -> OpenApiRouter<AppState> {
    settings::router()
        .merge(notes::router())
        .merge(pdf::router())
}
