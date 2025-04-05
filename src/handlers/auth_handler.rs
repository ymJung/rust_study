use actix_web::{web, HttpResponse, Responder};
use crate::models::user::{CreateUserDto, LoginDto};
use crate::services::auth_service::AuthService;

pub async fn register(
    service: web::Data<AuthService>,
    dto: web::Json<CreateUserDto>,
) -> impl Responder {
    match service.register(dto.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            if e.to_string().contains("already exists") {
                HttpResponse::Conflict().body(e.to_string())
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

pub async fn login(
    service: web::Data<AuthService>,
    dto: web::Json<LoginDto>,
) -> impl Responder {
    match service.login(dto.into_inner()).await {
        Ok(auth_response) => HttpResponse::Ok().json(auth_response),
        Err(e) => {
            if e.to_string().contains("Invalid credentials") {
                HttpResponse::Unauthorized().body(e.to_string())
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
