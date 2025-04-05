use crate::common::TestDb;
use fake::{Fake, Faker};
use rust_study::models::post::{CreatePostDto, UpdatePostDto};
use rust_study::services::post_service::PostService;
use serial_test::serial;
use test_context::test_context;
use uuid::Uuid;

mod common;

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_create_post(ctx: &TestDb) {
    let service = PostService::new(ctx.pool.clone());
    let author_id = Uuid::new_v4();
    
    let dto = CreatePostDto {
        title: Faker.fake::<String>(),
        content: Faker.fake::<String>(),
    };

    let post = service.create_post(dto.clone(), author_id).await.unwrap();

    assert_eq!(post.title, dto.title);
    assert_eq!(post.content, dto.content);
    assert_eq!(post.author_id, author_id);
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_get_post(ctx: &TestDb) {
    let service = PostService::new(ctx.pool.clone());
    let author_id = Uuid::new_v4();
    
    let title: String = Faker.fake();
    let content: String = Faker.fake();
    let post_id = common::create_test_post(&ctx.pool, &title, &content, author_id).await;

    let post = service.get_post(post_id).await.unwrap().unwrap();

    assert_eq!(post.id, post_id);
    assert_eq!(post.title, title);
    assert_eq!(post.content, content);
    assert_eq!(post.author_id, author_id);
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_get_posts_pagination(ctx: &TestDb) {
    let service = PostService::new(ctx.pool.clone());
    let author_id = Uuid::new_v4();

    // Create 15 test posts
    for _ in 0..15 {
        let title: String = Faker.fake();
        let content: String = Faker.fake();
        common::create_test_post(&ctx.pool, &title, &content, author_id).await;
    }

    // Test first page (10 posts)
    let posts = service.get_posts(1, 10).await.unwrap();
    assert_eq!(posts.len(), 10);

    // Test second page (5 posts)
    let posts = service.get_posts(2, 10).await.unwrap();
    assert_eq!(posts.len(), 5);
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_update_post(ctx: &TestDb) {
    let service = PostService::new(ctx.pool.clone());
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

    let updated_post = service
        .update_post(post_id, dto, author_id)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(updated_post.id, post_id);
    assert_eq!(updated_post.title, new_title);
    assert_eq!(updated_post.content, new_content);
    assert_eq!(updated_post.author_id, author_id);
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_delete_post(ctx: &TestDb) {
    let service = PostService::new(ctx.pool.clone());
    let author_id = Uuid::new_v4();
    
    let title: String = Faker.fake();
    let content: String = Faker.fake();
    let post_id = common::create_test_post(&ctx.pool, &title, &content, author_id).await;

    // Delete the post
    let result = service.delete_post(post_id, author_id).await.unwrap();
    assert!(result);

    // Verify post is deleted
    let post = service.get_post(post_id).await.unwrap();
    assert!(post.is_none());
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_delete_post_wrong_author(ctx: &TestDb) {
    let service = PostService::new(ctx.pool.clone());
    let author_id = Uuid::new_v4();
    let wrong_author_id = Uuid::new_v4();
    
    let title: String = Faker.fake();
    let content: String = Faker.fake();
    let post_id = common::create_test_post(&ctx.pool, &title, &content, author_id).await;

    // Try to delete with wrong author
    let result = service.delete_post(post_id, wrong_author_id).await.unwrap();
    assert!(!result);

    // Verify post still exists
    let post = service.get_post(post_id).await.unwrap();
    assert!(post.is_some());
}
