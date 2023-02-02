pub mod todo {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    pub struct Todo {
        pub id: String,
        pub name: String,
        pub description: String,
        pub is_completed: bool,
    }

    impl Todo {
        pub fn new() -> Self {
            Self {
                id: String::from("default id"),
                name: String::from("default name"),
                description: String::from("default description"),
                is_completed: false,
            }
        }
    }

    #[cfg(test)]
    mod unit_test {
        use super::Todo;

        #[test]
        fn test_todo_new() {
            // arange
            let todo_new = Todo::new();

            // act
            let todo_expec = Todo {
                id: String::from("default id"),
                name: String::from("default name"),
                description: String::from("default description"),
                is_completed: false,
            };

            // assertion
            assert_eq!(todo_expec, todo_new);
        }
    }
}
