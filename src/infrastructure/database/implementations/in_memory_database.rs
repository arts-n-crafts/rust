struct InMemoryDatabase {
    data: bool
}

impl InMemoryDatabase {
    fn new() -> Self {
        InMemoryDatabase { data: true }
    }
}

#[cfg(test)]
mod database {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn should_have_data() {
        let db = InMemoryDatabase::new();
        assert_eq!(db.data, true);
    }
}
