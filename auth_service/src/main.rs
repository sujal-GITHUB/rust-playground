use auth_service::{authenticate, auth_utils::models::Credentials};

fn main() {
    let cred = Credentials {
        username: String::from("user1"),
        password: String::from("password123"),
    };

    authenticate(cred);
}
