use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct NameInput {
  #[validate(length(min = 1, message = "Can not be empty"))]
  pub name: String,
}
