use actix_web::{test, web, App};
use fake::{Fake, Faker};
use rust_study::{
    handlers::post_handler,
    models::post::{CreatePostDto, UpdatePostDto},
    services::post_service::PostService,
};
use serde_json::json;
use test_context::test_context;
use uuid::Uuid;

mod common;
use common::TestDb;

async fn create_test_app(
    pool: sqlx::PgPool,
) -> impl actix_web::dev::Service<actix_http::Request, Response = actix_web::dev::ServiceResponse> {
    test::init_service(
        App::new()
            .app_data(web::Data::new(PostService::new(pool)))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/posts")
                            .route("", web::post().to(post_handler::create_post))
                            .route("", web::get().to(post_handler::get_posts))
                            .route("/{post_id}", web::get().to(post_handler::get_post))
                            .route("/{post_id}", web::put().to(post_handler::update_post))
                            .route("/{post_id}", web::delete().to(post_handler::delete_post)),
                    ),
            ),
    )
    .await
}

#[test_context(TestDb)]
#[actix_web::test]
async fn test_create_post_handler(ctx: &TestDb) {
    let app = create_test_app(ctx.pool.clone()).await;
    let author_id = Uuid::new_v4();

    let dto = CreatePostDto {
        title: Faker.fake::<String>(),
        content: Faker.fake::<String>(),
    };

    let req = test::TestRequest::post()
        .uri(&format!("/api/posts?author_id={}", author_id))
        .set_json(&dto)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["title"], json!(dto.title));
    assert_eq!(body["content"], json!(dto.content));
}

#[test_context(TestDb)]
#[actix_web::test]
async fn test_get_post_handler(ctx: &TestDb) {
    let app = create_test_app(ctx.pool.clone()).await;
    let author_id = Uuid::new_v4();

    let title: String = Faker.fake();
    let content: String = Faker.fake();
    let post_id = common::create_test_post(&ctx.pool, &title, &content, author_id).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/posts/{}", post_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["title"], json!(title));
    assert_eq!(body["content"], json!(content));
}

#[test_context(TestDb)]
#[actix_web::test]
async fn test_get_posts_handler(ctx: &TestDb) {
    let app = create_test_app(ctx.pool.clone()).await;
    let author_id = Uuid::new_v4();

    // Create 5 test posts
    for _ in 0..5 {
        let title: String = Faker.fake();
        let content: String = Faker.fake();
        common::create_test_post(&ctx.pool, &title, &content, author_id).await;
    }

    let req = test::TestRequest::get()
        .uri("/api/posts?page=1&per_page=10")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: Vec<serde_json::Value> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 5);
}

#[test_context(TestDb)]
#[actix_web::test]
async fn test_update_post_handler(ctx: &TestDb) {
    let app = create_test_app(ctx.pool.clone()).await;
    let author_id = Uuid::new_v4();

    let title: String = Faker.fake();
    let content: String = Faker.fake();
    let post_id = common::create_test_post(&ctx.pool, &title, &content, author_id).await;

    let new_title: String = Faker.fake();
    let new_content: String = Faker.fake();
    let dto = UpdatePostDto {
        title: Some(new_title.clone()),
        content: Some(new_content.clone()),
    };

    let req = test::TestRequest::put()
        .uri(&format!("/api/posts/{}?author_id={}", post_id, author_id))
        .set_json(&dto)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["title"], json!(new_title));
    assert_eq!(body["content"], json!(new_content));
}

#[test_context(TestDb)]
#[actix_web::test]
async fn test_delete_post_handler(ctx: &TestDb) {
    let app = create_test_app(ctx.pool.clone()).await;
    let author_id = Uuid::new_v4();

    let title: String = Faker.fake();
    let content: String = Faker.fake();
    let post_id = common::create_test_post(&ctx.pool, &title, &content, author_id).await;

    let req = test::TestRequest::delete()
        .uri(&format!("/api/posts/{}?author_id={}", post_id, author_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NO_CONTENT);

    // Verify post is deleted
    let req = test::TestRequest::get()
        .uri(&format!("/api/posts/{}", post_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}
