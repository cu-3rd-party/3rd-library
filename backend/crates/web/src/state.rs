use std::sync::Arc;

use library_application::auth::AuthService;
use library_application::materials::MaterialService;
use library_application::moderation::ModerationService;
use library_application::ports::RateLimiter;
use library_application::submissions::SubmissionService;
use library_application::users::UserService;
use library_infrastructure::Infrastructure;

/// Composition root: связывает реализации из инфраструктуры с сервисами приложения.
/// Клонируется на каждый запрос, поэтому внутри только `Arc`/дешёвые хэндлы.
#[derive(Clone)]
pub struct AppState {
    pub auth: AuthService,
    pub users: UserService,
    pub materials: MaterialService,
    pub submissions: SubmissionService,
    pub moderation: ModerationService,
    pub rate_limiter: Option<Arc<dyn RateLimiter>>,
}

impl AppState {
    pub fn new(infrastructure: Infrastructure) -> Self {
        let infra = infrastructure;
        Self {
            auth: AuthService::new(
                infra.users.clone(),
                infra.password_hasher,
                infra.tokens,
                infra.sessions,
                infra.mailer,
            ),
            users: UserService::new(infra.users.clone(), infra.materials.clone()),
            materials: MaterialService::new(infra.materials),
            submissions: SubmissionService::new(infra.users.clone(), infra.submissions.clone(), infra.storage),
            moderation: ModerationService::new(infra.users, infra.submissions, infra.unit_of_work),
            rate_limiter: infra.rate_limiter,
        }
    }
}
