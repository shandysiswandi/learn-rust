use crate::{
    model::{
        error::UError,
        notification::{FetchNotification, FilterNotification, Notification},
    },
    port::database::Database,
    port::usecase::Usecase,
};
use async_trait::async_trait;

pub struct UsecaseImpl {
    db: Box<dyn Database>,
}

impl UsecaseImpl {
    pub fn new(db: Box<dyn Database>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl Usecase for UsecaseImpl {
    async fn fetch_notification(
        &self,
        filter: &FilterNotification,
    ) -> Result<Vec<Notification>, UError> {
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

        let list = match self.db.fetch_notification(&arg).await {
            Ok(data) => data,
            Err(e) => return Err(UError::new("cannot get list notification", e.code)),
        };

        Ok(list)
    }

    async fn count_unread_notification(&self, user_id: &u64) -> Result<u16, UError> {
        let count = match self.db.count_unread_notification(user_id).await {
            Ok(data) => data,
            Err(e) => return Err(UError::new("cannot count notification", e.code)),
        };

        Ok(count)
    }

    async fn find_notification(&self, user_id: &u64, id: &u64) -> Result<Notification, UError> {
        let notif = match self.db.find_notification(user_id, id).await {
            Ok(data) => data,
            Err(e) => return Err(UError::new("cannot find notification", e.code)),
        };

        match notif {
            Some(data) => Ok(data),
            None => return Err(UError::new("notification not found", 404)),
        }
    }

    async fn read_notification(&self, user_id: &u64, id: &u64) -> Result<bool, UError> {
        let is_success = match self.db.read_notification(user_id, id).await {
            Ok(data) => data,
            Err(e) => return Err(UError::new("cannot read notification", e.code)),
        };

        Ok(is_success)
    }
}
