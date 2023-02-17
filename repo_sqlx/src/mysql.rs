use async_trait::async_trait;
use notification::{
  error::DBError,
  model::{FetchNotification, Notification},
  port::Database,
};
use sqlx::{MySql, Pool, QueryBuilder};

use crate::{Counter, RowNotification};

pub struct Repository {
  db: Pool<MySql>,
}

impl Repository {
  pub fn new(db: Pool<MySql>) -> Self {
    Self { db }
  }
}

#[async_trait]
impl Database for Repository {
  async fn fetch_notification(
    &self,
    arg: &FetchNotification,
  ) -> Result<Vec<Notification>, DBError> {
    let q =
            "SELECT n.id, n.title, n.detail, c.name category, COALESCE(nr.is_read,0) is_read, n.created_at
        FROM notifications n 
        JOIN categories c ON (n.category_id = c.id)
        LEFT JOIN notification_read nr ON (n.id = nr.notification_id)
        WHERE (n.user_id = ? OR n.user_id IS NULL) AND (nr.user_id = ? OR nr.user_id IS NULL)";

    let mut q = QueryBuilder::new(q);
    q.push_bind(arg.user_id);
    q.push_bind(arg.user_id);

    if let Some(val) = &arg.category {
      q.push(" AND c.name = ?");
      q.push_bind(val.clone());
    }

    if let Some(val) = &arg.is_read {
      q.push(" AND is_read = ?");
      q.push_bind(*val);
    }

    let result = q.build().fetch_all(&self.db).await;

    match result {
      Ok(_list) => {
        let a = Vec::new();
        Ok(a)
      }
      Err(e) => Err(DBError::Internal(e.to_string())),
    }
  }

  async fn count_unread_notification(&self, user_id: &u64) -> Result<u16, DBError> {
    let q = "SELECT COUNT(n.id) count 
        FROM notifications n
        JOIN categories c ON (n.category_id = c.id)
        LEFT JOIN notification_read nr ON (nr.notification_id = n.id)
        WHERE (n.user_id = 1 OR n.user_id IS NULL) AND (nr.user_id IS NULL);";

    let result = sqlx::query_as::<_, Counter>(q)
      .bind(user_id)
      .fetch_one(&self.db)
      .await;

    match result {
      Ok(c) => Ok(c.count),
      Err(e) => Err(DBError::Internal(e.to_string())),
    }
  }

  async fn find_notification(
    &self,
    user_id: &u64,
    id: &u64,
  ) -> Result<Option<Notification>, DBError> {
    let q = "SELECT title, detail, c.name category, created_at
        FROM notifications n
        JOIN categories c ON (n.category_id = c.id)
        WHERE (n.user_id = ? OR user_id IS NULL) AND n.id = ?;";

    let result = sqlx::query_as::<_, RowNotification>(q)
      .bind(user_id)
      .bind(id)
      .fetch_optional(&self.db)
      .await;

    let opt = match result {
      Ok(opt) => opt,
      Err(e) => return Err(DBError::Internal(e.to_string())),
    };

    match opt {
      Some(_row) => Ok(Some(Notification::default())), // this must be map first
      None => Err(DBError::NotFound("notification not found".to_string())),
    }
  }

  async fn read_notification(&self, user_id: &u64, id: &u64) -> Result<bool, DBError> {
    let q = "INSERT INTO notification_read(is_read, user_id, notification_id) VALUES(true, ?, ?);";

    let result = sqlx::query(q)
      .bind(user_id)
      .bind(id)
      .execute(&self.db)
      .await;

    match result {
      Ok(result) => Ok(result.rows_affected() > 0),
      Err(e) => Err(DBError::Internal(e.to_string())),
    }
  }
}
