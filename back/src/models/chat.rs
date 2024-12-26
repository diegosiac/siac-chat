use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct ChatModel {
    pub id: String,
    pub userId: String,
    pub messageId: String,
}
