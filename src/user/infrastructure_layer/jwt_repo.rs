use actix_web::web;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;

use crate::{user::domain_layer::user::StaffUser, AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    iss: AuthType,
    pub exp: i64,
    iat: i64,
    role: Issuer,
    pub sub: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthType {
    APIGW,
    Access,
    Refresh,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Issuer {
    Admin,
    User,
    Other,
}

impl AuthType {
    pub fn new(auth_type: &str) -> Self {
        match auth_type {
            "access" => Self::Access,
            "refresh" => Self::Refresh,
            _ => Self::APIGW,
        }
    }
}

impl Issuer {
    pub fn _new(issuer: &str) -> Self {
        match issuer {
            "admin" => Self::Admin,
            "user" => Self::User,
            _ => Self::Other,
        }
    }
}

impl AuthClaims {
    pub fn new(issuer: &str, role: Issuer, auth_type: &str, subject: String) -> Self {
        let iat = Utc::now().timestamp();
        let exp = match auth_type {
            "refresh" => (Utc::now() + Duration::days(30)).timestamp(),
            _ => (Utc::now() + Duration::seconds(300)).timestamp(),
        };

        AuthClaims {
            iss: AuthType::new(auth_type),
            exp,
            iat,
            role,
            sub: subject, // Set the subject field
        }
    }
}

#[allow(dead_code)]
pub async fn create_token(
    user: &StaffUser,
    issuer: &str,
    auth_type: &str,
    app_state: &web::Data<AppState>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let app_state = app_state.get_ref().clone();
    let jwt_secret: String =
        env::var("JWT_SECRET").unwrap_or_else(|_| app_state.jwt_secret.clone());
    let access_secret = jwt_secret.as_bytes();
    let role = if user.username == "arturs" {
        Issuer::Admin
    } else {
        Issuer::User
    };
    let claims = AuthClaims::new(issuer, role, auth_type, user.user_id.unwrap().to_string());
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(access_secret),
    )?;
    Ok(token)
}
pub async fn verify_token(
    token: &str,
    auth_type: &str,
    app_state: &web::Data<AppState>,
) -> Result<TokenData<AuthClaims>, jsonwebtoken::errors::Error> {
    let app_state = app_state.get_ref().clone();
    let jwt_secret: String =
        std::env::var("JWT_SECRET").unwrap_or_else(|_| app_state.jwt_secret.clone());
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());

    // Set up validation for the token
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true; // Ensure token expiration is validated

    // Token type check (access or refresh)
    let mut acceptable_issuers = HashSet::new(); // Create a HashSet for issuers
    if auth_type == "access" {
        acceptable_issuers.insert("Access".to_string());
    } else if auth_type == "refresh" {
        acceptable_issuers.insert("Refresh".to_string());
    }

    validation.iss = Some(acceptable_issuers); // Set the acceptable issuers

    // Decode the token and validate its claims
    let decoded_token = decode::<AuthClaims>(token, &decoding_key, &validation)?;

    Ok(decoded_token)
}
