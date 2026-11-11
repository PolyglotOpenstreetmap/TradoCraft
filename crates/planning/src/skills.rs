//! Skills catalogue, technicians' skills (level 1-3) and the skills a field job requires.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgConnection};
use tradocraft_core::{new_id, AppError, AppResult, AppState, CurrentUser, Permission};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

use crate::calendar::ensure_member;

type ErrorBody = tradocraft_core::error::ErrorBody;

#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct Skill {
    pub id: Uuid,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SkillInput {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct UserSkill {
    pub skill_id: Uuid,
    pub code: String,
    pub name: String,
    /// 1 = basic, 2 = proficient, 3 = expert.
    pub level: i16,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserSkillInput {
    pub skill_id: Uuid,
    pub level: i16,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserSkillsInput {
    /// Replaces the technician's skills.
    pub skills: Vec<UserSkillInput>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct JobSkillsInput {
    /// Replaces the skills the job requires.
    pub skill_ids: Vec<Uuid>,
}

pub async fn list_skills_inner(conn: &mut PgConnection, company_id: Uuid) -> AppResult<Vec<Skill>> {
    Ok(sqlx::query_as(
        "SELECT id, code, name FROM planning.skills WHERE company_id = $1 ORDER BY name, code",
    )
    .bind(company_id)
    .fetch_all(conn)
    .await?)
}

/// The skills catalogue of the company.
#[utoipa::path(get, path = "/planning/skills", tag = "planning", operation_id = "list_skills",
    responses((status = 200, body = Vec<Skill>), (status = 403, body = ErrorBody)))]
async fn list_skills(
    State(state): State<AppState>,
    user: CurrentUser,
) -> AppResult<Json<Vec<Skill>>> {
    user.require(Permission::PlanningRead)?;
    let mut conn = state.db.acquire().await?;
    Ok(Json(list_skills_inner(&mut conn, user.company_id).await?))
}

fn clean(input: &SkillInput) -> AppResult<(String, String)> {
    let (code, name) = (input.code.trim().to_string(), input.name.trim().to_string());
    if code.is_empty() || name.is_empty() {
        return Err(AppError::Validation(
            "give the skill a code and a name".into(),
        ));
    }
    Ok((code, name))
}

#[utoipa::path(post, path = "/planning/skills", tag = "planning", operation_id = "create_skill",
    request_body = SkillInput,
    responses((status = 201, body = Skill), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody)))]
async fn create_skill(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(input): Json<SkillInput>,
) -> AppResult<(StatusCode, Json<Skill>)> {
    user.require(Permission::PlanningAdmin)?;
    let (code, name) = clean(&input)?;
    let id = new_id();
    sqlx::query("INSERT INTO planning.skills (id, company_id, code, name) VALUES ($1, $2, $3, $4)")
        .bind(id)
        .bind(user.company_id)
        .bind(&code)
        .bind(&name)
        .execute(&state.db)
        .await?;
    Ok((StatusCode::CREATED, Json(Skill { id, code, name })))
}

#[utoipa::path(put, path = "/planning/skills/{id}", tag = "planning", operation_id = "update_skill",
    params(("id" = Uuid, Path)), request_body = SkillInput,
    responses((status = 200, body = Skill), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody)))]
async fn update_skill(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(input): Json<SkillInput>,
) -> AppResult<Json<Skill>> {
    user.require(Permission::PlanningAdmin)?;
    let (code, name) = clean(&input)?;
    let n = sqlx::query(
        "UPDATE planning.skills SET code = $3, name = $4 WHERE id = $1 AND company_id = $2",
    )
    .bind(id)
    .bind(user.company_id)
    .bind(&code)
    .bind(&name)
    .execute(&state.db)
    .await?
    .rows_affected();
    if n == 0 {
        return Err(AppError::NotFound("skill"));
    }
    Ok(Json(Skill { id, code, name }))
}

/// Deleting a skill removes it from technicians and jobs.
#[utoipa::path(delete, path = "/planning/skills/{id}", tag = "planning", operation_id = "delete_skill",
    params(("id" = Uuid, Path)), responses((status = 204), (status = 404, body = ErrorBody)))]
async fn delete_skill(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    user.require(Permission::PlanningAdmin)?;
    let n = sqlx::query("DELETE FROM planning.skills WHERE id = $1 AND company_id = $2")
        .bind(id)
        .bind(user.company_id)
        .execute(&state.db)
        .await?
        .rows_affected();
    if n == 0 {
        return Err(AppError::NotFound("skill"));
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn user_skills_of(
    conn: &mut PgConnection,
    company_id: Uuid,
    user_id: Uuid,
) -> AppResult<Vec<UserSkill>> {
    Ok(sqlx::query_as(
        "SELECT s.id AS skill_id, s.code, s.name, us.level FROM planning.user_skills us
         JOIN planning.skills s ON s.id = us.skill_id
         WHERE us.company_id = $1 AND us.user_id = $2 ORDER BY s.name, s.code",
    )
    .bind(company_id)
    .bind(user_id)
    .fetch_all(conn)
    .await?)
}

/// The skills of a technician (planners, or the technician themselves).
#[utoipa::path(get, path = "/planning/users/{id}/skills", tag = "planning", operation_id = "get_user_skills",
    params(("id" = Uuid, Path)), responses((status = 200, body = Vec<UserSkill>), (status = 403, body = ErrorBody)))]
