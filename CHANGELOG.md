# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/arts-n-crafts/rust/releases/tag/v0.1.0) - 2025-09-19

### Added

- Repository
- WithIdentifier
- command handler
- Command
- *(DomainEvent)* implement EventPayload with strum_macros
- *(Decider)* it_should_decide_to_emit_an_user_liked_event_based_on_like_user_command
- *(Decider)* it_should_decide_to_emit_an_user_created_event_based_on_create_user_command
- *(Decider)* it_should_evolve_to_the_current_state_with_likes
- *(Decider)* it_should_evolve_to_the_current_created_state
- *(Decider)* it_should_return_the_initial_state
- *(EventStore)* EventStore trait
- *(Database)* move DatabaseError inside InMemoryDatabase
- *(EventStore)* InMemoryEventStore
- *(DomainEvent)* make aggregateId property public
- *(EventStore)* StoredEvent
- *(EventStore)* StreamKey
- *(Database)* implement async_trait
- *(Database)* database query trait
- *(Database)* in-memory impl
- *(Database)* database trait
- DomainEvent
- *(Database)* InMemory impl should_retrieve_the_data
- *(Database)* InMemoryDatabase impl should_store_data
- *(Database)* InMemoryDatabase should_have_data
- *(Database)* should_be_executable

### Other

- release-plz config
- prepare release crates
- integration_mongodb_should_create_a_new_user
- fmt
- add fmt-fix script
- show missing lines in coverage and cover 'em
- show missing lines in coverage
- README
- remove run-script from pre-commit
- add code coverage test
- add husky scripts
- prefix integration tests with integration
- format
- lint
- *(BaseEvent)* introduce BaseEvent and separate StoredEvent (again)
- *(cqrs)* structurize
- *(DomainEvent)* enum as event type
- removed Uuid constraints
- *(Decider)* lint
- *(EventStore)* reduce amount of iterations
- *(MongoEventStore)* make connection_string injectable
- *(EventStore)* in_memory_event_store should_query_all_the_data_in_the_stream
- *(EventStore)* mongodb_should_load_the_events_of_the_stream
- *(EventStore)* MongoDB event store implementation
- *(DomainEvent)* add serialization, lifetimes, timestamp
- *(deps)* add serde to uuid
- *(Database)* remove hasty Database abstraction
- *(Database)* database_query_test integration test
- *(Database)* separate database errors
- *(Database)* InMemory impl should_store_and_retrieve_multiple_data
- *(Database)* InMemory impl should_store_and_retrieve_the_data
- initial commmit
- Initial commit
