use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::MartialArtsModel,
    schema::{CreateMartialArtsSchema, FilterOptions, UpdateMartialArtsSchema},
    AppState,
};

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres, and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn ma_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        MartialArtsModel,
        "SELECT * FROM martial_arts ORDER by id LIMIT $1 offset $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all martial arts entries"
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR,Json(error_response)))
    }

    let martialArts = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": martialArts.len(),
        "martial_arts": martialArts
    });
    Ok(Json(json_response))
}

pub async fn create_ma_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateMartialArtsSchema>
) -> Result <impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        MartialArtsModel,
        "INSERT INTO martial_arts (title, category, description, origin, founded_date) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        body.title.to_string(),
        body.category.to_string(),
        body.description.to_string(),
        body.origin.to_owned().unwrap_or("".to_string()),
        body.founded_date.to_owned().unwrap_or("".to_string())
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(martial_art) => {
            let ma_response = json!({"status": "success", "data": json!({
                "martial_art": martial_art
            })});
            
            return Ok((StatusCode::CREATED, Json(ma_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Martial art with that title already exists "
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", e)})),
            ));
        }
    }

}

pub async fn get_ma_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        MartialArtsModel, 
        "SELECT * FROM martial_arts WHERE id = $1",
        id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(martial_art) => {
            let ma_response = json!({"status": "success", "data": json!({
                "martial_art": martial_art
            })});
            
            return Ok(Json(ma_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Martial art with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn edit_ma_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateMartialArtsSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        MartialArtsModel, 
        "SELECT * FROM martial_arts WHERE id = $1",
        id
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Martial art with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND,Json(error_response)))
    }

    let martial_art = query_result.unwrap();

    let query_result = sqlx::query_as!(
        MartialArtsModel,
        "UPDATE martial_arts SET title = $1, category = $2, description = $3, origin = $4, founded_date = $5 WHERE id = $6 RETURNING *",
        body.title.to_owned().unwrap_or(martial_art.title),
        body.category.to_owned().unwrap_or(martial_art.category),
        body.description.to_owned().unwrap_or(martial_art.description),
        body.origin.to_owned().unwrap_or(martial_art.origin.unwrap()),
        body.founded_date.to_owned().unwrap_or(martial_art.founded_date.unwrap()),
        id
    )
    .fetch_all(&data.db)
    .await;

    match query_result {
        Ok(martial_art) => {
            let ma_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "martial_art": martial_art
            })});

            return Ok(Json(ma_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", err)})),
            ));
        }
    }
}

pub async fn delete_ma_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM martial_arts WHERE id = $1", id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Martial art with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)))
    }

    Ok(StatusCode::NO_CONTENT)
}
