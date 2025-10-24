pub enum Status {
    Connected,
    Disconnected,
}

pub fn database_connected() -> Status {
    // Connect to db
    Status::Connected
}

pub fn get_user(username: &str) {
    // Fetch user from db
    println!("Fetching user: {}", username);
}
