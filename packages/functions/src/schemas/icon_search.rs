use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct DTO {
    // 图标 ID 列表
    pub iconIdList: Option<Vec<i64>>,
    // 创建者 ID
    pub creator: Option<String>,
    // 图标分类列表
    pub typeIdList: Option<Vec<i64>>,
    // 当前页，从 0 开始
    pub current: Option<i64>,
    // 每页大小，默认为 10
    pub size: Option<i64>,
}

impl DTO {
    #[allow(dead_code)]
    fn default() -> Self {
        Self {
            iconIdList: None,
            creator: None,
            typeIdList: None,
            current: Some(0),
            size: Some(10),
        }
    }
}
