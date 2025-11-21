use crate::cleanup::user_data_cleanup_guard::UserDataCleanupGuard;
use axum::http::{Method, Request, StatusCode};
use axum::Router;
use http_body_util::{BodyExt, Full};
use rust_guess::boxed_error::BoxedError;
use rust_guess::user_data::UserData;
use rust_guess::web::web_api::get_web_api_router;
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
async fn test_web_api() {
    let app = get_web_api_router();

    let user_name = Uuid::new_v4().to_string();
    let mut initial_user_data = UserData::new(user_name.clone(), 5, 2);

    let _guard = UserDataCleanupGuard {
        user_name: user_name.clone(),
    };

    test_get(&app, &initial_user_data).await;
    test_post(&app, &initial_user_data).await;

    // win++
    initial_user_data.record_win();

    test_put(&app, &user_name, &initial_user_data).await;
    test_delete(&app, &user_name).await;
}

async fn test_get(app: &Router, user_info: &UserData) {
    let user_name = user_info.get_name();

    let create_request = Request::builder()
        .method(Method::POST)
        .uri("/users")
        .header("content-type", "application/json")
        .body(Full::from(serde_json::to_vec(&user_info).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(create_request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "POST /users failed: {:?}",
        response.status()
    );

    let get_request = Request::builder()
        .method(Method::GET)
        .uri(format!("/users/{}", user_name))
        .body(Full::from(vec![]))
        .unwrap();

    let response = app.clone().oneshot(get_request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "GET /users/{} failed: {:?}",
        user_name,
        response.status()
    );

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let received_user: UserData = serde_json::from_slice(&body)
        .map_err(BoxedError::from)
        .unwrap();

    assert_eq!(received_user.wins, 5, "Retrieved user data mismatch (wins)");
    assert_eq!(
        received_user.losses, 2,
        "Retrieved user data mismatch (losses)"
    );

    let get_nonexistent_request = Request::builder()
        .method(Method::GET)
        .uri("/users/DoesNotExist")
        .body(Full::from(vec![]))
        .unwrap();

    let response = app.clone().oneshot(get_nonexistent_request).await.unwrap();

    assert_eq!(
        response.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "GET non-existent user should return 500"
    );
}

async fn test_post(app: &Router, user_info: &UserData) {
    let create_request = Request::builder()
        .method(Method::POST)
        .uri("/users")
        .header("content-type", "application/json")
        .body(Full::from(serde_json::to_vec(user_info).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(create_request).await.unwrap();

    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "POST /users failed: {:?}",
        response.status()
    );
}

async fn test_delete(app: &Router, user_name: &str) {
    let get_nonexistent_request = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/users/{}", user_name))
        .body(Full::from(vec![]))
        .unwrap();

    let response = app.clone().oneshot(get_nonexistent_request).await.unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "DELETE /users/{} failed: {:?}",
        user_name,
        response.status()
    );
}

async fn test_put(app: &Router, user_name: &str, updated_data: &UserData) {
    let put_request = Request::builder()
        .method(Method::PUT)
        .uri(format!("/users/{}", user_name))
        .header("content-type", "application/json")
        .body(Full::from(serde_json::to_vec(&updated_data).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(put_request).await.unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "PUT /users/{} failed: {:?}",
        user_name,
        response.status()
    );

    let get_request = Request::builder()
        .method(Method::GET)
        .uri(format!("/users/{}", user_name))
        .body(Full::from(vec![]))
        .unwrap();

    let response = app.clone().oneshot(get_request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "GET /users/{} after PUT failed: {:?}",
        user_name,
        response.status()
    );

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let received_user: UserData =
        serde_json::from_slice(&body).expect("Failed to deserialize user data after PUT");

    assert_eq!(received_user.wins, 6, "Updated wins mismatch");
    assert_eq!(received_user.losses, 2, "Updated losses mismatch");

    let different_name = "different_user";
    let bad_put_request = Request::builder()
        .method(Method::PUT)
        .uri(format!("/users/{}", different_name))
        .header("content-type", "application/json")
        .body(Full::from(serde_json::to_vec(&updated_data).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(bad_put_request).await.unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "PUT /users/{}/{} should return 400",
        different_name,
        user_name
    );
}
