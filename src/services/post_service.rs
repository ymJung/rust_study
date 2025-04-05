// post_service.rs
// 게시글 관련 비즈니스 로직을 처리하는 서비스입니다.
// 게시글의 CRUD 작업과 페이지네이션을 처리합니다.

use sqlx::PgPool;
use uuid::Uuid;
use crate::models::post::{Post, CreatePostDto, UpdatePostDto};

// PostService는 게시글 관련 기능을 제공하는 서비스 구조체입니다.
pub struct PostService {
    db: PgPool,  // 데이터베이스 연결 풀
}

impl PostService {
    // 새로운 PostService 인스턴스를 생성합니다.
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    // 새 게시글을 생성합니다.
    pub async fn create_post(&self, dto: CreatePostDto, author_id: Uuid) -> Result<Post, sqlx::Error> {
        // 게시글을 데이터베이스에 저장
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (title, content, author_id)
            VALUES ($1, $2, $3)
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
            dto.title,
            dto.content,
            author_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(post)
    }

    // 특정 게시글을 조회합니다.
    pub async fn get_post(&self, post_id: Uuid) -> Result<Option<Post>, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts
            WHERE id = $1
            "#,
            post_id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(post)
    }

    // 게시글 목록을 페이지네이션하여 조회합니다.
    pub async fn get_posts(&self, page: i64, per_page: i64) -> Result<Vec<Post>, sqlx::Error> {
        let offset = (page - 1) * per_page;
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            per_page,
            offset
        )
        .fetch_all(&self.db)
        .await?;

        Ok(posts)
    }

    // 게시글을 수정합니다.
    // 작성자만 수정할 수 있습니다.
    pub async fn update_post(
        &self,
        post_id: Uuid,
        dto: UpdatePostDto,
        author_id: Uuid,
    ) -> Result<Option<Post>, sqlx::Error> {
        // 게시글 존재 여부와 작성자 확인
        let post = match sqlx::query_as!(
            Post,
            "SELECT * FROM posts WHERE id = $1",
            post_id
        )
        .fetch_optional(&self.db)
        .await?
        {
            Some(post) if post.author_id == author_id => post,
            _ => return Ok(None),
        };

        // 게시글 수정
        let updated = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET
                title = COALESCE($1, title),
                content = COALESCE($2, content),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $3
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
            dto.title,
            dto.content,
            post_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(Some(updated))
    }

    // 게시글을 삭제합니다.
    // 작성자만 삭제할 수 있습니다.
    pub async fn delete_post(&self, post_id: Uuid, author_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE id = $1 AND author_id = $2
            "#,
            post_id,
            author_id
        )
        .execute(&self.db)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
