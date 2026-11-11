//! Workshop departments (electronics, electric, mechanical workshop, ...). The site admin creates
//! and changes them and says who belongs to each; repairs are routed to them.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgConnection};
use tradocraft_core::{audit, new_id, AppError, AppResult, AppState, CurrentUser, Permission};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

type ErrorBody = tradocraft_core::error::ErrorBody;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Member {
    pub user_id: Uuid,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Department {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub sort_order: i32,
    pub active: bool,
    pub members: Vec<Member>,
    /// Repairs currently in this department (routed, in progress or waiting).
    pub open_repairs: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DepartmentInput {
    /// Short unique code, e.g. `ELEC`.
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub sort_order: i32,
    #[serde(default = "default_true")]
    pub active: bool,
    /// The colleagues who work in this department and may work on its repairs.
    #[serde(default)]
    pub member_user_ids: Vec<Uuid>,
}

fn default_true() -> bool {
    true
}

#[derive(FromRow)]
struct Row {
    id: Uuid,
    code: String,
    name: String,
    sort_order: i32,
    active: bool,
    open_repairs: i64,
}

async fn load_all(
    conn: &mut PgConnection,
    company_id: Uuid,
    only: Option<Uuid>,
) -> AppResult<Vec<Department>> {
    let rows: Vec<Row> = sqlx::query_as(
        "SELECT d.id, d.code, d.name, d.sort_order, d.active,
                (SELECT count(*) FROM repairs.repairs r WHERE r.department_id = d.id
                   AND r.status IN ('routed', 'in_progress', 'waiting_customer', 'waiting_parts')) AS open_repairs
         FROM repairs.departments d
         WHERE d.company_id = $1 AND ($2::uuid IS NULL OR d.id = $2)
         ORDER BY d.sort_order, d.name",
    )
    .bind(company_id)
    .bind(only)
    .fetch_all(&mut *conn)
    .await?;
    let ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
    let members: Vec<(Uuid, Uuid, String)> = sqlx::query_as(
        "SELECT m.department_id, u.id, u.display_name FROM repairs.department_members m
         JOIN core.users u ON u.id = m.user_id WHERE m.department_id = ANY($1) ORDER BY u.display_name",
    )
    .bind(&ids)
    .fetch_all(conn)
    .await?;
    Ok(rows
        .into_iter()
        .map(|r| Department {
            members: members
                .iter()
                .filter(|(d, _, _)| *d == r.id)
                .map(|(_, user_id, display_name)| Member {
                    user_id: *user_id,
                    display_name: display_name.clone(),
                })
                .collect(),
            id: r.id,
            code: r.code,
            name: r.name,
            sort_order: r.sort_order,
            active: r.active,
            open_repairs: r.open_repairs,
        })
        .collect())
}

/// Whether `user_id` works in the department.
pub async fn is_member(
    conn: &mut PgConnection,
    department_id: Uuid,
    user_id: Uuid,
) -> AppResult<bool> {
    Ok(sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM repairs.department_members WHERE department_id = $1 AND user_id = $2)",
    )
    .bind(department_id)
    .bind(user_id)
    .fetch_one(conn)
    .await?)
}

impl DepartmentInput {
    fn check(&self) -> AppResult<()> {
        if self.code.trim().is_empty() || self.name.trim().is_empty() {
            return Err(AppError::Validation("code and name are required".into()));
        }
        if self.code.trim().len() > 20 || self.name.trim().len() > 80 {
            return Err(AppError::Validation("code or name is too long".into()));
        }
        Ok(())
    }
}

