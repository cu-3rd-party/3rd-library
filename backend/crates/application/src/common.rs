use library_domain::common::Page;
pub use library_domain::common::PageRequest;

/// Страница результатов для ответа API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageDto<T> {
    pub items: Vec<T>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
}

impl<T> PageDto<T> {
    pub fn new<U>(page: Page<U>, request: PageRequest, f: impl FnMut(U) -> T) -> Self {
        Self {
            items: page.items.into_iter().map(f).collect(),
            page: request.page(),
            limit: request.limit(),
            total: page.total,
        }
    }
}
