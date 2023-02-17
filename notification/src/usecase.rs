use crate::{
  error::{Code, Error},
  model::{FetchNotification, FilterNotification, Notification},
  port::Database,
  port::Usecase,
};
use async_trait::async_trait;

pub struct UsecaseImpl {
  db: Box<dyn Database>,
}

impl UsecaseImpl {
  pub fn new(db: Box<dyn Database>) -> UsecaseImpl {
    UsecaseImpl { db }
  }
}

#[async_trait]
impl Usecase for UsecaseImpl {
  async fn fetch_notification(
    &self,
    filter: &FilterNotification,
  ) -> Result<Vec<Notification>, Error> {
    let mut arg = FetchNotification::default();

    if filter.category.eq("info") || filter.category.eq("promo") {
      arg.category = Some(filter.category.to_owned())
    }

    if filter.is_read.eq("true") {
      arg.is_read = Some(true)
    }

    if filter.is_read.eq("false") {
      arg.is_read = Some(false)
    }

    match self.db.fetch_notification(&arg).await {
      Ok(data) => Ok(data),
      Err(_) => Err(Error::new(Code::Internal, "failed to fetch notification")),
    }
  }

  async fn count_unread_notification(&self, user_id: &u64) -> Result<u16, Error> {
    match self.db.count_unread_notification(user_id).await {
      Ok(count) => Ok(count),
      Err(_) => Err(Error::new(
        Code::Internal,
        "failed to count unread notification",
      )),
    }
  }

  async fn find_notification(&self, user_id: &u64, id: &u64) -> Result<Notification, Error> {
    let notif = match self.db.find_notification(user_id, id).await {
      Ok(data) => data,
      Err(_) => return Err(Error::new(Code::Internal, "failed to find notification")),
    };

    match notif {
      Some(data) => Ok(data),
      None => Err(Error::new(Code::NotFound, "notification not found")),
    }
  }

  async fn read_notification(&self, user_id: &u64, id: &u64) -> Result<bool, Error> {
    match self.db.read_notification(user_id, id).await {
      Ok(is_success) => Ok(is_success),
      Err(_) => Err(Error::new(Code::Internal, "failed to read notification")),
    }
  }
}
