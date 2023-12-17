pub mod routes {
    pub async fn create() -> &'static str {
        "Created a user"
    }

    pub async fn read() -> &'static str {
        "Read all users"
    }

    pub async fn update() -> &'static str {
        "Updated a user"
    }

    pub async fn delete() -> &'static str {
        "Deleted a user"
    }
}
