use crate::model::{
    error::UError,
    notification::{FilterNotification, Notification},
};
use async_trait::async_trait;

#[async_trait]
pub trait Usecase: Send + Sync {
    /// to get list notification, it can also flter by category and read unread
    ///
    async fn fetch_notification(
        &self,
        filter: &FilterNotification,
    ) -> Result<Vec<Notification>, UError>;

    /// to count list notification with status unread.
    /// if more than 99 then it will alway endup 99 in backend,
    /// but frondend will change to with 99+
    ///
    async fn count_unread_notification(&self, user_id: &u64) -> Result<u16, UError>;

    /// tp get one notification detail by id
    ///
    async fn find_notification(&self, user_id: &u64, id: &u64) -> Result<Notification, UError>;

    /// to mark specific notification read
    ///
    async fn read_notification(&self, user_id: &u64, id: &u64) -> Result<bool, UError>;
}
