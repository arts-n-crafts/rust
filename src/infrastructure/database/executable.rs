pub trait Executable {
    fn execute(&self) -> bool;
}

#[cfg(test)]
mod database_executable {
    use super::*;
    use rstest::rstest;

    struct DatabaseQuery {}
    impl DatabaseQuery {
        pub fn new() -> Self {
            DatabaseQuery {}
        }
    }

    impl Executable for DatabaseQuery {
        fn execute(&self) -> bool {
            true
        }
    }

    #[rstest]
    fn should_be_executable() {
        let query = DatabaseQuery::new();
        assert_eq!(true, query.execute());
    }
}
