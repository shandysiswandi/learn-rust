#[derive(Debug, Default)]
pub struct Notification {}

#[derive(Debug, Default)]
pub struct FilterNotification {
  pub user_id: u64,

  // info|promo| default is empty
  pub category: String,

  // true|false| default is empty
  pub is_read: String,
}

#[derive(Debug, Default)]
pub struct FetchNotification {
  pub user_id: u64,
  pub category: Option<String>,
  pub is_read: Option<bool>,
}
