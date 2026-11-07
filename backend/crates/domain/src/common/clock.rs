use chrono::{DateTime, DurationRound, TimeDelta, Utc};

/// Текущее время с точностью до микросекунды — как хранит `timestamptz`.
/// Состояние в памяти бэкенда должно совпадать с БД побайтно, поэтому все метки берутся отсюда.
pub fn now() -> DateTime<Utc> {
    let now = Utc::now();
    now.duration_trunc(TimeDelta::microseconds(1)).unwrap_or(now)
}
