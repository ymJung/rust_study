// auth_service.rs
// 사용자 인증과 관련된 비즈니스 로직을 처리하는 서비스입니다.
// 회원가입, 로그인, JWT 토큰 관리 등의 기능을 제공합니다.

use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::{CreateUserDto, LoginDto, User, AuthResponse};
use crate::errors::AppError;
use crate::config::JWT_SECRET;

// AuthService는 사용자 인증 관련 기능을 제공하는 서비스 구조체입니다.
pub struct AuthService {
    db: PgPool,  // 데이터베이스 연결 풀
}

impl AuthService {
    // 새로운 AuthService 인스턴스를 생성합니다.
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    // 회원가입 처리를 수행합니다.
    // 이메일 중복 체크 후 비밀번호를 해시화하여 저장합니다.
    pub async fn register(&self, dto: CreateUserDto) -> Result<User, AppError> {
        // 이메일 중복 체크
        if self.get_user_by_email(&dto.email).await?.is_some() {
            return Err(AppError::EmailAlreadyExists);
        }

        // 비밀번호 해시화
        let password_hash = hash(dto.password.as_bytes(), DEFAULT_COST)?;

        // 새 사용자 생성
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            dto.username,
            dto.email,
            password_hash
        )
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }

    // 로그인 처리를 수행합니다.
    // 이메일과 비밀번호를 검증하고 JWT 토큰을 발급합니다.
    pub async fn login(&self, dto: LoginDto) -> Result<AuthResponse, AppError> {
        // 사용자 조회
        let user = self.get_user_by_email(&dto.email)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        // 비밀번호 검증
        if !verify(dto.password.as_bytes(), &user.password_hash)? {
            return Err(AppError::InvalidCredentials);
        }

        // JWT 토큰 생성
        let token = self.create_token(user.id)?;

        Ok(AuthResponse { token })
    }

    // 이메일로 사용자를 조회합니다.
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }

    // JWT 토큰을 생성합니다.
    // 토큰은 24시간 동안 유효합니다.
    fn create_token(&self, user_id: Uuid) -> Result<String, AppError> {
        #[derive(serde::Serialize)]
        struct Claims {
            sub: Uuid,           // 토큰 주체 (사용자 ID)
            exp: i64,            // 만료 시간 (Unix timestamp)
            iat: i64,            // 발급 시간 (Unix timestamp)
        }

        let now = Utc::now();
        let claims = Claims {
            sub: user_id,
            exp: (now + Duration::hours(24)).timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
        )?;

        Ok(token)
    }
}
