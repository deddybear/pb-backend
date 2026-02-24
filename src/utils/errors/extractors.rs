use axum::{
    extract::{
        rejection::{JsonRejection, PathRejection, QueryRejection},
        FromRequest, FromRequestParts, Path, Query, Request,
    },
    http::request::Parts,
};
use serde::de::DeserializeOwned;
use crate::utils::errors::AppError;

// ─── AppJson ─────────────────────────────────────────────────────────────────
/// Pengganti `axum::Json` yang konversi rejection ke `AppError` secara otomatis.
///
/// Gunakan `AppJson` di handler agar error parsing JSON langsung jadi response
/// yang konsisten tanpa perlu `map_err` manual.
///
/// ```rust
/// pub async fn create(AppJson(body): AppJson<CreatePostRequest>) -> AppResult<...>
/// ```
pub struct AppJson<T>(pub T);


impl<T, S> FromRequest<S> for AppJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let axum::Json(value) = axum::Json::<T>::from_request(req, state).await?;
        Ok(AppJson(value))
    }
}

// ─── AppPath ─────────────────────────────────────────────────────────────────
/// Pengganti `axum::extract::Path` yang konversi rejection ke `AppError`.
///
/// Jika UUID di path tidak valid atau tipe tidak cocok, langsung return 400
/// dengan pesan yang jelas.
///
/// ```rust
/// pub async fn get_by_id(AppPath(id): AppPath<Uuid>) -> AppResult<...>
/// ```
pub struct AppPath<T>(pub T);


impl<T, S> FromRequestParts<S> for AppPath<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
    Path<T>: FromRequestParts<S, Rejection = PathRejection>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(value) = Path::<T>::from_request_parts(parts, state).await?;
        Ok(AppPath(value))
    }
}

// ─── AppQuery ────────────────────────────────────────────────────────────────
/// Pengganti `axum::extract::Query` yang konversi rejection ke `AppError`.
///
/// Jika query string tidak bisa di-parse, langsung return 400 dengan pesan jelas.
///
/// ```rust
/// pub async fn get_all(AppQuery(q): AppQuery<PostQuery>) -> AppResult<...>
/// ```
pub struct AppQuery<T>(pub T);


impl<T, S> FromRequestParts<S> for AppQuery<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
    Query<T>: FromRequestParts<S, Rejection = QueryRejection>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request_parts(parts, state).await?;
        Ok(AppQuery(value))
    }
}
