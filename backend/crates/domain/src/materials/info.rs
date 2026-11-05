/// Описание материала: общее у заявки и опубликованного материала.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialInfo {
    pub title: String,
    pub description: String,
    pub courses: Vec<String>,
    pub subjects: Vec<String>,
    /// Тип материала (`type` в API и БД).
    pub kind: String,
    pub difficulty: String,
}
