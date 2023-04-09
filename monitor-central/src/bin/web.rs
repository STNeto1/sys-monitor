use std::net::SocketAddr;

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing, Json, Router,
};
use dotenv::dotenv;
use monitor_central::{get_from_env, DBStat};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Debug, Clone)]
struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect("No .env file found");

    let app_state = AppState {
        db: PgPoolOptions::new()
            .max_connections(5)
            .connect(&get_from_env("DATABASE_URL"))
            .await
            .map_err(|e| anyhow::anyhow!(e))?,
    };

    let app = Router::new()
        .route("/stats", routing::get(get_stats_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    return Ok(());
}

async fn get_stats_handler(State(state): State<AppState>) -> Result<Json<Vec<DBStat>>, AppError> {
    let result = sqlx::query_as::<_, DBStat>("SELECT * FROM stats")
        .fetch_all(&state.db)
        .await
        .map_err(|e| AppError(anyhow::anyhow!(e)))?;

    return Ok(Json(result));
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
