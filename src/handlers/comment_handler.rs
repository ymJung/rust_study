use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::comment::{CreateCommentDto, UpdateCommentDto};
use crate::services::comment_service::CommentService;
use crate::middleware::auth_middleware::get_current_user;

pub async fn create_comment(
    service: web::Data<CommentService>,
    post_id: web::Path<Uuid>,
    dto: web::Json<CreateCommentDto>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    let author_id = match get_current_user(&req.into()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match service
        .create_comment(post_id.into_inner(), author_id, dto.into_inner())
        .await
    {
        Ok(comment) => HttpResponse::Created().json(comment),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_post_comments(
    service: web::Data<CommentService>,
    post_id: web::Path<Uuid>,
    query: web::Query<PaginationQuery>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);

    match service.get_post_comments(post_id.into_inner(), page, per_page).await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_replies(
    service: web::Data<CommentService>,
    comment_id: web::Path<Uuid>,
    query: web::Query<PaginationQuery>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);

    match service.get_replies(comment_id.into_inner(), page, per_page).await {
        Ok(replies) => HttpResponse::Ok().json(replies),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_comment(
    service: web::Data<CommentService>,
    comment_id: web::Path<Uuid>,
    dto: web::Json<UpdateCommentDto>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    let author_id = match get_current_user(&req.into()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match service
        .update_comment(comment_id.into_inner(), author_id, dto.into_inner())
        .await
    {
        Ok(Some(comment)) => HttpResponse::Ok().json(comment),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_comment(
    service: web::Data<CommentService>,
    comment_id: web::Path<Uuid>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    let author_id = match get_current_user(&req.into()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match service.delete_comment(comment_id.into_inner(), author_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(serde::Deserialize)]
pub struct PaginationQuery {
    page: Option<i64>,
    per_page: Option<i64>,
}
