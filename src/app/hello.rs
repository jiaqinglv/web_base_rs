use axum::{Json, extract::Path};

pub async fn hello_world() -> String {
    "hello world".to_string()
}

pub async fn name(Path(user_id) : Path<String>) -> Json<String> {
    let user = user_id;
    Json(user)
}



