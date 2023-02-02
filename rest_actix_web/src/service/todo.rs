use crate::model::todo::Todo;
use anyhow::Error;

pub trait TodoPort {
    fn find(&self) -> Result<Vec<Todo>, Error>;
    fn find_one(&self, id: String) -> Result<Option<Todo>, Error>;
    fn create(&self, todo: Todo) -> Result<(), Error>;
    fn update(&self, todo: Todo) -> Result<(), Error>;
    fn delete(&self, id: String) -> Result<(), Error>;
}

pub struct TodoService;

impl TodoService {
    pub fn new() -> Self {
        Self {}
    }
}

impl TodoPort for TodoService {
    fn find(&self) -> Result<Vec<Todo>, Error> {
        todo!()
    }

    fn find_one(&self, _id: String) -> Result<Option<Todo>, Error> {
        todo!()
    }

    fn create(&self, _todo: Todo) -> Result<(), Error> {
        todo!()
    }

    fn update(&self, _todo: Todo) -> Result<(), Error> {
        todo!()
    }

    fn delete(&self, _id: String) -> Result<(), Error> {
        todo!()
    }
}
