pub mod event_bus {
    pub mod event_producer;
    pub mod in_memory_event_producer;
}

pub mod event_store;

pub mod database {
    pub mod database_query;
    pub mod in_memory_database;
}

mod outbox;
