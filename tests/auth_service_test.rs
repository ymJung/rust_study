use crate::common::TestDb;
use fake::{Fake, Faker};
use rust_study::{
    models::user::{CreateUserDto, LoginDto},
    services::auth_service::AuthService,
};
use serial_test::serial;
use test_context::test_context;

mod common;

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_register_user(ctx: &TestDb) {
    let service = AuthService::new(ctx.pool.clone());
    
    let dto = CreateUserDto {
        username: Faker.fake::<String>(),
        email: format!("{}@example.com", Faker.fake::<String>()),
        password: "password123".to_string(),
    };

    let user = service.register(dto.clone()).await.unwrap();

    assert_eq!(user.username, dto.username);
    assert_eq!(user.email, dto.email);
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_register_duplicate_email(ctx: &TestDb) {
    let service = AuthService::new(ctx.pool.clone());
    let email = format!("{}@example.com", Faker.fake::<String>());
    
    let dto1 = CreateUserDto {
        username: Faker.fake::<String>(),
        email: email.clone(),
        password: "password123".to_string(),
    };

    let dto2 = CreateUserDto {
        username: Faker.fake::<String>(),
        email: email.clone(),
        password: "password456".to_string(),
    };

    service.register(dto1).await.unwrap();
    let result = service.register(dto2).await;

    assert!(result.is_err());
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_login_success(ctx: &TestDb) {
    let service = AuthService::new(ctx.pool.clone());
    let password = "password123".to_string();
    
    let register_dto = CreateUserDto {
        username: Faker.fake::<String>(),
        email: format!("{}@example.com", Faker.fake::<String>()),
        password: password.clone(),
    };

    let created_user = service.register(register_dto.clone()).await.unwrap();

    let login_dto = LoginDto {
        email: register_dto.email,
        password,
    };

    let auth_response = service.login(login_dto).await.unwrap();

    assert_eq!(auth_response.user.id, created_user.id);
    assert!(!auth_response.token.is_empty());
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_login_wrong_password(ctx: &TestDb) {
    let service = AuthService::new(ctx.pool.clone());
    
    let register_dto = CreateUserDto {
        username: Faker.fake::<String>(),
        email: format!("{}@example.com", Faker.fake::<String>()),
        password: "password123".to_string(),
    };

    service.register(register_dto.clone()).await.unwrap();

    let login_dto = LoginDto {
        email: register_dto.email,
        password: "wrongpassword".to_string(),
    };

    let result = service.login(login_dto).await;
    assert!(result.is_err());
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_verify_token(ctx: &TestDb) {
    let service = AuthService::new(ctx.pool.clone());
    
    let register_dto = CreateUserDto {
        username: Faker.fake::<String>(),
        email: format!("{}@example.com", Faker.fake::<String>()),
        password: "password123".to_string(),
    };

    let created_user = service.register(register_dto.clone()).await.unwrap();

    let login_dto = LoginDto {
        email: register_dto.email,
        password: register_dto.password,
    };

    let auth_response = service.login(login_dto).await.unwrap();
    let claims = service.verify_token(&auth_response.token).unwrap();

    assert_eq!(claims.sub, created_user.id.to_string());
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_verify_invalid_token(ctx: &TestDb) {
    let service = AuthService::new(ctx.pool.clone());
    let result = service.verify_token("invalid.token.here").await;
    assert!(result.is_err());
}
