use crate::common::TestDb;
use fake::{Fake, Faker};
use rust_study::models::comment::{CreateCommentDto, UpdateCommentDto};
use rust_study::services::comment_service::CommentService;
use serial_test::serial;
use test_context::test_context;
use uuid::Uuid;

mod common;

async fn create_test_comment(
    service: &CommentService,
    post_id: Uuid,
    author_id: Uuid,
    parent_id: Option<Uuid>,
) -> Uuid {
    let dto = CreateCommentDto {
        content: Faker.fake::<String>(),
        parent_id,
    };

    let comment = service.create_comment(post_id, author_id, dto).await.unwrap();
    comment.id
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_create_comment(ctx: &TestDb) {
    let service = CommentService::new(ctx.pool.clone());
    let post_id = Uuid::new_v4();
    let author_id = Uuid::new_v4();
    
    let dto = CreateCommentDto {
        content: Faker.fake::<String>(),
        parent_id: None,
    };

    let comment = service.create_comment(post_id, author_id, dto.clone()).await.unwrap();

    assert_eq!(comment.content, dto.content);
    assert_eq!(comment.post_id, post_id);
    assert_eq!(comment.author_id, author_id);
    assert_eq!(comment.parent_id, None);
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_create_reply(ctx: &TestDb) {
    let service = CommentService::new(ctx.pool.clone());
    let post_id = Uuid::new_v4();
    let author_id = Uuid::new_v4();
    
    // Create parent comment
    let parent_id = create_test_comment(&service, post_id, author_id, None).await;
    
    // Create reply
    let dto = CreateCommentDto {
        content: Faker.fake::<String>(),
        parent_id: Some(parent_id),
    };

    let reply = service.create_comment(post_id, author_id, dto.clone()).await.unwrap();

    assert_eq!(reply.content, dto.content);
    assert_eq!(reply.post_id, post_id);
    assert_eq!(reply.author_id, author_id);
    assert_eq!(reply.parent_id, Some(parent_id));
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_get_post_comments(ctx: &TestDb) {
    let service = CommentService::new(ctx.pool.clone());
    let post_id = Uuid::new_v4();
    let author_id = Uuid::new_v4();

    // Create 15 comments
    for _ in 0..15 {
        create_test_comment(&service, post_id, author_id, None).await;
    }

    // Test first page (10 comments)
    let comments = service.get_post_comments(post_id, 1, 10).await.unwrap();
    assert_eq!(comments.len(), 10);

    // Test second page (5 comments)
    let comments = service.get_post_comments(post_id, 2, 10).await.unwrap();
    assert_eq!(comments.len(), 5);
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_get_replies(ctx: &TestDb) {
    let service = CommentService::new(ctx.pool.clone());
    let post_id = Uuid::new_v4();
    let author_id = Uuid::new_v4();

    // Create parent comment
    let parent_id = create_test_comment(&service, post_id, author_id, None).await;

    // Create 5 replies
    for _ in 0..5 {
        create_test_comment(&service, post_id, author_id, Some(parent_id)).await;
    }

    let replies = service.get_replies(parent_id, 1, 10).await.unwrap();
    assert_eq!(replies.len(), 5);
    for reply in replies {
        assert_eq!(reply.parent_id, Some(parent_id));
    }
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_update_comment(ctx: &TestDb) {
    let service = CommentService::new(ctx.pool.clone());
    let post_id = Uuid::new_v4();
    let author_id = Uuid::new_v4();

    let comment_id = create_test_comment(&service, post_id, author_id, None).await;

    let new_content: String = Faker.fake();
    let dto = UpdateCommentDto {
        content: new_content.clone(),
    };

    let updated_comment = service
        .update_comment(comment_id, author_id, dto)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(updated_comment.id, comment_id);
    assert_eq!(updated_comment.content, new_content);
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_delete_comment(ctx: &TestDb) {
    let service = CommentService::new(ctx.pool.clone());
    let post_id = Uuid::new_v4();
    let author_id = Uuid::new_v4();

    let comment_id = create_test_comment(&service, post_id, author_id, None).await;

    // Delete the comment
    let result = service.delete_comment(comment_id, author_id).await.unwrap();
    assert!(result);

    // Verify comment is deleted
    let comment = service.get_comment(comment_id).await.unwrap();
    assert!(comment.is_none());
}

#[test_context(TestDb)]
#[tokio::test]
#[serial]
async fn test_delete_comment_wrong_author(ctx: &TestDb) {
    let service = CommentService::new(ctx.pool.clone());
    let post_id = Uuid::new_v4();
    let author_id = Uuid::new_v4();
    let wrong_author_id = Uuid::new_v4();

    let comment_id = create_test_comment(&service, post_id, author_id, None).await;

    // Try to delete with wrong author
    let result = service.delete_comment(comment_id, wrong_author_id).await.unwrap();
    assert!(!result);

    // Verify comment still exists
    let comment = service.get_comment(comment_id).await.unwrap();
    assert!(comment.is_some());
}