async fn set_members(
    conn: &mut PgConnection,
    company_id: Uuid,
    id: Uuid,
    members: &[Uuid],
) -> AppResult<()> {
    let valid: i64 = sqlx::query_scalar(
        "SELECT count(DISTINCT user_id) FROM core.user_companies WHERE company_id = $1 AND user_id = ANY($2)",
    )
    .bind(company_id)
    .bind(members)
    .fetch_one(&mut *conn)
    .await?;
    let mut unique = members.to_vec();
    unique.sort();
    unique.dedup();
    if valid as usize != unique.len() {
        return Err(AppError::Validation(
            "a member is not a colleague in this company".into(),
        ));
    }
    sqlx::query("DELETE FROM repairs.department_members WHERE department_id = $1")
        .bind(id)
        .execute(&mut *conn)
        .await?;
    for u in unique {
        sqlx::query(
            "INSERT INTO repairs.department_members (department_id, user_id) VALUES ($1, $2)",
        )
        .bind(id)
        .bind(u)
        .execute(&mut *conn)
        .await?;
    }
    Ok(())
}

#[utoipa::path(get, path = "/departments", tag = "repairs", operation_id = "list_departments",
    responses((status = 200, body = Vec<Department>)))]
async fn list_departments(
    State(state): State<AppState>,
    user: CurrentUser,
) -> AppResult<Json<Vec<Department>>> {
    user.require(Permission::RepairsRead)?;
    let mut conn = state.db.acquire().await?;
    Ok(Json(load_all(&mut conn, user.company_id, None).await?))
}

#[utoipa::path(post, path = "/departments", tag = "repairs", operation_id = "create_department", request_body = DepartmentInput,
    responses((status = 201, body = Department), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody)))]
async fn create_department(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(input): Json<DepartmentInput>,
) -> AppResult<(StatusCode, Json<Department>)> {
    user.require(Permission::DepartmentsWrite)?;
    input.check()?;
    let mut tx = state.db.begin().await?;
    let id = new_id();
    sqlx::query(
        "INSERT INTO repairs.departments (id, company_id, code, name, sort_order, active) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(id)
    .bind(user.company_id)
    .bind(input.code.trim().to_uppercase())
    .bind(input.name.trim())
    .bind(input.sort_order)
    .bind(input.active)
    .execute(&mut *tx)
    .await
    .map_err(|e| match AppError::from(e) {
        AppError::Conflict(_) => AppError::Conflict(format!("department {} already exists", input.code.trim().to_uppercase())),
        other => other,
    })?;
    set_members(&mut tx, user.company_id, id, &input.member_user_ids).await?;
    audit::record(
        &mut tx,
        &user,
        "department",
        id,
        "created",
        serde_json::json!({ "code": input.code }),
    )
    .await?;
    let d = load_all(&mut tx, user.company_id, Some(id))
        .await?
        .remove(0);
    tx.commit().await?;
    Ok((StatusCode::CREATED, Json(d)))
}

#[utoipa::path(put, path = "/departments/{id}", tag = "repairs", operation_id = "update_department",
    params(("id" = Uuid, Path)), request_body = DepartmentInput,
    responses((status = 200, body = Department), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody)))]
async fn update_department(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(input): Json<DepartmentInput>,
) -> AppResult<Json<Department>> {
    user.require(Permission::DepartmentsWrite)?;
    input.check()?;
    let mut tx = state.db.begin().await?;
    let current = load_all(&mut tx, user.company_id, Some(id))
        .await?
        .into_iter()
        .next()
        .ok_or(AppError::NotFound("department"))?;
    if current.active && !input.active && current.open_repairs > 0 {
        return Err(AppError::Conflict(format!(
            "{} repairs are still in this department: route them elsewhere first",
            current.open_repairs
        )));
    }
    sqlx::query("UPDATE repairs.departments SET code = $2, name = $3, sort_order = $4, active = $5 WHERE id = $1")
        .bind(id)
        .bind(input.code.trim().to_uppercase())
        .bind(input.name.trim())
        .bind(input.sort_order)
        .bind(input.active)
        .execute(&mut *tx)
        .await?;
    set_members(&mut tx, user.company_id, id, &input.member_user_ids).await?;
    audit::record(
        &mut tx,
        &user,
        "department",
        id,
        "updated",
        serde_json::json!({ "active": input.active }),
    )
    .await?;
    let d = load_all(&mut tx, user.company_id, Some(id))
        .await?
        .remove(0);
    tx.commit().await?;
    Ok(Json(d))
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_departments, create_department))
        .routes(routes!(update_department))
}
