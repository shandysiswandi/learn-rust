use crate::model::{FetchNotification, Notification, UError};
use async_trait::async_trait;

#[async_trait]
pub trait Database: Send + Sync {
    async fn fetch_notification(
        &self,
        arg: &FetchNotification,
    ) -> Result<Vec<Notification>, UError>;

    async fn count_unread_notification(&self, user_id: &u64) -> Result<u16, UError>;

    async fn find_notification(
        &self,
        user_id: &u64,
        id: &u64,
    ) -> Result<Option<Notification>, UError>;

    async fn read_notification(&self, user_id: &u64, id: &u64) -> Result<bool, UError>;
}
