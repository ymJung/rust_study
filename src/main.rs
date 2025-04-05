// main.rs
// 게시판 애플리케이션의 진입점입니다.
// Actix-web을 사용하여 웹 서버를 구성하고, 라우팅과 미들웨어를 설정합니다.

use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod config;
mod models;
mod handlers;
mod middleware;
mod services;
mod utils;

use handlers::{auth_handler, post_handler, comment_handler};
use services::{auth_service::AuthService, post_service::PostService, comment_service::CommentService};
use middleware::auth_middleware::Auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // .env 파일에서 환경 변수를 로드합니다.
    dotenv().ok();
    // env_logger를 초기화하여 로깅을 활성화합니다.
    env_logger::init();

    // 환경 변수에서 서버 설정을 읽어옵니다.
    // unwrap_or_else를 사용하여 환경 변수가 없을 경우 기본값을 제공합니다.
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // PostgreSQL 연결 풀을 생성합니다.
    // 동시에 최대 5개의 데이터베이스 연결을 유지합니다.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    println!("🚀 Server running at http://{}:{}", host, port);

    // HTTP 서버를 구성하고 시작합니다.
    HttpServer::new(move || {
        // 새로운 App 인스턴스를 생성합니다.
        // 각 연결마다 새로운 App이 생성되므로, 여기서 정의된 모든 것이 복제됩니다.
        App::new()
            // Logger 미들웨어를 추가하여 HTTP 요청 로깅을 활성화합니다.
            .wrap(Logger::default())
            // 서비스 인스턴스들을 애플리케이션 데이터로 등록합니다.
            // web::Data로 래핑하여 여러 스레드에서 안전하게 공유할 수 있게 합니다.
            .app_data(web::Data::new(AuthService::new(pool.clone())))
            .app_data(web::Data::new(PostService::new(pool.clone())))
            .app_data(web::Data::new(CommentService::new(pool.clone())))
            // API 라우트를 설정합니다.
            .service(
                web::scope("/api")  // /api 접두사로 모든 엔드포인트를 그룹화합니다.
                    .service(
                        // 인증 관련 엔드포인트 (/api/auth/...)
                        web::scope("/auth")
                            .route("/register", web::post().to(auth_handler::register))
                            .route("/login", web::post().to(auth_handler::login))
                    )
                    .service(
                        // 게시글 관련 엔드포인트 (/api/posts/...)
                        web::scope("/posts")
                            .wrap(Auth)  // 인증 미들웨어 적용
                            .route("", web::post().to(post_handler::create_post))
                            .route("", web::get().to(post_handler::get_posts))
                            .route("/{post_id}", web::get().to(post_handler::get_post))
                            .route("/{post_id}", web::put().to(post_handler::update_post))
                            .route("/{post_id}", web::delete().to(post_handler::delete_post))
                            .service(
                                // 게시글의 댓글 관련 엔드포인트
                                web::scope("/{post_id}/comments")
                                    .wrap(Auth)
                                    .route("", web::post().to(comment_handler::create_comment))
                                    .route("", web::get().to(comment_handler::get_post_comments))
                            )
                    )
                    .service(
                        // 댓글 관련 엔드포인트 (/api/comments/...)
                        web::scope("/comments")
                            .wrap(Auth)
                            .route("/{comment_id}", web::put().to(comment_handler::update_comment))
                            .route("/{comment_id}", web::delete().to(comment_handler::delete_comment))
                            .route("/{comment_id}/replies", web::get().to(comment_handler::get_replies))
                    )
            )
    })
    // 서버를 바인딩하고 시작합니다.
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
