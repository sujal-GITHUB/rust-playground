pub fn login(cred: models::Credentials) {
    crate::database::get_user(&cred.username);
}

pub mod models;
