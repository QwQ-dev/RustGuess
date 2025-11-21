use crate::data_handler::{delete_user_data_from_file, get_user_data_from_file};
use crate::user_data::UserData;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};

pub fn get_web_api_router() -> Router {
    Router::new()
        .route(
            "/users/{name}",
            get(get_user).delete(delete_user).put(change_user_data),
        )
        .route("/users", post(create_user))
}

pub async fn start_web_api_service() {
    let router = get_web_api_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Web API started at {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}

async fn get_user(Path(user_name): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let user_data = get_user_data_from_file(&user_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user_data))
}

async fn create_user(Json(user_data): Json<UserData>) -> Result<StatusCode, StatusCode> {
    user_data
        .save()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

async fn delete_user(Path(user_name): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    delete_user_data_from_file(&user_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

async fn change_user_data(
    Path(path_name): Path<String>,
    Json(user_data): Json<UserData>,
) -> Result<impl IntoResponse, StatusCode> {
    // delete and then create
    let user_name = user_data.get_name();

    if path_name != user_name {
        return Err(StatusCode::BAD_REQUEST);
    }

    delete_user_data_from_file(user_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    user_data
        .save()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
