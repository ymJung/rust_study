// auth_middleware.rs
// JWT 토큰을 검증하고 현재 인증된 사용자의 ID를 요청에 주입하는 미들웨어입니다.
// Actix-web의 미들웨어 시스템을 사용하여 구현되었습니다.

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};
use uuid::Uuid;

use crate::services::auth_service::AuthService;

// Auth 구조체는 미들웨어 팩토리입니다.
// 이 구조체는 새로운 미들웨어 인스턴스를 생성하는 역할을 합니다.
pub struct Auth;

// Transform 트레이트 구현
// 이는 미들웨어 팩토리의 동작을 정의합니다.
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    // S는 다음 미들웨어 또는 핸들러를 나타냅니다.
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // 새로운 미들웨어 인스턴스를 생성합니다.
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

// AuthMiddleware는 실제 미들웨어 구현체입니다.
pub struct AuthMiddleware<S> {
    service: S,
}

// Service 트레이트 구현
// 이는 실제 미들웨어의 동작을 정의합니다.
impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // forward_ready는 다음 서비스가 준비되었는지 확인합니다.
    forward_ready!(service);

    // 실제 미들웨어 로직이 구현된 부분입니다.
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Authorization 헤더에서 토큰을 추출합니다.
        let auth_header = req.headers().get("Authorization");
        // AuthService 인스턴스를 가져옵니다.
        let auth_service = req.app_data::<actix_web::web::Data<AuthService>>().cloned();

        // 토큰이나 AuthService가 없으면 인증 실패
        if auth_header.is_none() || auth_service.is_none() {
            return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized"))));
        }

        // "Bearer " 접두사를 확인하고 실제 토큰을 추출합니다.
        let auth_header = auth_header.unwrap().to_str().unwrap_or("");
        if !auth_header.starts_with("Bearer ") {
            return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized("Invalid token format"))));
        }

        let token = auth_header[7..].to_string();
        let auth_service = auth_service.unwrap();

        // 토큰을 검증하고 사용자 ID를 추출합니다.
        match auth_service.verify_token(&token) {
            Ok(claims) => {
                if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                    // 사용자 ID를 요청의 확장(extensions)에 저장합니다.
                    // 이를 통해 핸들러에서 현재 인증된 사용자의 ID를 조회할 수 있습니다.
                    req.extensions_mut().insert(user_id);
                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    })
                } else {
                    Box::pin(ready(Err(actix_web::error::ErrorUnauthorized("Invalid user ID"))))
                }
            }
            Err(_) => Box::pin(ready(Err(actix_web::error::ErrorUnauthorized("Invalid token")))),
        }
    }
}

// 현재 인증된 사용자의 ID를 가져오는 헬퍼 함수입니다.
// 핸들러에서 이 함수를 사용하여 현재 요청을 보낸 사용자의 ID를 조회할 수 있습니다.
pub fn get_current_user(req: &ServiceRequest) -> Option<Uuid> {
    req.extensions().get::<Uuid>().copied()
}
