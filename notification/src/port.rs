use crate::error::{DBError, Error};
use crate::model::{FetchNotification, FilterNotification, Notification};
use async_trait::async_trait;

#[async_trait]
pub trait Database: Send + Sync {
  async fn fetch_notification(&self, arg: &FetchNotification)
    -> Result<Vec<Notification>, DBError>;

  async fn count_unread_notification(&self, user_id: &u64) -> Result<u16, DBError>;

  async fn find_notification(
    &self,
    user_id: &u64,
    id: &u64,
  ) -> Result<Option<Notification>, DBError>;

  async fn read_notification(&self, user_id: &u64, id: &u64) -> Result<bool, DBError>;
}

#[async_trait]
pub trait Usecase: Send + Sync {
  /// to get list notification, it can also flter by category and read unread
  ///
  async fn fetch_notification(
    &self,
    filter: &FilterNotification,
  ) -> Result<Vec<Notification>, Error>;

  /// to count list notification with status unread.
  /// if more than 99 then it will alway endup 99 in backend,
  /// but frondend will change to with 99+
  ///
  async fn count_unread_notification(&self, user_id: &u64) -> Result<u16, Error>;

  /// tp get one notification detail by id
  ///
  async fn find_notification(&self, user_id: &u64, id: &u64) -> Result<Notification, Error>;

  /// to mark specific notification read
  ///
  async fn read_notification(&self, user_id: &u64, id: &u64) -> Result<bool, Error>;
}
