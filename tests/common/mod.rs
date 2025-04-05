use sqlx::PgPool;
use test_context::{AsyncTestContext, TestContext};
use uuid::Uuid;

pub struct TestDb {
    pub pool: PgPool,
}

impl TestDb {
    pub async fn new() -> Self {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url).await.unwrap();
        Self { pool }
    }

    pub async fn clear_tables(&self) {
        sqlx::query!("TRUNCATE TABLE users, posts, comments CASCADE")
            .execute(&self.pool)
            .await
            .unwrap();
    }
}

#[async_trait::async_trait]
impl AsyncTestContext for TestDb {
    async fn setup() -> Self {
        let db = Self::new().await;
        db.clear_tables().await;
        db
    }

    async fn teardown(self) {
        self.clear_tables().await;
    }
}

// 테스트용 헬퍼 함수들
pub async fn create_test_post(pool: &PgPool, title: &str, content: &str, author_id: Uuid) -> Uuid {
    let post = sqlx::query!(
        r#"
        INSERT INTO posts (title, content, author_id)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        title,
        content,
        author_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    post.id
}

pub async fn create_test_comment(
    pool: &PgPool,
    content: &str,
    post_id: Uuid,
    author_id: Uuid,
    parent_id: Option<Uuid>,
) -> Uuid {
    let comment = sqlx::query!(
        r#"
        INSERT INTO comments (content, post_id, author_id, parent_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        content,
        post_id,
        author_id,
        parent_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    comment.id
}

pub async fn create_test_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password_hash: &str,
) -> Uuid {
    let user = sqlx::query!(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        username,
        email,
        password_hash
    )
    .fetch_one(pool)
    .await
    .unwrap();

    user.id
}
