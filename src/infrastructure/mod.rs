mod event_store;
mod event_bus;
mod database {
    mod executable;
    mod implementations {
        mod in_memory_database;
    }
}

mod outbox;
mod command_bus;
mod query_bus;
