use mysql::Pool;

pub struct Repository {
  db: Pool,
}

impl Repository {
  pub fn new(db: Pool) -> Self {
    Self { db }
  }
}
