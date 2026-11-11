//! Production: production orders built from the BOM (ADR 0036).
//!
//! A planner releases a confirmed order (or builds for stock): the active BOM is frozen on a
//! production order, its components are reserved, and completing the order consumes them
//! (backflush) and puts the output into stock. The sales order follows: `in_planning`, `planned`,
//! `in_production`, `ready_for_shipping`, with a promised date.

pub mod complete;
pub mod divisions;
pub mod orders;

use tradocraft_core::AppState;
use utoipa_axum::router::OpenApiRouter;

/// Builds the production router (divisions and production orders).
pub fn router() -> OpenApiRouter<AppState> {
    divisions::router()
        .merge(orders::router())
        .merge(complete::router())
}
