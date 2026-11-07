//! Сериализация сценариев «прочитать → изменить → записать» по ключу сущности.
//!
//! Репозитории сохраняют сущность целиком, поэтому две параллельные правки одной сущности
//! без блокировки теряли бы одна другую (и, например, одну заявку можно было одобрить дважды).
//! Замок держится до конца записи (коммита), сущности с разными ключами не ждут друг друга.

use std::sync::LazyLock;

use tokio::sync::{Mutex, MutexGuard};
use uuid::Uuid;

const STRIPES: usize = 4096;

static LOCKS: LazyLock<Box<[Mutex<()>]>> = LazyLock::new(|| (0..STRIPES).map(|_| Mutex::new(())).collect());

/// Замок сущности `id` (общий для всех сервисов процесса).
pub(crate) async fn lock(id: Uuid) -> MutexGuard<'static, ()> {
    let (high, low) = id.as_u64_pair();
    let stripe = (high ^ low.rotate_left(17)).wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 52;
    LOCKS[stripe as usize % STRIPES].lock().await
}
