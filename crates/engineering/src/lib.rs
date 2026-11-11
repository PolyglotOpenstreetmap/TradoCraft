//! Engineering: bills of materials and serial-number traceability.

pub mod bom;
pub mod explode;
pub mod serials;

use tradocraft_core::AppState;
use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter<AppState> {
    bom::router().merge(serials::router())
}
