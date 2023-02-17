use async_trait::async_trait;
use notification::{
  error::DBError,
  model::{FetchNotification, Notification},
  port::Database,
};
use sqlx::{Pool, Postgres};

pub struct Repository {
  db: Pool<Postgres>,
}

impl Repository {
  pub fn new(db: Pool<Postgres>) -> Self {
    Self { db }
  }
}

#[async_trait]
impl Database for Repository {
  async fn fetch_notification(
    &self,
    _arg: &FetchNotification,
  ) -> Result<Vec<Notification>, DBError> {
    dbg!(&self.db);
    todo!()
  }

  async fn count_unread_notification(&self, _user_id: &u64) -> Result<u16, DBError> {
    todo!()
  }

  async fn find_notification(
    &self,
    _user_id: &u64,
    _id: &u64,
  ) -> Result<Option<Notification>, DBError> {
    todo!()
  }

  async fn read_notification(&self, _user_id: &u64, _id: &u64) -> Result<bool, DBError> {
    todo!()
  }
}
