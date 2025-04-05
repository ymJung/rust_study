// user.rs
// 사용자 관련 데이터 모델과 DTO(Data Transfer Object)를 정의합니다.
// 데이터베이스와 API 요청/응답에 사용되는 구조체들이 포함되어 있습니다.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// User 구조체는 데이터베이스의 users 테이블과 매핑됩니다.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,              // 사용자의 고유 식별자
    pub username: String,       // 사용자 이름
    pub email: String,         // 이메일 주소 (유니크)
    #[serde(skip_serializing)] // 비밀번호 해시는 JSON 응답에 포함되지 않습니다
    pub password_hash: String, // bcrypt로 해시화된 비밀번호
    pub created_at: DateTime<Utc>, // 계정 생성 시간
    pub updated_at: DateTime<Utc>, // 계정 업데이트 시간
}

// CreateUserDto는 회원가입 요청에서 사용되는 데이터 구조입니다.
#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub username: String,  // 사용자 이름 (필수)
    pub email: String,     // 이메일 주소 (필수, 유니크)
    pub password: String,  // 비밀번호 (필수, 평문)
}

// LoginDto는 로그인 요청에서 사용되는 데이터 구조입니다.
#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub email: String,     // 이메일 주소
    pub password: String,  // 비밀번호 (평문)
}
