use crate::user::{
    domain_layer::user::StaffUser,
    infrastructure_layer::{jwt_repo, user_repository::UserRepository},
};
use crate::AppState;
use actix_web::HttpRequest;
use actix_web::{
    cookie::Cookie,
    http::{
        self,
        header::{self, HeaderMap},
    },
    web, HttpResponse, Responder,
};
use chrono::{Duration, Utc};
use serde_json::json;
use uuid::Uuid;

pub async fn get_all_users(state: web::Data<AppState>) -> impl Responder {
    let repo = UserRepository::new();
    match repo.get_all(state.into_inner()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_user_by_id(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
) -> impl Responder {
    let repo = UserRepository::new();
    match repo
        .get_by_id(state.into_inner(), user_id.into_inner())
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn register_user(
    state: web::Data<AppState>,
    user: web::Json<StaffUser>,
) -> impl Responder {
    let repo = UserRepository::new();
    match repo.save(state.into_inner(), user.into_inner()).await {
        Ok(saved_user) => HttpResponse::Ok().json(saved_user),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

pub async fn get_user_full_names(state: web::Data<AppState>) -> impl Responder {
    let repo = UserRepository::new();
    match repo.get_all_user_full_names(state.into_inner()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}


pub async fn update_user(state: web::Data<AppState>, user: web::Json<StaffUser>) -> impl Responder {
    let repo = UserRepository::new();
    match repo.update(state.into_inner(), user.into_inner()).await {
        Ok(updated_user) => HttpResponse::Ok().json(updated_user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn delete_user(state: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let repo = UserRepository::new();
    match repo.delete(state.into_inner(), user_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn login_user(state: web::Data<AppState>, user: web::Json<StaffUser>) -> impl Responder {
    let repo = UserRepository::new();

    // Validate password
    if user.passwd.is_empty() {
        return HttpResponse::BadRequest().json(json!({"error": "Password is required"}));
    }

    // Pass the `Arc<AppState>` by calling `.into_inner()`
    match repo
        .login(state.clone().into_inner(), user.into_inner())
        .await
    {
        Ok(user) => {
            // Create access and refresh tokens
            let access_token_result = jwt_repo::create_token(&user, "user", "access", &state).await;
            let refresh_token_result =
                jwt_repo::create_token(&user, "user", "refresh", &state).await;

            // Check if token creation was successful
            if let (Ok(access_token), Ok(refresh_token)) =
                (access_token_result, refresh_token_result)
            {
                // Create cookies for access and refresh tokens
                let access_cookie = Cookie::build("access_token", access_token)
                    .path("/") // Set path for token availability across the site
                    .http_only(true) // Prevent JavaScript access to the cookie
                    .same_site(actix_web::cookie::SameSite::Lax) // CSRF protection
                    .finish();

                let refresh_cookie = Cookie::build("refresh_token", refresh_token)
                    .path("/")
                    .http_only(true)
                    .same_site(actix_web::cookie::SameSite::Lax)
                    .finish();

                // Return user info and set cookies in the response
                HttpResponse::Ok()
                    .cookie(access_cookie)
                    .cookie(refresh_cookie)
                    .json(json!({"message": "Login successful", "user": user}))
            } else {
                HttpResponse::InternalServerError().json(json!({"error": "Error creating tokens"}))
            }
        }
        Err(e) => {
            // If login fails, return an error
            HttpResponse::Unauthorized().json(json!({"error": e.to_string()}))
        }
    }
}

pub async fn refresh_token(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    let refresh_token_from_body = if let Some(token) =
        body.get("refreshToken").and_then(|v| v.as_str())
    {
        token.to_string()
    } else {
        return HttpResponse::BadRequest().json(json!({"error": "refreshToken missing in body"}));
    };

    match jwt_repo::verify_token(&refresh_token_from_body, "refresh", &state).await {
        Ok(claims) => {
            let user_id = claims.claims.sub;
            let exp = claims.claims.exp;
            if exp < chrono::Utc::now().timestamp() {
                return HttpResponse::Unauthorized()
                    .json(json!({"error": "Refresh token expired"}));
            }

            let repo = UserRepository::new();
            match repo
                .get_by_id(state.clone().into_inner(), user_id.parse().unwrap())
                .await
            {
                Ok(user) => match jwt_repo::create_token(&user, "user", "access", &state).await {
                    Ok(new_access_token) => {
                        let access_cookie = Cookie::build("access_token", new_access_token)
                            .path("/")
                            .http_only(true)
                            .same_site(actix_web::cookie::SameSite::Lax)
                            .finish();

                        HttpResponse::Ok()
                            .cookie(access_cookie)
                            .json(json!({"message": "Access token refreshed successfully"}))
                    }
                    Err(_) => HttpResponse::InternalServerError()
                        .json(json!({"error": "Error creating new access token"})),
                },
                Err(_) => HttpResponse::Unauthorized().json(json!({"error": "User not found"})),
            }
        }
        Err(_) => HttpResponse::Unauthorized().json(json!({"error": "Invalid refresh token"})),
    }
}

pub async fn logout_user(state: web::Data<AppState>) -> impl Responder {
    // Create expired cookies to clear tokens
    let expired_access_cookie = Cookie::build("access_token", "")
        .path("/")
        .http_only(true)
        .same_site(actix_web::cookie::SameSite::Lax)
        .expires(Some(
            time::OffsetDateTime::now_utc() - time::Duration::days(1),
        )) // Set expiration in the past
        .finish();

    let expired_refresh_cookie = Cookie::build("refresh_token", "")
        .path("/")
        .http_only(true)
        .same_site(actix_web::cookie::SameSite::Lax)
        .expires(Some(
            time::OffsetDateTime::now_utc() - time::Duration::days(1),
        )) // Set expiration in the past
        .finish();

    // Return response with expired cookies to remove them from the client
    HttpResponse::Ok()
        .cookie(expired_access_cookie)
        .cookie(expired_refresh_cookie)
        .json(json!({"message": "Logout successful"}))
}
