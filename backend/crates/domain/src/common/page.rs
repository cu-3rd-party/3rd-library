/// Запрошенная страница: номер с 1 и размер в пределах `1..=MAX_LIMIT` (лишнее обрезается, как раньше в API).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRequest {
    page: i64,
    limit: i64,
}

impl PageRequest {
    pub const DEFAULT_LIMIT: i64 = 20;
    pub const MAX_LIMIT: i64 = 100;

    pub fn new(page: Option<i64>, limit: Option<i64>) -> Self {
        Self {
            page: page.unwrap_or(1).max(1),
            limit: limit.unwrap_or(Self::DEFAULT_LIMIT).clamp(1, Self::MAX_LIMIT),
        }
    }

    pub fn page(&self) -> i64 {
        self.page
    }

    pub fn limit(&self) -> i64 {
        self.limit
    }

    pub fn offset(&self) -> i64 {
        (self.page - 1).saturating_mul(self.limit)
    }
}

impl Default for PageRequest {
    fn default() -> Self {
        Self::new(None, None)
    }
}

/// Страница результатов и общее число записей под тем же фильтром.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: i64,
}

impl<T> Page<T> {
    pub fn map<U>(self, f: impl FnMut(T) -> U) -> Page<U> {
        Page {
            items: self.items.into_iter().map(f).collect(),
            total: self.total,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_first_page() {
        let page = PageRequest::new(None, None);
        assert_eq!((page.page(), page.limit(), page.offset()), (1, 20, 0));
    }

    #[test]
    fn clamps_page_and_limit() {
        let page = PageRequest::new(Some(-3), Some(1000));
        assert_eq!((page.page(), page.limit()), (1, PageRequest::MAX_LIMIT));
        assert_eq!(PageRequest::new(Some(3), Some(0)).offset(), 2);
    }
}
