# Rust Bulletin Board

Rust로 구현한 간단한 게시판 애플리케이션입니다. 사용자 인증, 게시글 관리, 댓글 기능을 제공합니다.
**Windsurf IDE를 활용한 학습 프로젝트입니다.**

## 기능

- 사용자 인증
  - 회원가입 및 로그인
  - JWT 기반 인증 (24시간 유효)
  - bcrypt를 사용한 비밀번호 해싱
  - 미들웨어를 통한 인증 상태 검증
- 게시글 관리
  - 게시글 CRUD (작성, 조회, 수정, 삭제)
  - 페이지네이션 지원
  - 작성자 권한 관리 (본인 게시글만 수정/삭제 가능)
  - 작성자 정보 포함 응답
- 댓글
  - 게시글에 대한 댓글 CRUD
  - 대댓글 시스템 지원
  - 작성자 권한 관리 (본인 댓글만 수정/삭제 가능)
  - 댓글 수 카운팅

## 프로젝트 구조

```
src/
├── handlers/       # HTTP 요청 처리
├── middleware/     # 인증 미들웨어
├── models/         # 데이터 모델 및 DTO
├── services/       # 비즈니스 로직
├── config.rs       # 환경 설정
├── errors.rs       # 에러 처리
└── main.rs         # 서버 설정 및 라우팅

tests/              # 통합 테스트
```

## 기술 스택

- **언어**: Rust 1.75+
- **웹 프레임워크**: Actix-web 4.0
- **데이터베이스**: PostgreSQL 15+
- **ORM**: SQLx
- **인증**: JsonWebToken
- **암호화**: Bcrypt
- **개발 도구**:
  - Windsurf IDE (AI 기반 개발 지원)
  - VSCode
- **테스트**: 
  - Actix-web TestServer
  - Fake (테스트 데이터 생성)
  - Test-context (테스트 환경 관리)

## 설치 및 실행

### 필수 조건

- Rust (최신 stable 버전)
- PostgreSQL 15+
- Docker (선택사항)

### 환경 변수 설정

`.env` 파일을 프로젝트 루트에 생성하고 다음 내용을 추가합니다:

```env
HOST=127.0.0.1
PORT=8080
DATABASE_URL=postgres://username:password@localhost:5432/dbname
JWT_SECRET=your_jwt_secret_key
```

### 데이터베이스 설정

```bash
# PostgreSQL 실행 (Docker 사용 시)
docker run --name postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres

# 마이그레이션 실행
sqlx database create
sqlx migrate run
```

### 실행

```bash
# 개발 모드
cargo run

# 테스트 실행
cargo test
```

## API 엔드포인트

### 인증

```
POST /api/auth/register
- 회원가입
- Request: { "username": "string", "email": "string", "password": "string" }
- Response: { "id": "uuid", "username": "string", "email": "string" }

POST /api/auth/login
- 로그인
- Request: { "email": "string", "password": "string" }
- Response: { "token": "string" }
```

### 게시글

모든 게시글 API는 Authorization 헤더에 JWT 토큰이 필요합니다.

```
GET /api/posts
- 게시글 목록 조회
- Query: ?page=1&per_page=10
- Response: {
    "items": [
      {
        "id": "uuid",
        "title": "string",
        "content": "string",
        "author": {
          "id": "uuid",
          "username": "string"
        },
        "created_at": "datetime",
        "updated_at": "datetime"
      }
    ],
    "total": "number",
    "page": "number",
    "per_page": "number",
    "total_pages": "number"
  }

POST /api/posts
- 게시글 작성
- Request: { "title": "string", "content": "string" }
- Response: 게시글 객체

GET /api/posts/{id}
- 게시글 상세 조회
- Response: 게시글 객체

PUT /api/posts/{id}
- 게시글 수정 (작성자만 가능)
- Request: { "title": "string?", "content": "string?" }
- Response: 게시글 객체

DELETE /api/posts/{id}
- 게시글 삭제 (작성자만 가능)
- Response: 204 No Content
```

### 댓글

모든 댓글 API는 Authorization 헤더에 JWT 토큰이 필요합니다.

```
GET /api/posts/{post_id}/comments
- 댓글 목록 조회
- Query: ?page=1&per_page=10
- Response: {
    "items": [
      {
        "id": "uuid",
        "content": "string",
        "author": {
          "id": "uuid",
          "username": "string"
        },
        "parent_id": "uuid?",
        "created_at": "datetime",
        "updated_at": "datetime",
        "reply_count": "number"
      }
    ],
    "total": "number",
    "page": "number",
    "per_page": "number",
    "total_pages": "number"
  }

POST /api/posts/{post_id}/comments
- 댓글 작성
- Request: { "content": "string", "parent_id": "uuid?" }
- Response: 댓글 객체

PUT /api/comments/{id}
- 댓글 수정 (작성자만 가능)
- Request: { "content": "string" }
- Response: 댓글 객체

DELETE /api/comments/{id}
- 댓글 삭제 (작성자만 가능)
- Response: 204 No Content
```

## 코드 문서화

프로젝트의 모든 주요 컴포넌트에는 상세한 주석이 포함되어 있어 코드의 이해를 돕습니다:

- 파일 수준 주석: 각 파일의 목적과 주요 기능 설명
- 구조체 주석: 데이터 모델의 필드와 관계 설명
- 함수 주석: 매개변수, 반환값, 주요 로직 설명
- 비즈니스 로직 주석: 권한 검증, 데이터 처리 흐름 설명

## 라이선스

MIT License
