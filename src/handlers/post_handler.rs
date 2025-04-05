// post_handler.rs
// 게시글 관련 HTTP 요청을 처리하는 핸들러들을 정의합니다.
// 각 핸들러는 요청을 받아 적절한 서비스 메서드를 호출하고 결과를 반환합니다.

use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::post::{CreatePostDto, UpdatePostDto};
use crate::services::post_service::PostService;
use crate::middleware::auth_middleware::get_current_user;

// 게시글 작성 핸들러
// POST /api/posts
pub async fn create_post(
    service: web::Data<PostService>,  // 의존성 주입된 PostService
    dto: web::Json<CreatePostDto>,    // JSON 요청 본문
    req: actix_web::HttpRequest,      // 현재 요청 객체
) -> impl Responder {
    // 현재 인증된 사용자의 ID를 가져옵니다.
    let author_id = match get_current_user(&req.into()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    // PostService를 통해 게시글을 생성합니다.
    match service.create_post(dto.into_inner(), author_id).await {
        Ok(post) => HttpResponse::Created().json(post),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// 게시글 상세 조회 핸들러
// GET /api/posts/{post_id}
pub async fn get_post(
    service: web::Data<PostService>,  // 의존성 주입된 PostService
    post_id: web::Path<Uuid>,         // URL 경로 매개변수
) -> impl Responder {
    match service.get_post(post_id.into_inner()).await {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// 게시글 목록 조회 핸들러
// GET /api/posts?page=1&per_page=10
pub async fn get_posts(
    service: web::Data<PostService>,
    query: web::Query<PaginationQuery>,  // URL 쿼리 매개변수
) -> impl Responder {
    // 페이지네이션 매개변수의 기본값을 설정합니다.
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);

    match service.get_posts(page, per_page).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// 게시글 수정 핸들러
// PUT /api/posts/{post_id}
pub async fn update_post(
    service: web::Data<PostService>,
    post_id: web::Path<Uuid>,
    dto: web::Json<UpdatePostDto>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    // 현재 인증된 사용자의 ID를 가져옵니다.
    let author_id = match get_current_user(&req.into()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    // 게시글을 수정합니다. 작성자만 수정할 수 있습니다.
    match service
        .update_post(post_id.into_inner(), dto.into_inner(), author_id)
        .await
    {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// 게시글 삭제 핸들러
// DELETE /api/posts/{post_id}
pub async fn delete_post(
    service: web::Data<PostService>,
    post_id: web::Path<Uuid>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    // 현재 인증된 사용자의 ID를 가져옵니다.
    let author_id = match get_current_user(&req.into()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    // 게시글을 삭제합니다. 작성자만 삭제할 수 있습니다.
    match service.delete_post(post_id.into_inner(), author_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// 페이지네이션을 위한 쿼리 매개변수 구조체
#[derive(serde::Deserialize)]
pub struct PaginationQuery {
    page: Option<i64>,      // 요청할 페이지 번호
    per_page: Option<i64>,  // 페이지당 항목 수
}
