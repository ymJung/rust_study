// post.rs
// 게시글 관련 데이터 모델과 DTO를 정의합니다.
// 게시글의 CRUD 작업에 사용되는 구조체들이 포함되어 있습니다.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Post 구조체는 데이터베이스의 posts 테이블과 매핑됩니다.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: Uuid,               // 게시글의 고유 식별자
    pub title: String,          // 게시글 제목
    pub content: String,        // 게시글 내용
    pub author_id: Uuid,        // 작성자 ID (users 테이블의 FK)
    pub created_at: DateTime<Utc>, // 작성 시간
    pub updated_at: DateTime<Utc>, // 수정 시간
}

// CreatePostDto는 게시글 작성 요청에서 사용되는 데이터 구조입니다.
#[derive(Debug, Deserialize)]
pub struct CreatePostDto {
    pub title: String,    // 게시글 제목 (필수)
    pub content: String,  // 게시글 내용 (필수)
}

// UpdatePostDto는 게시글 수정 요청에서 사용되는 데이터 구조입니다.
#[derive(Debug, Deserialize)]
pub struct UpdatePostDto {
    pub title: Option<String>,    // 게시글 제목 (선택)
    pub content: Option<String>,  // 게시글 내용 (선택)
}
