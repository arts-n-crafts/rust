#[derive(Debug)]
pub enum DatabaseError {
    DatabaseStoreError(Box<dyn std::error::Error>),
    DatabaseQueryError(Box<dyn std::error::Error>),
}
