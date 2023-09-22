use crate::database::tasks;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter,
    TransactionTrait,
};
use sea_orm::{DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestTask {
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

pub async fn create_task(
    State(database): State<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) {
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        ..Default::default()
    };
    let x = new_task.save(&database).await.unwrap();

    dbg!(x);
}

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

pub async fn get_task(
    State(database): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = tasks::Entity::find_by_id(id)
        .one(&database)
        .await
        .unwrap()
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ResponseTask {
        id: task.id,
        title: task.title,
        priority: task.priority,
        description: task.description,
    }))
}

#[derive(Deserialize)]
pub struct GetTaskQueryParams {
    priority: Option<String>,
}

pub async fn get_all_tasks(
    State(database): State<DatabaseConnection>,
    Query(params): Query<GetTaskQueryParams>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let mut filter = Condition::all();
    if let Some(priority) = params.priority {
        if priority.is_empty() {
            filter = filter.add(tasks::Column::Priority.is_null());
        } else {
            filter = filter.add(tasks::Column::Priority.eq(priority));
        }
    }

    Ok(Json(
        tasks::Entity::find()
            .filter(filter)
            .all(&database)
            .await
            .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
            .into_iter()
            .map(move |x| ResponseTask {
                id: x.id,
                title: x.title,
                priority: x.priority,
                description: x.description,
            })
            .collect(),
    ))
}

#[derive(Deserialize)]
pub struct UpdateTask {
    priority: Option<String>,
    title: String,
    completed_at: Option<DateTimeWithTimeZone>,
    description: Option<String>,
    deleted_at: Option<DateTimeWithTimeZone>,
    user_id: Option<i32>,
    is_default: Option<bool>,
}

pub async fn atomic_update(
    Path(id): Path<i32>,
    State(database): State<DatabaseConnection>,
    Json(update_task): Json<UpdateTask>,
) -> Result<(), StatusCode> {
    let task = tasks::ActiveModel {
        id: Set(id),
        priority: Set(update_task.priority),
        title: Set(update_task.title),
        completed_at: Set(update_task.completed_at),
        description: Set(update_task.description),
        deleted_at: Set(update_task.deleted_at),
        user_id: Set(update_task.user_id),
        is_default: Set(update_task.is_default),
    };

    tasks::Entity::update(task)
        .filter(tasks::Column::Id.eq(id))
        .exec(&database)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

#[derive(Deserialize)]
pub struct PatchTask {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    priority: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    description: Option<Option<String>>,
}

pub async fn partial_update(
    Path(id): Path<i32>,
    State(database): State<DatabaseConnection>,
    Json(patch_task): Json<PatchTask>,
) -> Result<(), StatusCode> {
    let txn = database
        .begin()
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut task = tasks::Entity::find_by_id(id)
        .one(&txn)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into_active_model();

    if let Some(priority) = patch_task.priority {
        task.priority = Set(priority);
    }

    if let Some(completed_at) = patch_task.completed_at {
        task.completed_at = Set(completed_at);
    }

    if let Some(description) = patch_task.description {
        task.description = Set(description);
    }

    tasks::Entity::update(task)
        .filter(tasks::Column::Id.eq(id))
        .exec(&txn)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    txn.commit()
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

pub async fn delete_task(
    Path(id): Path<i32>,
    State(database): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let affected_rows = tasks::Entity::delete_by_id(id)
        .exec(&database)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected;

    if affected_rows == 1 {
        Ok(())
    } else if affected_rows == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
