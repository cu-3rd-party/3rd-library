use library_domain::users::{UserId, UserRepository};

use crate::AppResult;

/// Роль модератора проверяется по БД, а не по токену: снятие роли действует сразу.
pub(crate) async fn is_moderator(users: &dyn UserRepository, user: UserId) -> AppResult<bool> {
    Ok(users.find_by_id(user).await?.is_some_and(|user| user.is_moderator()))
}
