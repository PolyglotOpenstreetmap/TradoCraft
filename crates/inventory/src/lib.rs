//! Inventory: warehouses and locations (including service vans), units of measure, lots, the
//! immutable stock move ledger with its balance projection, reservations for sales orders and
//! stock counts. Design: [ADR 0019](../../docs/adr/0019-inventory-stock-ledger.md).
//!
//! Other modules post stock changes with [`ledger::post_move`] in their own transaction; they
//! never write the inventory tables themselves.

pub mod counts;
pub mod ledger;
pub mod moves;
pub mod reservations;
pub mod setup;
pub mod stock;

pub use ledger::{
    check_balances, post_move, rebuild_balances, reverse_move, virtual_location, NewMove, Posted,
    SerialEffect, StockMove,
};
pub use reservations::release_for_order;

use tradocraft_core::AppState;
use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter<AppState> {
    setup::router()
        .merge(moves::router())
        .merge(stock::router())
        .merge(reservations::router())
        .merge(counts::router())
}
