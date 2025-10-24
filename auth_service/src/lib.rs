
mod database;

pub mod auth_utils;

use database::{Status, database_connected};
use auth_utils::{models::Credentials, login};

pub fn authenticate(creds: Credentials) -> bool {
    if let Status::Connected = database_connected() {
        auth_utils::login(creds);
        true
    } else {
        false
    }
}
