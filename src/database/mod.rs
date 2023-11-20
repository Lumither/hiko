use surrealdb::engine::remote::ws::Client;

pub struct TaskDatabase {
    client: Client,
}