async fn get_user_skills(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<UserSkill>>> {
    if id != user.user_id {
        user.require(Permission::PlanningRead)?;
    }
    let mut conn = state.db.acquire().await?;
    Ok(Json(user_skills_of(&mut conn, user.company_id, id).await?))
}

/// Skills of this company among `ids`, or a validation error naming that one is unknown.
async fn ensure_skills(conn: &mut PgConnection, company_id: Uuid, ids: &[Uuid]) -> AppResult<()> {
    let n: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM planning.skills WHERE company_id = $1 AND id = ANY($2)",
    )
    .bind(company_id)
    .bind(ids)
    .fetch_one(conn)
    .await?;
    if n as usize != ids.len() {
        return Err(AppError::Validation("unknown skill".into()));
    }
    Ok(())
}

#[utoipa::path(put, path = "/planning/users/{id}/skills", tag = "planning", operation_id = "set_user_skills",
    params(("id" = Uuid, Path)), request_body = UserSkillsInput,
    responses((status = 200, body = Vec<UserSkill>), (status = 403, body = ErrorBody), (status = 422, body = ErrorBody)))]
async fn set_user_skills(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(input): Json<UserSkillsInput>,
) -> AppResult<Json<Vec<UserSkill>>> {
    user.require(Permission::PlanningAdmin)?;
    let mut ids: Vec<Uuid> = input.skills.iter().map(|s| s.skill_id).collect();
    ids.sort();
    ids.dedup();
    if ids.len() != input.skills.len() {
        return Err(AppError::Validation("a skill is listed twice".into()));
    }
    if input.skills.iter().any(|s| !(1..=3).contains(&s.level)) {
        return Err(AppError::Validation("the level must be 1 to 3".into()));
    }
    let mut tx = state.db.begin().await?;
    ensure_member(&mut tx, user.company_id, id).await?;
    ensure_skills(&mut tx, user.company_id, &ids).await?;
    sqlx::query("DELETE FROM planning.user_skills WHERE company_id = $1 AND user_id = $2")
        .bind(user.company_id)
        .bind(id)
        .execute(&mut *tx)
        .await?;
    for s in &input.skills {
        sqlx::query("INSERT INTO planning.user_skills (company_id, user_id, skill_id, level) VALUES ($1, $2, $3, $4)")
            .bind(user.company_id)
            .bind(id)
            .bind(s.skill_id)
            .bind(s.level)
            .execute(&mut *tx)
            .await?;
    }
    let out = user_skills_of(&mut tx, user.company_id, id).await?;
    tx.commit().await?;
    Ok(Json(out))
}

async fn ensure_job(conn: &mut PgConnection, company_id: Uuid, job_id: Uuid) -> AppResult<()> {
    let ok: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM field.jobs WHERE id = $1 AND company_id = $2)",
    )
    .bind(job_id)
    .bind(company_id)
    .fetch_one(conn)
    .await?;
    if ok {
        Ok(())
    } else {
        Err(AppError::NotFound("job"))
    }
}

async fn job_skills_of(
    conn: &mut PgConnection,
    company_id: Uuid,
    job_id: Uuid,
) -> AppResult<Vec<Skill>> {
    Ok(sqlx::query_as(
        "SELECT s.id, s.code, s.name FROM field.job_required_skills r JOIN planning.skills s ON s.id = r.skill_id
         WHERE r.company_id = $1 AND r.job_id = $2 ORDER BY s.name, s.code",
    )
    .bind(company_id)
    .bind(job_id)
    .fetch_all(conn)
    .await?)
}

/// The skills a field job requires.
#[utoipa::path(get, path = "/planning/jobs/{id}/skills", tag = "planning", operation_id = "get_job_skills",
    params(("id" = Uuid, Path)), responses((status = 200, body = Vec<Skill>), (status = 404, body = ErrorBody)))]
async fn get_job_skills(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<Skill>>> {
    user.require(Permission::PlanningRead)?;
    let mut conn = state.db.acquire().await?;
    ensure_job(&mut conn, user.company_id, id).await?;
    Ok(Json(job_skills_of(&mut conn, user.company_id, id).await?))
}

#[utoipa::path(put, path = "/planning/jobs/{id}/skills", tag = "planning", operation_id = "set_job_skills",
    params(("id" = Uuid, Path)), request_body = JobSkillsInput,
    responses((status = 200, body = Vec<Skill>), (status = 404, body = ErrorBody), (status = 422, body = ErrorBody)))]
async fn set_job_skills(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(input): Json<JobSkillsInput>,
) -> AppResult<Json<Vec<Skill>>> {
    user.require(Permission::PlanningWrite)?;
    let mut ids = input.skill_ids.clone();
    ids.sort();
    ids.dedup();
    let mut tx = state.db.begin().await?;
    ensure_job(&mut tx, user.company_id, id).await?;
    ensure_skills(&mut tx, user.company_id, &ids).await?;
    sqlx::query("DELETE FROM field.job_required_skills WHERE job_id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    for s in &ids {
        sqlx::query("INSERT INTO field.job_required_skills (company_id, job_id, skill_id) VALUES ($1, $2, $3)")
            .bind(user.company_id)
            .bind(id)
            .bind(s)
            .execute(&mut *tx)
            .await?;
    }
    let out = job_skills_of(&mut tx, user.company_id, id).await?;
    tx.commit().await?;
    Ok(Json(out))
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_skills, create_skill))
        .routes(routes!(update_skill, delete_skill))
        .routes(routes!(get_user_skills, set_user_skills))
        .routes(routes!(get_job_skills, set_job_skills))
}
