// comment_service.rs
// 댓글 관련 비즈니스 로직을 처리하는 서비스입니다.
// 댓글의 CRUD 작업과 대댓글 기능을 처리합니다.

use sqlx::PgPool;
use uuid::Uuid;
use crate::models::comment::{Comment, CreateCommentDto, UpdateCommentDto};

// CommentService는 댓글 관련 기능을 제공하는 서비스 구조체입니다.
pub struct CommentService {
    db: PgPool,  // 데이터베이스 연결 풀
}

impl CommentService {
    // 새로운 CommentService 인스턴스를 생성합니다.
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    // 새 댓글을 생성합니다.
    pub async fn create_comment(
        &self,
        post_id: Uuid,
        author_id: Uuid,
        dto: CreateCommentDto,
    ) -> Result<Comment, sqlx::Error> {
        // 댓글 저장
        let comment = sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO comments (content, post_id, author_id, parent_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, content, post_id, author_id, parent_id, created_at, updated_at
            "#,
            dto.content,
            post_id,
            author_id,
            dto.parent_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(comment)
    }

    // 특정 게시글의 댓글 목록을 조회합니다.
    pub async fn get_post_comments(
        &self,
        post_id: Uuid,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<Comment>, sqlx::Error> {
        // 페이지네이션 적용하여 댓글 조회
        let offset = (page - 1) * per_page;
        let comments = sqlx::query_as!(
            Comment,
            r#"
            SELECT id, content, post_id, author_id, parent_id, created_at, updated_at
            FROM comments
            WHERE post_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            post_id,
            per_page,
            offset
        )
        .fetch_all(&self.db)
        .await?;

        Ok(comments)
    }

    // 댓글을 조회합니다.
    pub async fn get_comment(&self, comment_id: Uuid) -> Result<Option<Comment>, sqlx::Error> {
        let comment = sqlx::query_as!(
            Comment,
            r#"
            SELECT id, content, post_id, author_id, parent_id, created_at, updated_at
            FROM comments
            WHERE id = $1
            "#,
            comment_id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(comment)
    }

    // 댓글을 수정합니다.
    // 작성자만 수정할 수 있습니다.
    pub async fn update_comment(
        &self,
        comment_id: Uuid,
        author_id: Uuid,
        dto: UpdateCommentDto,
    ) -> Result<Option<Comment>, sqlx::Error> {
        // 댓글 존재 여부와 작성자 확인
        let comment = match sqlx::query_as!(
            Comment,
            "SELECT * FROM comments WHERE id = $1",
            comment_id
        )
        .fetch_optional(&self.db)
        .await?
        {
            Some(comment) if comment.author_id == author_id => comment,
            _ => return Ok(None),
        };

        // 댓글 수정
        let updated = sqlx::query_as!(
            Comment,
            r#"
            UPDATE comments
            SET content = $1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            RETURNING id, content, post_id, author_id, parent_id, created_at, updated_at
            "#,
            dto.content,
            comment_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(Some(updated))
    }

    // 댓글을 삭제합니다.
    // 작성자만 삭제할 수 있습니다.
    pub async fn delete_comment(
        &self,
        comment_id: Uuid,
        author_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM comments
            WHERE id = $1 AND author_id = $2
            "#,
            comment_id,
            author_id
        )
        .execute(&self.db)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // 대댓글을 조회합니다.
    pub async fn get_replies(
        &self,
        parent_id: Uuid,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<Comment>, sqlx::Error> {
        // 페이지네이션 적용하여 대댓글 조회
        let offset = (page - 1) * per_page;
        let replies = sqlx::query_as!(
            Comment,
            r#"
            SELECT id, content, post_id, author_id, parent_id, created_at, updated_at
            FROM comments
            WHERE parent_id = $1
            ORDER BY created_at ASC
            LIMIT $2 OFFSET $3
            "#,
            parent_id,
            per_page,
            offset
        )
        .fetch_all(&self.db)
        .await?;

        Ok(replies)
    }
}
