// comment.rs
// 댓글 관련 데이터 모델과 DTO를 정의합니다.
// 댓글의 CRUD 작업과 대댓글 기능에 사용되는 구조체들이 포함되어 있습니다.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Comment 구조체는 데이터베이스의 comments 테이블과 매핑됩니다.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,               // 댓글의 고유 식별자
    pub content: String,        // 댓글 내용
    pub post_id: Uuid,         // 게시글 ID (posts 테이블의 FK)
    pub author_id: Uuid,       // 작성자 ID (users 테이블의 FK)
    pub parent_id: Option<Uuid>, // 부모 댓글 ID (대댓글인 경우)
    pub created_at: DateTime<Utc>, // 작성 시간
    pub updated_at: DateTime<Utc>, // 수정 시간
}

// CreateCommentDto는 댓글 작성 요청에서 사용되는 데이터 구조입니다.
#[derive(Debug, Deserialize)]
pub struct CreateCommentDto {
    pub content: String,         // 댓글 내용 (필수)
    pub parent_id: Option<Uuid>, // 부모 댓글 ID (대댓글 작성 시)
}

// UpdateCommentDto는 댓글 수정 요청에서 사용되는 데이터 구조입니다.
#[derive(Debug, Deserialize)]
pub struct UpdateCommentDto {
    pub content: String,  // 새로운 댓글 내용
}
