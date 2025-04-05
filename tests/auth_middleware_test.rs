use actix_web::{
    test, web, App, HttpResponse,
    http::{header, StatusCode},
};
use fake::{Fake, Faker};
use rust_study::{
    middleware::auth_middleware::Auth,
    models::user::CreateUserDto,
    services::auth_service::AuthService,
};
use test_context::test_context;

mod common;
use common::TestDb;

async fn protected_route() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn create_test_app(
    pool: sqlx::PgPool,
) -> impl actix_web::dev::Service<actix_http::Request, Response = actix_web::dev::ServiceResponse> {
    test::init_service(
        App::new()
            .app_data(web::Data::new(AuthService::new(pool)))
            .service(
                web::scope("/api")
                    .wrap(Auth)
                    .route("/protected", web::get().to(protected_route)),
            ),
    )
    .await
}

#[test_context(TestDb)]
#[actix_web::test]
async fn test_auth_middleware_no_token(ctx: &TestDb) {
    let app = create_test_app(ctx.pool.clone()).await;

    let req = test::TestRequest::get()
        .uri("/api/protected")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[test_context(TestDb)]
#[actix_web::test]
async fn test_auth_middleware_invalid_token(ctx: &TestDb) {
    let app = create_test_app(ctx.pool.clone()).await;

    let req = test::TestRequest::get()
        .uri("/api/protected")
        .insert_header((header::AUTHORIZATION, "Bearer invalid.token.here"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[test_context(TestDb)]
#[actix_web::test]
async fn test_auth_middleware_valid_token(ctx: &TestDb) {
    let auth_service = AuthService::new(ctx.pool.clone());
    let app = create_test_app(ctx.pool.clone()).await;

    // Create a test user and get token
    let dto = CreateUserDto {
        username: Faker.fake::<String>(),
        email: format!("{}@example.com", Faker.fake::<String>()),
        password: "password123".to_string(),
    };

    let user = auth_service.register(dto.clone()).await.unwrap();
    let auth_response = auth_service
        .login(rust_study::models::user::LoginDto {
            email: dto.email,
            password: dto.password,
        })
        .await
        .unwrap();

    let req = test::TestRequest::get()
        .uri("/api/protected")
        .insert_header((
            header::AUTHORIZATION,
            format!("Bearer {}", auth_response.token),
        ))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[test_context(TestDb)]
#[actix_web::test]
async fn test_auth_middleware_malformed_token(ctx: &TestDb) {
    let app = create_test_app(ctx.pool.clone()).await;

    let req = test::TestRequest::get()
        .uri("/api/protected")
        .insert_header((header::AUTHORIZATION, "NotBearer some.token.here"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
